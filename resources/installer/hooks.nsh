; JARVIS — дополнительные шаги мастера установки NSIS

!macro NSIS_HOOK_PREINSTALL
  ; Закрыть старые процессы перед копированием файлов
  nsExec::ExecToLog 'taskkill /F /IM jarvis-gui.exe /T'
  Pop $0
  nsExec::ExecToLog 'taskkill /F /IM jarvis-app.exe /T'
  Pop $0
!macroend

!macro NSIS_HOOK_POSTINSTALL
  ; Ярлык на рабочем столе (если пользователь не отключил в мастере)
  CreateShortCut "$DESKTOP\JARVIS.lnk" "$INSTDIR\jarvis-gui.exe" "" "$INSTDIR\jarvis-gui.exe" 0
!macroend
