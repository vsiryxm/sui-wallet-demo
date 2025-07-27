use anyhow::Result;
use sui_wallet_libs::SuiWallet;

// 使用示例
fn main() -> Result<()> {
    println!("=== Sui钱包生成验证测试 ===\n");
    
    // 生成新钱包
    println!("🔹 生成新的Sui钱包");
    let wallet = SuiWallet::generate()?;
    
    println!("✅ 新的Sui钱包已生成！");
    println!("地址: {}", wallet.address);
    println!("公钥: {}", wallet.public_key);
    println!("私钥: {}", wallet.private_key);
    println!("助记词: {}", wallet.mnemonic);
    println!("地址长度: {} 字符 ({} 字节)", wallet.address.len(), (wallet.address.len() - 2) / 2);
    
    // 验证地址格式
    assert!(wallet.address.starts_with("0x"), "地址应该以0x开头");
    assert_eq!(wallet.address.len(), 66, "Sui地址应该是66个字符长度（0x + 64位十六进制）");
    
    println!("✅ 地址格式验证通过\n");
    
    Ok(())
}
