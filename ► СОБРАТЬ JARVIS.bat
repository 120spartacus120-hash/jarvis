@echo off
chcp 65001 >nul
title Сборка JARVIS
cd /d "%~dp0"
echo После сборки запускайте только: ► ЗАПУСК JARVIS.vbs
echo.
call "%~dp0scripts\build\rebuild_gui.bat"
echo.
pause
