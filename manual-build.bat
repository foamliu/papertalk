@echo off
chcp 65001 >nul
echo ========================================
echo PaperTalk 手动构建启动
echo ========================================
echo.

echo 1. 停止之前的开发服务器...
taskkill /f /im node.exe >nul 2>&1
echo [完成] 清理完成

echo.

echo 2. 构建前端...
npm run build
if %errorlevel% neq 0 (
    echo [错误] 前端构建失败
    pause
    exit /b 1
)
echo [成功] 前端构建完成

echo.

echo 3. 构建 Rust 后端...
cd src-tauri
cargo build --release
if %errorlevel% neq 0 (
    echo [错误] Rust 构建失败
    pause
    exit /b 1
)
echo [成功] Rust 构建完成

echo.

echo 4. 启动应用...
echo 注意: 这将直接运行编译后的应用，而不是开发服务器
.\target\release\papertalk.exe
if %errorlevel% neq 0 (
    echo [错误] 应用启动失败
    echo 尝试使用开发模式...
    cd ..
    npm run tauri:dev
)

pause
