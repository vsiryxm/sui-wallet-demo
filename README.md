# Sui钱包生成DEMO

一个基于Sui区块链的钱包生成DEMO，遵循Sui钱包标准。

## 🚀 DEMO功能

- ✅ 生成符合Sui标准的钱包地址
- ✅ 支持BIP39助记词生成和恢复
- ✅ 使用Ed25519椭圆曲线密码学
- ✅ 采用SLIP-0010密钥派生标准
- ✅ Blake2b256哈希算法实现
- ✅ 与官方Suiet钱包完全兼容

## 📋 输出信息

每个生成的钱包包含：
- **钱包地址**: 64字符的十六进制地址（0x开头）
- **公钥**: 32字节的Ed25519公钥
- **私钥**: 32字节的Ed25519私钥  
- **助记词**: 12个单词的BIP39助记词

## 🛠️ 安装与运行

### 前置要求
- Rust 1.70+
- Cargo

### 克隆项目
```bash
git clone <项目地址>
cd sui-wallet
```

### 安装依赖
```bash
cargo build
```

### 运行钱包生成器
```bash
cargo run --bin sui-wallet-demo
```

## 🧪 测试

### 运行所有测试
```bash
cargo test
```

### 运行性能基准测试
```bash
cargo bench
```

## 📊 示例输出

```
=== Sui钱包生成验证测试 ===

🔹 生成新的Sui钱包
✅ 新的Sui钱包已生成！
地址: 0xee80fbd4fce180310880f53086cd85d3df48f81bad03020d82c0057a8656eb3d
公钥: 1706132e833aca83f77fc3896d620214c1f48830b5db7929a6e02bb3e0398330
私钥: bc91a957874eb0d41ee52ceed9ebbcb02dc1ba93ced2bf7616e4adf330faddcc
助记词: rich soldier vacant zero sea asset soup frozen pen ritual story ask
地址长度: 66 字符 (32 字节)
✅ 地址格式验证通过
```

## 🔧 技术实现

### 核心技术栈
- **椭圆曲线**: Ed25519 (更安全、更快速)
- **密钥派生**: SLIP-0010 标准
- **哈希算法**: Blake2b256
- **助记词**: BIP39 标准

### 地址生成流程
1. 生成BIP39助记词（128位熵）
2. 使用SLIP-0010派生私钥（路径: m/44'/784'/0'/0'/0'）
3. 从私钥生成Ed25519公钥
4. 计算地址: Blake2b256(0x00 || public_key)

## ⚠️ 安全提醒

- 🔒 **私钥安全**: 私钥拥有钱包完全控制权，请妥善保管
- 💾 **助记词备份**: 助记词是恢复钱包的唯一方式，务必安全备份
- 🧪 **测试环境**: 此为演示代码，生产使用请加强安全措施
- 🚫 **不要分享**: 永远不要在公开场合分享私钥或助记词

## 📚 相关文档

- [Sui官方文档](https://docs.sui.io/)
- [Ed25519签名算法](https://ed25519.cr.yp.to/)
- [SLIP-0010标准](https://github.com/satoshilabs/slips/blob/master/slip-0010.md)
- [Blake2b哈希算法](https://blake2.net/)


