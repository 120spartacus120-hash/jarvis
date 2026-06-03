@echo off
chcp 65001 >nul
cd /d "C:\Users\Asus\Desktop\EXE\Jarvis\jarvis-master"
call "scripts\build\rebuild_gui.bat" > rebuild-log.txt 2>&1
echo EXIT=%ERRORLEVEL%>> rebuild-log.txt
