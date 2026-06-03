@echo off
chcp 65001 >nul
title JARVIS - installer build
cd /d "%~dp0"
call "%~dp0scripts\build\build_installer.bat"
echo.
pause
