# PaperTalk Windows 安装指南

## 第一步：安装 Node.js

1. 访问 [Node.js 官网](https://nodejs.org/)
2. 下载 **LTS 版本**（推荐版本 18+）
3. 运行安装程序，按照默认设置安装
4. 安装完成后，打开命令提示符（cmd）验证安装：
   ```cmd
   node --version
   npm --version
   ```

## 第二步：安装 Rust

1. 访问 [Rust 官网](https://rustup.rs/)
2. 下载 `rustup-init.exe`
3. 运行安装程序，选择默认选项（按 1 回车）
4. 安装完成后，重启命令提示符，验证安装：
   ```cmd
   rustc --version
   cargo --version
   ```

## 第三步：安装 Visual Studio Build Tools

1. 下载 [Visual Studio Build Tools](https://visualstudio.microsoft.com/zh-hans/visual-cpp-build-tools/)
2. 运行安装程序
3. 选择 "C++ 生成工具" 工作负载
4. 在右侧勾选：
   - MSVC v143 - VS 2022 C++ x64/x86 生成工具
   - Windows 11 SDK
5. 点击安装

## 第四步：安装 Ollama

1. 访问 [Ollama 官网](https://ollama.ai/)
2. 下载 Windows 版本
3. 运行安装程序
4. 安装完成后，打开新的命令提示符验证：
   ```cmd
   ollama --version
   ```

## 第五步：下载 AI 模型

```cmd
ollama pull qwen3:8b-q4_K_M
```
这个模型大约 5.1GB，下载需要一些时间。

## 第六步：设置 PaperTalk 项目

1. 打开命令提示符，进入项目目录：
   ```cmd
   cd d:\Users\Administrator\code\papertalk
   ```

2. 安装项目依赖：
   ```cmd
   npm install
   ```

## 第七步：启动应用

### 开发模式运行：
```cmd
npm run tauri:dev
```

### 构建生产版本：
```cmd
npm run tauri:build
```
构建完成后，安装包会在 `src-tauri/target/release/bundle/` 目录中。

## 故障排除

### 如果 npm install 失败：
- 确保 Node.js 已正确安装
- 尝试清理缓存：`npm cache clean --force`
- 使用淘宝镜像：`npm config set registry https://registry.npmmirror.com`

### 如果 Rust 安装失败：
- 确保已安装 Visual Studio Build Tools
- 尝试使用管理员权限运行命令提示符

### 如果 Ollama 无法连接：
- 确保 Ollama 服务正在运行
- 检查端口 11434 是否被占用
- 重启 Ollama 服务

### 如果构建失败：
- 确保所有依赖都已正确安装
- 检查系统 PATH 环境变量
- 尝试重启计算机

## 快速验证步骤

1. 打开命令提示符
2. 依次运行以下命令验证环境：
   ```cmd
   node --version      # 应该显示版本号
   npm --version       # 应该显示版本号
   rustc --version     # 应该显示 Rust 版本
   ollama --version    # 应该显示 Ollama 版本
   ```
3. 如果所有命令都正常，就可以运行应用了

## 注意事项

- 确保系统有足够的磁盘空间（至少 10GB）
- 首次启动可能需要一些时间下载依赖
- 建议在稳定的网络环境下操作
- 如果遇到防火墙提示，请允许应用访问网络

完成以上步骤后，PaperTalk 就可以在 Windows 上正常运行了！
