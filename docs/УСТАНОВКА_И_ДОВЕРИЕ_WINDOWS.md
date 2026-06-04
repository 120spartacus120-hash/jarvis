# Установщик, обновления GitHub и предупреждение «вирус»

## Что уже есть в проекте

### Автообновление (GitHub)

- Код: `frontend/src/lib/updater.ts`, баннер `UpdateBanner.svelte`
- Настройка: `crates/jarvis-gui/tauri.conf.json` → `plugins.updater`
- CI: `.github/workflows/release.yml` — при теге `v1.0.1` собирается установщик и `latest.json`
- Секрет **TAURI_SIGNING_PRIVATE_KEY** — подпись **обновлений** (minisign), чтобы кнопка «Обновить» в программе была безопасной

Это **не** убирает жёлтое окно Windows при **первой** установке exe.

### Установщик

- Сборка: **NSIS** (`JARVIS_*_x64-setup.exe`), не zip и не «просто exe из папки»
- Локально: `► СОБРАТЬ УСТАНОВЩИК.bat`
- Для пользователей: файл **`*-setup.exe`** со страницы [Releases](https://github.com/120spartacus120-hash/jarvis/releases/latest)

Доп. настройки мастера: `crates/jarvis-gui/tauri.windows.conf.json`, тексты — `resources/installer/`.

---

## Почему Windows пишет «неизвестный издатель» / «вирус»

1. **Нет подписи Authenticode** — у exe нет платного сертификата издателя (DigiCert, Sectigo и т.д.).
2. **Новый файл** — мало людей скачали, у SmartScreen нет «репутации».
3. **Поведение программы** — микрофон, автозапуск, фоновый `jarvis-app.exe`; эвристика Defender может ругаться на **любой** неподписанный голосовой ассистент.

Подпись обновлений Tauri (**jarvis-updater.key**) решает только проверку обновлений внутри JARVIS, **не** установку с нуля.

---

## Как убрать предупреждение по-настоящему

### 1. Купить сертификат Code Signing (обязательно для «тихой» установки)

- Обычный **OV Code Signing** (~$200–400/год) или **EV** (дороже, репутация SmartScreen быстрее).
- Формат: `.pfx` + пароль.

### 2. Подписать установщик и exe

Локально (после сборки):

```powershell
cd "C:\Users\Asus\Desktop\EXE\Jarvis\jarvis-master"
$env:WINDOWS_CODESIGN_PFX_PATH = "C:\path\to\cert.pfx"
$env:WINDOWS_CODESIGN_PASSWORD = "пароль"
.\scripts\build\sign_windows.ps1 "target\release\bundle\nsis\JARVIS_1.0.0_x64-setup.exe"
```

Скрипт также можно подключить в CI через `bundle.windows.signCommand` в `tauri.conf.json`.

### 3. Отправить ложное срабатывание в Microsoft

Если Defender всё равно ругается после подписи:

- [Отправить файл на анализ](https://www.microsoft.com/en-us/wdsi/filesubmission) — выбрать «False positive», приложить подписанный `*-setup.exe`.

### 4. Что писать пользователям до покупки сертификата

В README / на GitHub Releases:

1. Скачивать только **`*-setup.exe`** с Releases, не сборки из чатов.
2. При SmartScreen: **Подробнее** → **Выполнить в любом случае**.
3. Не распространять старые MSI `0.0.3` из `bundle\msi\`.

---

## Выпуск версии для пользователей и автообновления

1. Версия в `Cargo.toml` и `tauri.conf.json` (например `1.0.2`).
2. `git push` + тег:

```powershell
git tag v1.0.2
git push origin v1.0.2
```

3. Дождаться зелёного **Release** в Actions.
4. На Releases появятся `JARVIS_1.0.2_x64-setup.exe` и `latest.json`.
5. У установленных пользователей при запуске — баннер «Доступно обновление» → **Обновить**.

Секрет **TAURI_SIGNING_PRIVATE_KEY** — см. `docs/КУДА_ВСТАВИТЬ_СЕКРЕТ.md`.

---

## Чеклист для тебя (Кирилл)

| Задача | Статус |
|--------|--------|
| Автообновление с GitHub | Уже в коде |
| NSIS-установщик (мастер, русский) | `tauri.windows.conf.json` |
| Убрать «вирус» полностью | Нужен **Code Signing** + при желании отправка в Microsoft |
| Не отдавать zip / VBS / старый MSI | Только `*-setup.exe` с Releases |
