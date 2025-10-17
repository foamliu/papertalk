@echo off
chcp 65001 >nul
echo ========================================
echo PaperTalk 修复验证
echo ========================================
echo.

echo 1. 清理之前的构建缓存...
cargo clean
echo [完成] 清理完成

echo.

echo 2. 检查依赖解析...
cargo check
if %errorlevel% neq 0 (
    echo [错误] 依赖解析失败
    echo 请检查 Cargo.toml 配置
    pause
    exit /b 1
)
echo [成功] 依赖解析通过

echo.

echo 3. 构建项目...
cargo build
if %errorlevel% neq 0 (
    echo [错误] 构建失败
    pause
    exit /b 1
)

echo [成功] 构建完成！
echo.
echo 4. 现在可以运行完整开发环境：
echo    npm run tauri:dev
echo.
echo ========================================
echo 修复验证完成！
echo ========================================
echo.
pause
