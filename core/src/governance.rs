use std::fs;
use std::path::PathBuf;
use std::io;
use std::time::{SystemTime, UNIX_EPOCH};
use chrono::Local;
use toml; // Added toml crate

use crate::keyring::AeonKeyring;
use crate::Mandate; // Added Mandate struct

pub fn approve_candidate(name: &str) -> io::Result<()> {
    let base_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).parent().unwrap().to_path_buf();
    let candidates_dir = base_path.join("candidates");
    let mandates_dir = base_path.join("mandates");

    let candidate_path = candidates_dir.join(format!("{}.toml", name));
    let mandate_dest = mandates_dir.join(format!("{}.toml", name));

    if !candidate_path.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("Candidate '{}' not found in {:?}", name, candidates_dir),
        ));
    }

    // 1. Load or initialize keyring
    let keyring = AeonKeyring::init().map_err(|e| {
        io::Error::new(io::ErrorKind::Other, format!("Failed to load keyring: {}", e))
    })?;

    // 2. Read and parse candidate TOML
    let content = fs::read_to_string(&candidate_path)?;
    let mut mandate: Mandate = toml::from_str(&content).map_err(|e| {
        io::Error::new(io::ErrorKind::InvalidData, format!("Failed to parse TOML: {}", e))
    })?;
    
    // 3. REGENERATE DID with public key (Sprint 2 - ADR-015 Amendment)
    let pubkey_hex = keyring.public_key_hex();
    let old_did = mandate.did.clone();
    let new_did = format!("did:aeon:{}:{}:{}", mandate.agent_id, mandate.version, pubkey_hex);
    
    if old_did != new_did {
        println!("📝 [GOVERNANCE] Updating DID:");
        println!("   Old: {}", old_did);
        println!("   New: {}", new_did);
    }
    
    mandate.did = new_did;
    
    // 4. Serialize updated mandate to TOML
    let updated_toml = toml::to_string(&mandate).map_err(|e| {
        io::Error::new(io::ErrorKind::InvalidData, format!("Failed to serialize TOML: {}", e))
    })?;
    let content_trimmed = updated_toml.trim();

    // 5. Sign the UPDATED (with new DID) content with ed25519
    let signature = keyring.sign(content_trimmed.as_bytes());
    let signature_hex = hex::encode(signature.to_bytes());
    
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    let datetime = Local::now().to_rfc3339();

    // 6. Append Cryptographic Signature Block
    let signed_content = format!(
        "{}\n\n# ==========================================\n# 🔐 GOVERNANCE ORACLE SIGNATURE\n# Signed-By: Sovereign (AEON Keyring)\n# Public-Key: {}\n# Timestamp: {} ({})\n# Signature: {}\n# Algorithm: ed25519\n# ==========================================\n",
        content_trimmed,
        pubkey_hex,
        timestamp,
        datetime,
        signature_hex
    );

    // 7. Move to Mandates (Write new, Delete old)
    fs::write(&mandate_dest, signed_content)?;
    fs::remove_file(&candidate_path)?;

    println!("✅ [GOVERNANCE] Signed and Promoted:");
    println!("   From: {}", candidate_path.display());
    println!("   To:   {}", mandate_dest.display());
    println!("   Signature: {}...", &signature_hex[..16]);
    println!("   Public Key: {}...", &pubkey_hex[..16]);
    
    Ok(())
}

/// Verify the ed25519 signature of a mandate file
pub fn verify_mandate_signature(mandate_path: &std::path::Path) -> Result<bool, Box<dyn std::error::Error>> {
    use ed25519_dalek::{Signature, VerifyingKey, Verifier};


    // [SECURITY] 1. Check file size (DoS Prevention)
    let metadata = fs::metadata(mandate_path)?;
    if metadata.len() > 5 * 1024 * 1024 { // 5MB limit
        return Err("Mandate file too large (>5MB)".into());
    }

    // [SECURITY] 2. Read file (Safe now due to size limit)
    let content = fs::read_to_string(mandate_path)?;
    
    // [SECURITY] 3. Robust Parsing (State Machine-ish)
    // We expect a split between Content and Signature Block
    let delimiter = "\n\n# ==========================================\n# 🔐 GOVERNANCE ORACLE SIGNATURE";
    let parts: Vec<&str> = content.split(delimiter).collect();
    
    if parts.len() != 2 {
        return Err("Invalid mandate format: Missing signature block delimiter".into());
    }
    
    let original_content = parts[0].trim();
    let signature_block = parts[1];

    // Extract signature and public key from the block
    let sig_marker = "# Signature: ";
    let pubkey_marker = "# Public-Key: ";
    
    let signature_hex = signature_block.lines()
        .find(|line| line.starts_with(sig_marker))
        .and_then(|line| line.strip_prefix(sig_marker))
        .ok_or("Signature not found in block")?;
    
    let pubkey_hex = signature_block.lines()
        .find(|line| line.starts_with(pubkey_marker))
        .and_then(|line| line.strip_prefix(pubkey_marker))
        .ok_or("Public key not found in block")?;
    
    // Parse signature and public key
    let sig_bytes = hex::decode(signature_hex.trim())?;
    let pubkey_bytes = hex::decode(pubkey_hex.trim())?;
    
    if sig_bytes.len() != 64 { return Err("Invalid signature length".into()); }
    if pubkey_bytes.len() != 32 { return Err("Invalid public key length".into()); }
    
    let signature = Signature::from_bytes(&sig_bytes.as_slice().try_into().unwrap());
    let verifying_key = VerifyingKey::from_bytes(&pubkey_bytes.as_slice().try_into().unwrap())?;
    
    // Verify
    match verifying_key.verify(original_content.as_bytes(), &signature) {
        Ok(_) => Ok(true),
        Err(_) => Ok(false),
    }
}
