use anyhow::Result;
use bip39::Mnemonic;
use blake2::{Blake2b, digest::{consts::U32, Digest}};
use ed25519_dalek::SigningKey;
use slip10_ed25519::derive_ed25519_private_key;
use rand::{rngs::OsRng, RngCore};

pub struct SuiWallet {
    pub address: String,
    pub public_key: String,
    pub private_key: String,
    pub mnemonic: String,
}

impl SuiWallet {
    pub fn generate() -> Result<Self> {
        // 生成助记词
        let mut entropy = [0u8; 16]; // 128位熵用于12词助记词
        OsRng.fill_bytes(&mut entropy);
        let mnemonic = Mnemonic::from_entropy(&entropy)?;
        let seed = mnemonic.to_seed("");
        
        // 密钥派生（Sui官方标准）
        // 派生路径: m/44'/784'/0'/0'/0'
        let derivation_path = [
            0x8000002C, // 44' (hardened)
            0x80000310, // 784' (hardened, Sui币种代码)
            0x80000000, // 0' (hardened, 账户索引)
            0x80000000, // 0' (hardened, 变更索引)
            0x80000000, // 0' (hardened, 地址索引)
        ];
        
        let private_key_bytes = derive_ed25519_private_key(&seed, &derivation_path);
        
        // 生成公钥
        let signing_key = SigningKey::from_bytes(&private_key_bytes);
        let public_key = signing_key.verifying_key();
        
        // 计算地址
        let scheme_byte = 0x00u8; // Ed25519标识
        let mut hasher = Blake2b::<U32>::default();
        Digest::update(&mut hasher, &[scheme_byte]);
        Digest::update(&mut hasher, public_key.as_bytes());
        let result = hasher.finalize();
        let address = format!("0x{}", hex::encode(result));
        
        Ok(SuiWallet {
            address,
            public_key: hex::encode(public_key.as_bytes()),
            private_key: hex::encode(&private_key_bytes),
            mnemonic: mnemonic.to_string(),
        })
    }
    
    // 从已有助记词生成钱包
    pub fn from_mnemonic(mnemonic: &Mnemonic) -> Result<Self> {
        let seed = mnemonic.to_seed("");
        
        // 密钥派生（Sui官方标准）
        let derivation_path = [
            0x8000002C, // 44' (hardened)
            0x80000310, // 784' (hardened, Sui币种代码)
            0x80000000, // 0' (hardened)
            0x80000000, // 0' (hardened)
            0x80000000, // 0' (hardened)
        ];
        
        let private_key_bytes = derive_ed25519_private_key(&seed, &derivation_path);
        
        let signing_key = SigningKey::from_bytes(&private_key_bytes);
        let public_key = signing_key.verifying_key();
        
        let scheme_byte = 0x00u8;
        let mut hasher = Blake2b::<U32>::default();
        Digest::update(&mut hasher, &[scheme_byte]);
        Digest::update(&mut hasher, public_key.as_bytes());
        let result = hasher.finalize();
        let address = format!("0x{}", hex::encode(result));
        
        Ok(SuiWallet {
            address,
            public_key: hex::encode(public_key.as_bytes()),
            private_key: hex::encode(&private_key_bytes),
            mnemonic: mnemonic.to_string(),
        })
    }
}
