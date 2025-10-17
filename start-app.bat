@echo off
chcp 65001 >nul
echo ========================================
echo PaperTalk 启动脚本
echo ========================================
echo.

echo 检查环境...
node --version >nul 2>&1
if %errorlevel% neq 0 (
    echo [错误] Node.js 未安装或未配置 PATH
    echo 请确保 Node.js 已安装并添加到系统 PATH
    pause
    exit /b 1
)

echo [成功] Node.js 已安装

echo.

echo 安装 Tauri CLI (如果未安装)...
npm list -g @tauri-apps/cli >nul 2>&1
if %errorlevel% neq 0 (
    echo 安装 Tauri CLI...
    npm install -g @tauri-apps/cli
)

echo.

echo 启动开发服务器...
echo 注意: 首次启动可能需要较长时间下载依赖
echo.
npm run tauri:dev

pause
