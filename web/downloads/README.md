# PaperTalk 安装文件下载目录

这个目录应该包含各个平台的安装文件：

## 文件结构建议：
```
web/downloads/
├── windows/
│   └── PaperTalk_0.1.0_x64_en-US.msi
├── macos/
│   └── PaperTalk_0.1.0_aarch64_darwin.dmg
└── linux/
    └── PaperTalk_0.1.0_amd64.deb
```

## 当前状态：
目前下载按钮只是模拟下载，显示通知信息。要启用真正的下载功能，需要：

1. **构建实际的安装文件**：
   - Windows: `.exe` 安装程序
   - macOS: `.dmg` 磁盘映像
   - Linux: `.deb` 包

2. **更新下载链接**：
   - 修改 `web/index.html` 中的下载按钮为 `<a>` 标签
   - 指向实际的安装文件路径

3. **文件命名建议**：
   - Windows: `PaperTalk-Setup-{version}.exe`
   - macOS: `PaperTalk-{version}.dmg`
   - Linux: `PaperTalk-{version}.deb`

## 如何生成安装文件：
使用 Tauri 构建命令：
```bash
npm run tauri build
```

这会为每个平台生成安装文件，通常输出到：
- `src-tauri/target/release/bundle/`

## 部署建议：
1. 将生成的安装文件复制到 `web/downloads/` 对应目录
2. 更新网站上的下载链接
3. 确保文件可以通过 web 服务器访问
