# 解决 "OUT_DIR env var is not set" 错误

## 问题分析

`OUT_DIR` 环境变量未设置错误通常发生在以下情况：

1. **构建脚本未正确执行** - `build.rs` 文件没有被 Cargo 处理
2. **Rust 工具链问题** - 安装不完整或配置错误
3. **构建缓存问题** - 之前的构建状态干扰
4. **权限问题** - 无法创建构建目录

## 解决方案

### 方法一：完全重新安装 Rust 环境

1. **卸载并重新安装 Rust**:
   ```cmd
   rustup self uninstall
   ```
   然后重新安装: https://rustup.rs/

2. **安装完成后验证**:
   ```cmd
   rustc --version
   cargo --version
   rustup show
   ```

### 方法二：手动设置构建环境

1. **手动运行构建脚本**:
   ```cmd
   cd src-tauri
   cargo build -v  # 使用详细输出
   ```

2. **检查构建输出**:
   ```cmd
   dir target\debug\build\
   ```
   应该能看到生成的构建文件。

### 方法三：使用提供的修复脚本

运行 `fix-build.bat` 脚本，它会：
- 检查 Rust 环境
- 清理构建缓存
- 手动运行构建
- 验证构建目录

### 方法四：检查 Visual Studio Build Tools

确保已安装正确的构建工具：
- **Visual Studio Build Tools 2022**
- **C++ 构建工具**
- **Windows 11 SDK**

### 方法五：环境变量检查

检查以下环境变量是否设置：
```cmd
echo %CARGO_HOME%
echo %RUSTUP_HOME%
echo %OUT_DIR%
```

## 逐步解决流程

### 步骤 1: 验证 Rust 安装
```cmd
rustc --version
cargo --version
rustup component list
```

### 步骤 2: 清理项目
```cmd
cd src-tauri
cargo clean
```

### 步骤 3: 手动构建
```cmd
cargo build --verbose
```

### 步骤 4: 检查构建输出
```cmd
dir target\debug\build\tauri-build-*
```

应该看到类似这样的目录：
```
target/debug/build/tauri-build-xxxxxxx/
```

### 步骤 5: 启动开发服务器
```cmd
cd ..
npm run tauri:dev
```

## 如果仍然失败

### 检查构建脚本权限
确保 `src-tauri/build.rs` 文件存在且内容正确。

### 检查 Cargo.toml 配置
确保 `[build-dependencies]` 部分正确：
```toml
[build-dependencies]
tauri-build = { version = "1.5", features = [] }
```

### 使用 Rust 稳定版本
```cmd
rustup default stable
```

### 检查磁盘空间
确保有足够的磁盘空间用于构建。

## 成功标志

当构建成功时，您应该看到：
- `cargo build` 命令完成没有错误
- `target/debug/build/` 目录中有生成的文件
- `npm run tauri:dev` 能够正常启动应用窗口

## 紧急解决方案

如果所有方法都失败，可以尝试：

1. **使用预构建的二进制文件** (如果可用)
2. **在不同的机器上构建**
3. **使用 Docker 环境构建**

## 获取帮助

如果问题持续存在，请提供：
1. 完整的错误信息
2. `cargo build --verbose` 的输出
3. 系统环境信息
4. Rust 和 Cargo 版本信息

提交到：https://github.com/tauri-apps/tauri/issues
