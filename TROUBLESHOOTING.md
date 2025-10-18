# PaperTalk 故障排除指南

## 常见构建问题及解决方案

### 1. "OUT_DIR env var is not set" 错误

**问题描述**:
```
error: OUT_DIR env var is not set, do you have a build script?
```

**解决方案**:
1. 确保 Rust 工具链已正确安装
2. 运行以下命令重新构建:
   ```cmd
   cd src-tauri
   cargo clean
   cargo build
   ```

### 2. Rust/Cargo 命令未找到

**问题描述**:
```
cargo : 无法将“cargo”项识别为 cmdlet、函数、脚本文件或可运行程序的名称
```

**解决方案**:
1. 重新安装 Rust: https://rustup.rs/
2. 安装完成后重启命令提示符
3. 验证安装: `rustc --version` 和 `cargo --version`

### 3. Tauri 依赖版本冲突

**问题描述**:
```
failed to select a version for `gtk-sys`
```

**解决方案**:
- 确保所有 Tauri 相关依赖使用相同版本
- 当前项目使用 Tauri v1.5

### 4. 构建环境配置

#### Windows 环境要求:
- **Node.js** 16+ (推荐 LTS 版本)
- **Rust** (通过 rustup 安装)
- **Visual Studio Build Tools** 或 **Visual Studio 2022**
  - 需要 C++ 构建工具
  - Windows 11 SDK

#### 验证环境:
```cmd
node --version
npm --version
rustc --version
cargo --version
```

### 5. 快速修复步骤

#### 方法一: 完全重新构建
```cmd
# 清理项目
cd src-tauri
cargo clean

# 重新构建
cargo build

# 启动开发服务器
cd ..
npm run tauri:dev
```

#### 方法二: 使用提供的脚本
```cmd
# 运行验证脚本
verify-fix.bat

# 如果验证通过，运行启动脚本
start-app.bat
```

### 6. 网络问题

如果下载依赖超时，可以配置 Rust 镜像:

**创建或编辑 `~/.cargo/config`**:
```toml
[source.crates-io]
replace-with = 'tuna'

[source.tuna]
registry = "https://mirrors.tuna.tsinghua.edu.cn/git/crates.io-index.git"
```

### 7. 权限问题

在 Windows 上，可能需要以管理员权限运行命令提示符。

### 8. 磁盘空间

确保有足够的磁盘空间:
- 至少 2GB 用于 Rust 工具链
- 至少 1GB 用于 Node.js 依赖
- 至少 5GB 用于 Ollama 模型

### 9. 防火墙设置

如果遇到网络连接问题，确保防火墙允许:
- Node.js 访问网络
- Cargo 下载依赖
- Ollama 服务运行

## 如果所有方法都失败

1. **完全重新安装环境**:
   - 卸载并重新安装 Node.js
   - 重新安装 Rust: `rustup self uninstall` 然后重新安装
   - 删除 `node_modules` 和 `src-tauri/target` 文件夹

2. **使用 Docker** (高级选项):
   ```dockerfile
   # 可以使用 Tauri 官方 Docker 镜像进行构建
   ```

3. **寻求社区帮助**:
   - Tauri Discord: https://discord.gg/tauri
   - GitHub Issues: 提交详细错误信息

## 成功构建的标志

当构建成功时，您应该看到:
- Vite 开发服务器启动 (http://localhost:1420)
- Tauri 应用窗口打开
- 没有编译错误
- 应用界面正常显示

如果遇到其他问题，请记录完整的错误信息并参考相关文档。
