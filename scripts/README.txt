scripts\build\  — сборка и установка
  setup.bat           первая установка (Rust, npm, полная сборка)
  rebuild_gui.bat     интерфейс + jarvis-gui (после правок UI)
  rebuild_backend.bat jarvis-app + jarvis-gui + post_build
  build_installer.bat MSI и Setup.exe
  build_internal.bat  полная сборка (вызывается из setup)
  build_debug_app.bat отладочная сборка jarvis-app

scripts\tools\  — вспомогательное
  post_build.py       копирует DLL и resources в target\
  create_desktop_shortcut.ps1

В корне проекта — только ► ЗАПУСК и ► СОБРАТЬ (см. КАК_ЗАПУСКАТЬ.txt).
