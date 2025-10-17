@echo off
chcp 65001 >nul
echo ========================================
echo PaperTalk 构建修复脚本
echo ========================================
echo.

echo 1. 检查 Rust 环境...
rustc --version >nul 2>&1
if %errorlevel% neq 0 (
    echo [错误] Rust 未正确安装
    echo 请重新安装 Rust: https://rustup.rs/
    echo 安装后重启命令提示符
    pause
    exit /b 1
)
echo [成功] Rust 环境正常

echo.

echo 2. 清理构建缓存...
cd src-tauri
cargo clean
echo [完成] 清理完成

echo.

echo 3. 手动运行构建脚本...
echo 这将生成必要的构建文件...
cargo build --verbose
if %errorlevel% neq 0 (
    echo [错误] 构建失败
    echo 请检查详细错误信息
    pause
    exit /b 1
)

echo [成功] 构建完成！

echo.

echo 4. 验证 OUT_DIR 环境变量...
echo 检查构建输出目录...
dir target\debug\build\ 2>nul
if %errorlevel% neq 0 (
    echo [警告] 构建目录可能未正确生成
    echo 尝试重新构建...
    cargo clean
    cargo build
)

echo.

echo 5. 现在尝试启动开发服务器...
cd ..
npm run tauri:dev

pause
