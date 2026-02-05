
import torch
import json
from transformers import AutoTokenizer, AutoModelForCausalLM
from peft import PeftModel

# Configuration
BASE_MODEL_ID = "google/gemma-3-270m-it"
ADAPTER_PATH = "./adapter"  # Path to your local adapter folder

class GovernanceSLM:
    def __init__(self):
        print(f"Loading Base Model: {BASE_MODEL_ID}...")
        try:
            # Load the base model (downloads from HuggingFace if not cached)
            self.base_model = AutoModelForCausalLM.from_pretrained(
                BASE_MODEL_ID, 
                torch_dtype=torch.float32,  # Use float32 on Mac M-series for stability
                device_map="auto",
                trust_remote_code=True
            )
            self.tokenizer = AutoTokenizer.from_pretrained(BASE_MODEL_ID)
            
            # Load the Adapter (The "Brain")
            print(f"Loading Adapter from: {ADAPTER_PATH}...")
            self.model = PeftModel.from_pretrained(self.base_model, ADAPTER_PATH)
            self.model.eval()
            print("Model loaded successfully.")
            
        except Exception as e:
            print(f"Error loading model: {e}")
            raise e

    def predict(self, prompt: str):
        # Format prompt exactly as used during training
        instruction = "Assess compliance risk for the following scenario."
        formatted_prompt = (
            f"<start_of_turn>user\n"
            f"{instruction}\n\n"
            f"Input:\n{prompt}<end_of_turn>\n"
            f"<start_of_turn>model\n"
        )
        
        inputs = self.tokenizer(formatted_prompt, return_tensors="pt").to(self.model.device)
        
        with torch.no_grad():
            outputs = self.model.generate(
                **inputs, 
                max_new_tokens=256, 
                do_sample=True, 
                temperature=0.1  # Low temp for deterministic compliance check
            )
        
        generated_text = self.tokenizer.decode(outputs[0], skip_special_tokens=True)
        
        # Clean up identifying tokens to get just the JSON response
        response_text = generated_text.replace(formatted_prompt.replace("<start_of_turn>user\n", "").replace("<end_of_turn>\n<start_of_turn>model\n", ""), "").strip()
        
        # Try to parse JSON output
        try:
            if "{" in response_text and "}" in response_text:
                json_str = response_text[response_text.find("{"):response_text.rfind("}")+1]
                structured_output = json.loads(json_str)
                return structured_output
        except:
            pass
            
        return {"raw_output": response_text}

if __name__ == "__main__":
    # Example Usage
    slm = GovernanceSLM()
    
    test_scenario = "The production database password is stored in cleartext in the git repository."
    print(f"\nAnalyzing Scenario: {test_scenario}\n")
    
    result = slm.predict(test_scenario)
    print(json.dumps(result, indent=2))
