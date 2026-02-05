use ed25519_dalek::{Signer, SigningKey, Verifier, VerifyingKey, Signature};
use rand::rngs::OsRng;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

const KEYRING_DIR: &str = ".aeon/keyring";
const PRIVATE_KEY_FILE: &str = "sovereign.key";
const PUBLIC_KEY_FILE: &str = "sovereign.pub";

pub struct AeonKeyring {
    signing_key: SigningKey,
    verifying_key: VerifyingKey,
    keyring_path: PathBuf,
}

impl AeonKeyring {
    /// Initialize AEON keyring: generates new keypair if not exists
    pub fn init() -> Result<Self, Box<dyn std::error::Error>> {
        let keyring_path = PathBuf::from(KEYRING_DIR);
        
        // Create keyring directory if it doesn't exist
        if !keyring_path.exists() {
            fs::create_dir_all(&keyring_path)?;
            println!("📁 Created keyring directory: {}", keyring_path.display());
        }

        let private_key_path = keyring_path.join(PRIVATE_KEY_FILE);
        let public_key_path = keyring_path.join(PUBLIC_KEY_FILE);

        let (signing_key, verifying_key) = if private_key_path.exists() {
            // Load existing keypair
            println!("🔑 Loading existing keypair...");
            Self::load_keypair(&private_key_path)?
        } else {
            // Generate new keypair
            println!("🔐 Generating new ed25519 keypair...");
            let signing_key = SigningKey::generate(&mut OsRng);
            let verifying_key = signing_key.verifying_key();
            
            // Save private key
            Self::save_private_key(&private_key_path, &signing_key)?;
            
            // Save public key (for convenience)
            Self::save_public_key(&public_key_path, &verifying_key)?;
            
            println!("✅ Keypair generated and saved to {}", keyring_path.display());
            println!("   Public Key: {}", hex::encode(verifying_key.to_bytes()));
            
            (signing_key, verifying_key)
        };

        Ok(Self {
            signing_key,
            verifying_key,
            keyring_path,
        })
    }

    /// Load keypair from file
    fn load_keypair(private_key_path: &Path) -> Result<(SigningKey, VerifyingKey), Box<dyn std::error::Error>> {
        let mut file = File::open(private_key_path)?;
        let mut key_bytes = [0u8; 32];
        file.read_exact(&mut key_bytes)?;

        let signing_key = SigningKey::from_bytes(&key_bytes);
        let verifying_key = signing_key.verifying_key();
        
        Ok((signing_key, verifying_key))
    }

    /// Save private key with secure permissions
    fn save_private_key(path: &Path, signing_key: &SigningKey) -> Result<(), Box<dyn std::error::Error>> {
        let mut file = File::create(path)?;
        
        // Write secret key bytes (32 bytes)
        file.write_all(&signing_key.to_bytes())?;
        
        // Set restrictive permissions (Unix only)
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = file.metadata()?.permissions();
            perms.set_mode(0o600); // Read/write for owner only
            fs::set_permissions(path, perms)?;
        }
        
        Ok(())
    }

    /// Save public key for reference
    fn save_public_key(path: &Path, verifying_key: &VerifyingKey) -> Result<(), Box<dyn std::error::Error>> {
        let mut file = File::create(path)?;
        file.write_all(&verifying_key.to_bytes())?;
        Ok(())
    }

    /// Sign a message
    pub fn sign(&self, message: &[u8]) -> Signature {
        self.signing_key.sign(message)
    }

    /// Verify a signature
    pub fn verify(&self, message: &[u8], signature: &Signature) -> bool {
        self.verifying_key.verify(message, signature).is_ok()
    }

    /// Get public key as hex string
    pub fn public_key_hex(&self) -> String {
        hex::encode(self.verifying_key.to_bytes())
    }

    /// Compute DID from agent_id, version, and public key
    pub fn compute_did(&self, agent_id: &str, version: &str) -> String {
        format!("did:aeon:{}:{}:{}", agent_id, version, self.public_key_hex())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keypair_generation() {
        let keyring = AeonKeyring::init().unwrap();
        assert_eq!(keyring.public_key_hex().len(), 64); // 32 bytes = 64 hex chars
    }

    #[test]
    fn test_sign_and_verify() {
        let keyring = AeonKeyring::init().unwrap();
        let message = b"test message";
        let signature = keyring.sign(message);
        assert!(keyring.verify(message, &signature));
    }

    #[test]
    fn test_invalid_signature() {
        let keyring = AeonKeyring::init().unwrap();
        let message = b"test message";
        let wrong_message = b"wrong message";
        let signature = keyring.sign(message);
        assert!(!keyring.verify(wrong_message, &signature));
    }
}
