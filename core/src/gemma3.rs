use candle_core::{DType, Device, Module, Result, Tensor, D};
use candle_nn::{Linear, VarBuilder, Activation};
use candle_transformers::utils;
use std::sync::Arc;

fn linear(d1: usize, d2: usize, bias: bool, vb: VarBuilder) -> Result<Linear> {
    if bias {
        candle_nn::linear(d1, d2, vb)
    } else {
        candle_nn::linear_no_bias(d1, d2, vb)
    }
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct Config {
    pub attention_bias: bool,
    pub head_dim: usize,
    pub hidden_act: Option<Activation>,
    pub hidden_activation: Option<Activation>,
    pub hidden_size: usize,
    pub intermediate_size: usize,
    pub num_attention_heads: usize,
    pub num_hidden_layers: usize,
    pub num_key_value_heads: usize,
    pub rms_norm_eps: f64,
    pub rope_theta: f64,
    pub vocab_size: usize,
    pub max_position_embeddings: usize,
    pub layer_types: Option<Vec<String>>, 
    pub sliding_window: Option<usize>,
}

impl Config {
    fn hidden_act(&self) -> Result<Activation> {
        match (self.hidden_act, self.hidden_activation) {
            (None, Some(act)) | (Some(act), None) => Ok(act),
            (Some(_), Some(_)) => candle_core::bail!("both hidden_act and hidden_activation are set"),
            (None, None) => candle_core::bail!("none of hidden_act and hidden_activation are set"),
        }
    }
}

#[derive(Debug, Clone)]
struct RmsNorm {
    weight: Tensor,
    eps: f64,
}

impl RmsNorm {
    fn new(dim: usize, eps: f64, vb: VarBuilder) -> Result<Self> {
        let weight = vb.get(dim, "weight")?;
        Ok(Self { weight, eps })
    }
}

impl Module for RmsNorm {
    fn forward(&self, x: &Tensor) -> Result<Tensor> {
        let x_dtype = x.dtype();
        let internal_dtype = match x_dtype {
            DType::F16 | DType::BF16 => DType::F32,
            d => d,
        };
        let hidden_size = x.dim(D::Minus1)?;
        let x = x.to_dtype(internal_dtype)?;
        let norm_x = (x.sqr()?.sum_keepdim(D::Minus1)? / hidden_size as f64)?;
        let x_normed = x.broadcast_div(&(norm_x + self.eps)?.sqrt()?)?;
        x_normed
            .to_dtype(x_dtype)?
            .broadcast_mul(&(&self.weight + 1.0)?)
    }
}

#[derive(Debug, Clone)]
struct RotaryEmbedding {
    sin_local: Tensor,
    cos_local: Tensor,
    sin_global: Tensor,
    cos_global: Tensor,
}

impl RotaryEmbedding {
    fn new(dtype: DType, cfg: &Config, dev: &Device) -> Result<Self> {
        let dim = cfg.head_dim;
        let max_seq_len = cfg.max_position_embeddings;
        
        let local_theta = 10_000.0;
        let global_theta = 1_000_000.0;

        let (sin_local, cos_local) = Self::compute_freqs(dim, max_seq_len, local_theta, dtype, dev)?;
        let (sin_global, cos_global) = Self::compute_freqs(dim, max_seq_len, global_theta, dtype, dev)?;
        
        Ok(Self {
            sin_local,
            cos_local,
            sin_global,
            cos_global,
        })
    }

    fn compute_freqs(dim: usize, max_seq_len: usize, theta: f64, dtype: DType, dev: &Device) -> Result<(Tensor, Tensor)> {
        let inv_freq: Vec<_> = (0..dim)
            .step_by(2)
            .map(|i| 1f32 / theta.powf(i as f64 / dim as f64) as f32)
            .collect();
        let inv_freq_len = inv_freq.len();
        let inv_freq = Tensor::from_vec(inv_freq, (1, inv_freq_len), dev)?.to_dtype(dtype)?;
        let t = Tensor::arange(0u32, max_seq_len as u32, dev)?
            .to_dtype(dtype)?
            .reshape((max_seq_len, 1))?;
        let freqs = t.matmul(&inv_freq)?;
        Ok((freqs.sin()?, freqs.cos()?))
    }

    fn apply_rotary_emb_qkv(
        &self,
        q: &Tensor,
        k: &Tensor,
        seqlen_offset: usize,
        is_global: bool,
    ) -> Result<(Tensor, Tensor)> {
        let (_b_sz, _h, seq_len, _n_embd) = q.dims4()?;
        
        let (sin, cos) = if is_global {
            (&self.sin_global, &self.cos_global)
        } else {
            (&self.sin_local, &self.cos_local)
        };

        let cos = cos.narrow(0, seqlen_offset, seq_len)?;
        let sin = sin.narrow(0, seqlen_offset, seq_len)?;
        let q_embed = candle_nn::rotary_emb::rope(&q.contiguous()?, &cos, &sin)?;
        let k_embed = candle_nn::rotary_emb::rope(&k.contiguous()?, &cos, &sin)?;
        Ok((q_embed, k_embed))
    }
}

#[allow(clippy::upper_case_acronyms)]
struct MLP {
    gate_proj: Linear,
    up_proj: Linear,
    down_proj: Linear,
    act_fn: Activation,
}

impl MLP {
    fn new(cfg: &Config, vb: VarBuilder) -> Result<Self> {
        let hidden_sz = cfg.hidden_size;
        let intermediate_sz = cfg.intermediate_size;
        let gate_proj = linear(hidden_sz, intermediate_sz, false, vb.pp("gate_proj"))?;
        let up_proj = linear(hidden_sz, intermediate_sz, false, vb.pp("up_proj"))?;
        let down_proj = linear(intermediate_sz, hidden_sz, false, vb.pp("down_proj"))?;
        Ok(Self {
            gate_proj,
            up_proj,
            down_proj,
            act_fn: cfg.hidden_act()?,
        })
    }
}

impl Module for MLP {
    fn forward(&self, xs: &Tensor) -> Result<Tensor> {
        let lhs = xs.apply(&self.gate_proj)?.apply(&self.act_fn)?;
        let rhs = xs.apply(&self.up_proj)?;
        (lhs * rhs)?.apply(&self.down_proj)
    }
}

#[derive(Debug, Clone)]
struct Attention {
    q_proj: Linear,
    k_proj: Linear,
    v_proj: Linear,
    o_proj: Linear,
    num_heads: usize,
    num_kv_heads: usize,
    num_kv_groups: usize,
    head_dim: usize,
    rotary_emb: Arc<RotaryEmbedding>,
    kv_cache: Option<(Tensor, Tensor)>,
    use_flash_attn: bool,
    is_global: bool,
    sliding_window: Option<usize>,
}

impl Attention {
    fn new(
        rotary_emb: Arc<RotaryEmbedding>,
        use_flash_attn: bool,
        cfg: &Config,
        vb: VarBuilder,
        layer_type: &str,
    ) -> Result<Self> {
        let hidden_sz = cfg.hidden_size;
        let num_heads = cfg.num_attention_heads;
        let num_kv_heads = cfg.num_key_value_heads;
        let num_kv_groups = num_heads / num_kv_heads;
        let head_dim = cfg.head_dim;
        let bias = cfg.attention_bias;
        let q_proj = linear(hidden_sz, num_heads * head_dim, bias, vb.pp("q_proj"))?;
        let k_proj = linear(hidden_sz, num_kv_heads * head_dim, bias, vb.pp("k_proj"))?;
        let v_proj = linear(hidden_sz, num_kv_heads * head_dim, bias, vb.pp("v_proj"))?;
        let o_proj = linear(num_heads * head_dim, hidden_sz, bias, vb.pp("o_proj"))?;
        
        let is_global = layer_type == "full_attention";
        let sliding_window = if !is_global { cfg.sliding_window } else { None };

        Ok(Self {
            q_proj,
            k_proj,
            v_proj,
            o_proj,
            num_heads,
            num_kv_heads,
            num_kv_groups,
            head_dim,
            rotary_emb,
            kv_cache: None,
            use_flash_attn,
            is_global,
            sliding_window,
        })
    }

    fn forward(
        &mut self,
        xs: &Tensor,
        attention_mask: Option<&Tensor>,
        seqlen_offset: usize,
    ) -> Result<Tensor> {
        let (b_sz, q_len, _) = xs.dims3()?;

        let query_states = self.q_proj.forward(xs)?;
        let key_states = self.k_proj.forward(xs)?;
        let value_states = self.v_proj.forward(xs)?;

        let query_states = query_states
            .reshape((b_sz, q_len, self.num_heads, self.head_dim))?
            .transpose(1, 2)?;
        let key_states = key_states
            .reshape((b_sz, q_len, self.num_kv_heads, self.head_dim))?
            .transpose(1, 2)?;
        let value_states = value_states
            .reshape((b_sz, q_len, self.num_kv_heads, self.head_dim))?
            .transpose(1, 2)?;

        // Apply Global vs Local RoPE
        let (query_states, key_states) =
            self.rotary_emb
                .apply_rotary_emb_qkv(&query_states, &key_states, seqlen_offset, self.is_global)?;

        let (key_states, value_states) = match &self.kv_cache {
            None => (key_states, value_states),
            Some((prev_k, prev_v)) => {
                let key_states = Tensor::cat(&[prev_k, &key_states], 2)?;
                let value_states = Tensor::cat(&[prev_v, &value_states], 2)?;
                (key_states, value_states)
            }
        };
        self.kv_cache = Some((key_states.clone(), value_states.clone()));

        let key_states = utils::repeat_kv(key_states, self.num_kv_groups)?.contiguous()?;
        let value_states =
            utils::repeat_kv(value_states, self.num_kv_groups)?.contiguous()?;

        let attn_output = if self.use_flash_attn {
             // Flash Attn sliding window support is complex, fallback to manual for safety if sliding
             // For now assume standard flash attn (broken for sliding) or disable it
             // Let's rely on standard logic but with mask
             candle_core::bail!("Flash Attention not supported properly for Gemma3 yet")
        } else {
            let scale = 1f64 / f64::sqrt(self.head_dim as f64);
            let mut attn_weights = (query_states.matmul(&key_states.transpose(2, 3)?)? * scale)?;

            // --- INSERT START AT LINE 274 ---
            let softcap = 50.0f64; 
            attn_weights = (attn_weights.to_dtype(DType::F32)? / softcap)?;
            attn_weights = attn_weights.tanh()?;
            attn_weights = (attn_weights * softcap)?.to_dtype(xs.dtype())?;
            // --- INSERT END ---
            
            // Apply standard CAUSAL mask
            let attn_weights = match attention_mask {
                None => attn_weights,
                Some(mask) => attn_weights.broadcast_add(mask)?,
            };
            
            // Apply SLIDING WINDOW mask if needed
            let attn_weights = if let Some(window) = self.sliding_window {
                 let (b, h, q_l, k_l) = attn_weights.dims4()?;
                 if q_l == 1 {
                     let current_pos = seqlen_offset;
                     let min_allowed = current_pos.saturating_sub(window);
                     
                     if min_allowed > 0 {
                        let dev = attn_weights.device();
                        let dtype = attn_weights.dtype();
                        
                        let indices = Tensor::arange(0u32, k_l as u32, dev)?.to_dtype(DType::F32)?;
                        let min_allowed_t = Tensor::new(min_allowed as f32, dev)?;
                        
                        // Mask is 1.0 where (index < min_allowed), 0.0 otherwise
                        let mask = indices.broadcast_lt(&min_allowed_t)?.to_dtype(dtype)?;
                        
                        // Create penalty (-inf * mask)
                        // Using a large negative number instead of true INF to avoid NaN in some backends
                        let penalty_val = -1e9f32; 
                        let penalty_t = Tensor::new(penalty_val, dev)?.to_dtype(dtype)?;
                        let penalty = mask.broadcast_mul(&penalty_t)?;
                        
                        attn_weights.broadcast_add(&penalty)?
                     } else {
                         attn_weights
                     }
                 } else {
                     // For prefill (q_l > 1), strictly we should mask, but assuming prompt < 512 for now.
                     // TODO: Implement full 2D sliding mask for long prompts.
                     attn_weights
                 }
            } else {
                attn_weights
            };

            let attn_weights = candle_nn::ops::softmax_last_dim(&attn_weights)?;
            attn_weights.matmul(&value_states)?
        };
        attn_output
            .transpose(1, 2)?
            .reshape((b_sz, q_len, ()))?
            .apply(&self.o_proj)
    }

    fn clear_kv_cache(&mut self) {
        self.kv_cache = None
    }
}

struct DecoderLayer {
    self_attn: Attention,
    mlp: MLP,
    input_layernorm: RmsNorm,
    post_attention_layernorm: RmsNorm,
}

impl DecoderLayer {
    fn new(
        rotary_emb: Arc<RotaryEmbedding>,
        use_flash_attn: bool,
        cfg: &Config,
        vb: VarBuilder,
        layer_idx: usize,
    ) -> Result<Self> {
        // Determine layer type
        let layer_type = if let Some(types) = &cfg.layer_types {
            types.get(layer_idx).unwrap_or(&"sliding_attention".to_string()).clone()
        } else {
            "sliding_attention".to_string() // Fallback
        };

        let self_attn = Attention::new(rotary_emb, use_flash_attn, cfg, vb.pp("self_attn"), &layer_type)?;
        let mlp = MLP::new(cfg, vb.pp("mlp"))?;
        let input_layernorm =
            RmsNorm::new(cfg.hidden_size, cfg.rms_norm_eps, vb.pp("input_layernorm"))?;
        let post_attention_layernorm = RmsNorm::new(
            cfg.hidden_size,
            cfg.rms_norm_eps,
            vb.pp("post_attention_layernorm"),
        )?;
        Ok(Self {
            self_attn,
            mlp,
            input_layernorm,
            post_attention_layernorm,
        })
    }

    fn forward(
        &mut self,
        xs: &Tensor,
        attention_mask: Option<&Tensor>,
        seqlen_offset: usize,
    ) -> Result<Tensor> {
        let residual = xs;
        let xs = self.input_layernorm.forward(xs)?;
        let xs = self.self_attn.forward(&xs, attention_mask, seqlen_offset)?;
        let xs = (xs + residual)?;
        let residual = &xs;
        let xs = xs.apply(&self.post_attention_layernorm)?.apply(&self.mlp)?;
        residual + xs
    }

    fn clear_kv_cache(&mut self) {
        self.self_attn.clear_kv_cache()
    }
}

pub struct Model {
    embed_tokens: candle_nn::Embedding,
    layers: Vec<DecoderLayer>,
    norm: RmsNorm,
    lm_head: Linear,
    device: Device,
    dtype: DType,
    hidden_size: usize,
}

impl Model {
    pub fn new(use_flash_attn: bool, cfg: &Config, vb: VarBuilder) -> Result<Self> {
        let vb_m = vb.pp("model");
        let embed_tokens =
            candle_nn::embedding(cfg.vocab_size, cfg.hidden_size, vb_m.pp("embed_tokens"))?;
        let rotary_emb = Arc::new(RotaryEmbedding::new(vb.dtype(), cfg, vb_m.device())?);
        let mut layers = Vec::with_capacity(cfg.num_hidden_layers);
        let vb_l = vb_m.pp("layers");
        for layer_idx in 0..cfg.num_hidden_layers {
            let layer =
                DecoderLayer::new(rotary_emb.clone(), use_flash_attn, cfg, vb_l.pp(layer_idx), layer_idx)?;
            layers.push(layer)
        }
        let norm = RmsNorm::new(cfg.hidden_size, cfg.rms_norm_eps, vb_m.pp("norm"))?;
        let lm_head = Linear::new(embed_tokens.embeddings().clone(), None);
        Ok(Self {
            embed_tokens,
            layers,
            norm,
            lm_head,
            device: vb.device().clone(),
            dtype: vb.dtype(),
            hidden_size: cfg.hidden_size,
        })
    }

    pub fn prepare_decoder_attention_mask(
        &self,
        b_size: usize,
        tgt_len: usize,
        seqlen_offset: usize,
    ) -> Result<Tensor> {
        let mask: Vec<_> = (0..tgt_len)
            .flat_map(|i| (0..tgt_len).map(move |j| if i < j { f32::NEG_INFINITY } else { 0. }))
            .collect();
        let mask = Tensor::from_slice(&mask, (tgt_len, tgt_len), &self.device)?;
        let mask = if seqlen_offset > 0 {
            let mask0 = Tensor::zeros((tgt_len, seqlen_offset), DType::F32, &self.device)?;
            Tensor::cat(&[&mask0, &mask], D::Minus1)?
        } else {
            mask
        };
        mask.expand((b_size, 1, tgt_len, tgt_len + seqlen_offset))?
            .to_dtype(self.dtype)
    }

    pub fn forward(&mut self, input_ids: &Tensor, seqlen_offset: usize) -> Result<Tensor> {
        let (b_size, seq_len) = input_ids.dims2()?;
        let attention_mask = if seq_len <= 1 {
            None
        } else {
            let mask = self.prepare_decoder_attention_mask(b_size, seq_len, seqlen_offset)?;
            Some(mask)
        };
        let xs = self.embed_tokens.forward(input_ids)?;
        let mut xs = (xs * (self.hidden_size as f64).sqrt())?;
        for layer in self.layers.iter_mut() {
            xs = layer.forward(&xs, attention_mask.as_ref(), seqlen_offset)?
        }
        xs.narrow(1, seq_len - 1, 1)?
            .apply(&self.norm)?
            .apply(&self.lm_head)
    }
}
