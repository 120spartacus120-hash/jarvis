Option Explicit

Dim fso, shell, root, releaseDir, exePath, exeFile
Dim indexPath, indexFile, answer

Set fso = CreateObject("Scripting.FileSystemObject")
Set shell = CreateObject("WScript.Shell")
root = fso.GetParentFolderName(WScript.ScriptFullName)
releaseDir = root & "\target\release"
exePath = releaseDir & "\jarvis-gui.exe"

If Not fso.FileExists(exePath) Then
    releaseDir = root & "\target\debug"
    exePath = releaseDir & "\jarvis-gui.exe"
End If

If Not fso.FileExists(exePath) Then
    MsgBox "JARVIS ещё не собран." & vbCrLf & vbCrLf & _
        "Запустите в папке проекта:" & vbCrLf & _
        "► СОБРАТЬ JARVIS.bat", vbCritical, "JARVIS"
    WScript.Quit 1
End If

Set exeFile = fso.GetFile(exePath)

indexPath = root & "\frontend\dist\client\index.html"
If fso.FileExists(indexPath) Then
    Set indexFile = fso.GetFile(indexPath)
    If indexFile.DateLastModified > exeFile.DateLastModified Then
        answer = MsgBox( _
            "Интерфейс обновлён, а jarvis-gui.exe ещё старый." & vbCrLf & vbCrLf & _
            "Чтобы увидеть выбор городов (Москва, Челябинск, Якутск), нажмите «Нет» и запустите:" & vbCrLf & _
            "► СОБРАТЬ JARVIS.bat" & vbCrLf & vbCrLf & _
            "Запустить старую версию сейчас?", _
            vbYesNo + vbExclamation, "JARVIS")
        If answer = vbNo Then WScript.Quit 0
    End If
End If

shell.Run "cmd /c subst J: /d >nul 2>&1 & subst J: """ & releaseDir & """", 0, True
If Err.Number <> 0 Then
    MsgBox "Ne udalos podklyuchit disk J.", vbExclamation, "JARVIS"
    WScript.Quit 1
End If

shell.CurrentDirectory = "J:\"
shell.Run """J:\jarvis-gui.exe""", 0, False
