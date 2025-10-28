# PaperTalk CI/CD 配置指南

本文档说明如何配置 GitHub Actions 来自动构建 PaperTalk 应用程序。

## 功能特性

- ✅ **多平台构建**: 支持 Windows、macOS、Linux 三个平台
- ✅ **定时构建**: 每天自动构建最新版本
- ✅ **发布管理**: 自动创建 GitHub Release
- ✅ **代码签名**: 支持应用签名（需要配置）

## 配置步骤

### 1. 设置 GitHub Secrets

为了启用代码签名，需要在 GitHub 仓库中设置以下 secrets：

1. 进入仓库设置 → Secrets and variables → Actions
2. 添加以下 secrets：

- `TAURI_PRIVATE_KEY`: Tauri 应用的私钥
- `TAURI_KEY_PASSWORD`: 私钥的密码（如果有）

### 2. 生成 Tauri 签名密钥

如果没有签名密钥，可以使用以下命令生成：

```bash
cd src-tauri
cargo tauri signer generate -w ~/.tauri/papertalk.key
```

然后将生成的私钥内容添加到 `TAURI_PRIVATE_KEY` secret 中。

### 3. 工作流触发条件

- **推送代码**: 当代码推送到 main/master 分支时自动构建
- **Pull Request**: 创建 PR 时进行构建测试
- **定时任务**: 每天 UTC 时间 02:00（北京时间 10:00）自动构建

## 构建产物

每个平台会生成对应的安装包：

- **Windows**: `.msi` 安装包
- **macOS**: `.dmg` 磁盘映像
- **Linux**: `.AppImage` 和 `.deb` 包

## 手动触发构建

除了自动触发外，也可以手动触发构建：

1. 进入 GitHub Actions 页面
2. 选择 "Build and Release" 工作流
3. 点击 "Run workflow" 按钮

## 故障排除

### 常见问题

1. **构建失败**: 检查 Node.js 和 Rust 依赖是否正确安装
2. **签名失败**: 确认 `TAURI_PRIVATE_KEY` 和 `TAURI_KEY_PASSWORD` 设置正确
3. **依赖安装失败**: 确保系统依赖已正确安装

### 调试建议

- 查看 GitHub Actions 日志获取详细错误信息
- 在本地运行 `npm run tauri:build` 测试构建过程
- 确保所有依赖项在 CI 环境中可用

## 自定义配置

可以根据需要修改 `.github/workflows/build.yml` 文件：

- 修改定时任务时间（cron 表达式）
- 添加更多构建平台
- 调整构建参数
- 更改发布策略
