@echo off
chcp 65001 >nul
echo ========================================
echo PaperTalk Windows 安装脚本
echo ========================================
echo.

echo 检查 Node.js...
node --version >nul 2>&1
if %errorlevel% neq 0 (
    echo [错误] Node.js 未安装
    echo 请从 https://nodejs.org/ 下载安装 Node.js LTS 版本
    echo 安装完成后重新运行此脚本
    pause
    exit /b 1
)
echo [成功] Node.js 已安装: 
node --version

echo.

echo 检查 Rust...
rustc --version >nul 2>&1
if %errorlevel% neq 0 (
    echo [错误] Rust 未安装
    echo 请从 https://rustup.rs/ 下载安装 Rust
    echo 安装完成后重新运行此脚本
    pause
    exit /b 1
)
echo [成功] Rust 已安装: 
rustc --version

echo.

echo 检查 Ollama...
ollama --version >nul 2>&1
if %errorlevel% neq 0 (
    echo [警告] Ollama 未安装
    echo 请从 https://ollama.ai/ 下载安装 Ollama
    echo 安装后运行: ollama pull qwen3:8b-q4_K_M
)

echo.

echo 安装 npm 依赖...
npm install
if %errorlevel% neq 0 (
    echo [错误] npm 依赖安装失败
    echo 请检查网络连接或使用淘宝镜像:
    echo npm config set registry https://registry.npmmirror.com
    pause
    exit /b 1
)

echo.
echo ========================================
echo 安装完成！
echo ========================================
echo.
echo 下一步操作:
echo 1. 如果 Ollama 未安装，请从 https://ollama.ai/ 下载安装
echo 2. 下载 AI 模型: ollama pull qwen3:8b-q4_K_M
echo 3. 启动应用: npm run tauri:dev
echo.
echo 详细说明请查看 WINDOWS_INSTALL_GUIDE.md
echo.
pause
