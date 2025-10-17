# PaperTalk - PDF Reading Assistant

PaperTalk 是一个基于 Tauri + Vue3 的桌面应用程序，专门为科研人员和学生设计，提供本地化的 PDF 阅读和翻译功能。

## 功能特性

- 📄 **PDF 阅读器** - 支持导入和阅读 PDF 文件
- 🔍 **划词翻译** - 选择文本后自动翻译，支持本地 LLM
- 🌙 **夜间模式** - 舒适的阅读体验
- 📝 **笔记功能** - 为重要内容添加笔记
- 🔒 **本地运行** - 所有数据本地处理，保护隐私
- 🚀 **快速启动** - 冷启动时间 ≤ 3 秒

## 技术栈

- **前端**: Vue 3 + Element Plus + Pinia
- **后端**: Tauri + Rust
- **PDF 渲染**: pdfjs-dist
- **AI 翻译**: Ollama (qwen3:8b-q4_K_M 模型)
- **数据库**: SQLite

## 系统要求

- Windows 10/11, macOS 10.15+, 或 Ubuntu 18.04+
- 至少 8GB RAM
- 至少 10GB 可用存储空间（用于模型下载）

## Windows 安装步骤（推荐）

### 方法一：使用一键安装脚本

1. 双击运行 `setup.bat`
2. 脚本会自动检查环境并安装依赖
3. 按照提示完成后续步骤

### 方法二：手动安装

#### 1. 安装 Node.js
- 访问 [Node.js 官网](https://nodejs.org/) 下载 LTS 版本
- 运行安装程序，按默认设置安装
- 验证安装：打开 cmd，运行 `node --version` 和 `npm --version`

#### 2. 安装 Rust
- 访问 [Rust 官网](https://rustup.rs/) 下载 rustup-init.exe
- 运行安装程序，选择默认选项（按 1 回车）
- 验证安装：`rustc --version`

#### 3. 安装 Visual Studio Build Tools
- 下载 [Visual Studio Build Tools](https://visualstudio.microsoft.com/zh-hans/visual-cpp-build-tools/)
- 安装时选择 "C++ 生成工具" 工作负载
- 勾选 MSVC v143 和 Windows 11 SDK

#### 4. 安装 Ollama
- 访问 [Ollama 官网](https://ollama.ai/) 下载 Windows 版本
- 运行安装程序
- 验证安装：`ollama --version`

#### 5. 下载 AI 模型
```cmd
ollama pull qwen3:8b-q4_K_M
```

#### 6. 安装项目依赖
```cmd
cd d:\Users\Administrator\code\papertalk
npm install
```

#### 7. 启动应用
```cmd
npm run tauri:dev
```

## Linux/macOS 安装步骤

### 1. 安装依赖

```bash
# 安装 Node.js 依赖
npm install

# 安装 Rust 工具链 (如果尚未安装)
# 访问 https://rustup.rs/ 安装 Rust
```

### 2. 安装 Ollama

```bash
# macOS
brew install ollama

# Linux
curl -fsSL https://ollama.ai/install.sh | sh
```

### 3. 下载模型

```bash
ollama pull qwen3:8b-q4_K_M
```

### 4. 开发模式运行

```bash
npm run tauri:dev
```

### 5. 构建应用

```bash
npm run tauri:build
```

## 使用说明

1. **首次启动**: 应用会自动检测 Ollama 状态，如果未检测到会提示安装
2. **打开 PDF**: 点击"打开PDF"按钮或拖拽文件到应用窗口
3. **划词翻译**: 在 PDF 中选择文本，右侧面板会显示翻译按钮
4. **添加笔记**: 在右侧笔记区域输入内容，自动保存
5. **切换模式**: 点击右上角按钮切换日间/夜间模式

## 快捷键

- `Ctrl + T`: 翻译选中的段落
- `Ctrl + B`: 高亮文本
- `Ctrl + N`: 新建笔记
- `Ctrl + O`: 打开文件

## 项目结构

```
papertalk/
├── src/                    # 前端源码
│   ├── App.vue            # 主应用组件
│   ├── main.js            # 应用入口
│   └── stores/            # 状态管理
├── src-tauri/             # Rust 后端
│   ├── src/
│   │   └── main.rs        # Tauri 应用入口
│   ├── Cargo.toml         # Rust 依赖配置
│   └── tauri.conf.json    # Tauri 应用配置
├── docs/                  # 文档
└── package.json           # Node.js 依赖配置
```

## 性能指标

- 冷启动时间: ≤ 3 秒
- 划词翻译延迟: ≤ 800 ms (RTX3060)
- 内存占用: ≤ 300 MB (不含模型)
- 渲染 10 页 PDF: ≤ 1 秒

## 开发计划

- [x] MVP 版本基础功能
- [ ] PDF 实际渲染实现
- [ ] 文件对话框集成
- [ ] 数据库持久化
- [ ] 快捷键支持
- [ ] 术语库功能
- [ ] 批量翻译

## 许可证

MIT License

## 贡献

欢迎提交 Issue 和 Pull Request！
