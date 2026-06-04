@echo off
chcp 65001 >nul
echo ==========================================
echo    Очистка кэша разработчика JARVIS
echo ==========================================
echo.

echo [1/2] Очистка кэша Rust (папка target)...
cargo clean
echo.

echo [2/2] Удаление кэша Node.js (папка frontend/node_modules)...
if exist "frontend\node_modules" (
    rmdir /s /q "frontend\node_modules"
    echo Кэш фронтенда удален.
) else (
    echo Папка node_modules уже пуста.
)
echo.

echo ==========================================
echo Очистка завершена! Место на диске освобождено.
echo ВНИМАНИЕ: При следующем запуске проекта потребуется ввести:
echo 1. cd frontend
echo 2. npm install
echo ==========================================
pause
