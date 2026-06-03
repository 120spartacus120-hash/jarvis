# Обновления JARVIS через GitHub

## 1. Залить код на GitHub (один раз)

Сейчас Git на ПК залогинен как **другой аккаунт** (`1Spartacus1`). Репозиторий — **120spartacus120-hash/jarvis**.

В PowerShell:

```powershell
cd "C:\Users\Asus\Desktop\EXE\Jarvis\jarvis-master"
gh auth login
# выбери GitHub.com → HTTPS → Login with browser → аккаунт 120spartacus120-hash

git push -u origin main
```

Если ветка называется `master`:

```powershell
git branch -M main
git push -u origin main
```

## 2. Секрет для подписи обновлений

1. Открой репозиторий → **Settings** → **Secrets and variables** → **Actions** → **New repository secret**
2. Имя: `TAURI_SIGNING_PRIVATE_KEY`
3. Значение: **весь текст** из файла  
   `.tauri/jarvis-updater.key`  
   (одна строка, начинается с `dW50cnVzdGVk...`)
4. Секрет `TAURI_SIGNING_PRIVATE_KEY_PASSWORD` — **не нужен** (ключ без пароля).

Публичный ключ уже прописан в `crates/jarvis-gui/tauri.conf.json`.

## 3. Выпустить версию

1. Подними версию в `Cargo.toml` (`workspace.package.version`) и `crates/jarvis-gui/tauri.conf.json` (`version`), например `1.0.1`.
2. Закоммить и запушить.
3. Создай тег:

```powershell
git tag v1.0.1
git push origin v1.0.1
```

4. В **Actions** дождись зелёной галочки **Release**.
5. На странице **Releases** появятся установщик и `latest.json`.

## 4. Как это работает у пользователя

- При запуске JARVIS проверяет  
  `https://github.com/120spartacus120-hash/jarvis/releases/latest/download/latest.json`
- Если версия новее — сверху баннер **«Доступно обновление»** → **Обновить**.
- После установки программа перезапускается.

Первую установку друг всё равно делает один раз через `setup.exe` из Releases.

## 5. Локальная сборка с подписью

```powershell
$env:TAURI_SIGNING_PRIVATE_KEY = Get-Content ".\.tauri\jarvis-updater.key" -Raw
cd frontend
npm run build
cd ..
cargo build -p jarvis-app --release
cargo tauri build -p jarvis-gui --target x86_64-pc-windows-msvc
```
