@echo off
chcp 65001 >nul
setlocal EnableDelayedExpansion

set "ROOT=%~dp0..\..\"
cd /d "%ROOT%"

echo.
echo ========================================
echo   JARVIS - установка зависимостей
echo ========================================
echo.

where rustc >nul 2>&1
if errorlevel 1 (
    echo [ОШИБКА] Rust не найден. Установите: https://rustup.rs
    pause
    exit /b 1
)

where node >nul 2>&1
if errorlevel 1 (
    echo [ОШИБКА] Node.js не найден. Установите: https://nodejs.org
    pause
    exit /b 1
)

where python >nul 2>&1
if errorlevel 1 (
    echo [ОШИБКА] Python не найден.
    pause
    exit /b 1
)

where link >nul 2>&1
if errorlevel 1 (
    echo [ВНИМАНИЕ] Компилятор MSVC не найден в PATH.
    echo Устанавливаю Visual Studio Build Tools...
    winget install --id Microsoft.VisualStudio.2022.BuildTools -e --accept-source-agreements --accept-package-agreements --override "--wait --passive --add Microsoft.VisualStudio.Workload.VCTools --includeRecommended"
    if errorlevel 1 (
        echo [ОШИБКА] Не удалось установить Build Tools.
        echo Установите вручную: "Desktop development with C++"
        pause
        exit /b 1
    )
    echo.
    echo Перезапустите scripts\build\setup.bat после установки Build Tools.
    pause
    exit /b 0
)

echo [1/4] npm install (frontend)...
cd /d "%ROOT%frontend"
call npm install
if errorlevel 1 goto :fail
call npx routify
if errorlevel 1 goto :fail

echo.
echo [2/4] Сборка проекта (это займёт 10-30 минут)...
call "%ROOT%scripts\build\build_internal.bat"
if errorlevel 1 goto :fail

echo.
echo ========================================
echo   Установка завершена!
echo   Запускайте: ► ЗАПУСК JARVIS.vbs
echo ========================================
pause
exit /b 0

:fail
echo.
echo [ОШИБКА] Установка прервана. Смотрите сообщения выше.
pause
exit /b 1
