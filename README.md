<div align="center">
  <img src="resources/icons/128x128.png" alt="JARVIS Logo" width="128" height="128" style="border-radius: 20px;">
  
  # JARVIS Voice Assistant
  
  **Умный голосовой ассистент для Windows, работающий полностью оффлайн.**
  
  [![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
  [![Tauri](https://img.shields.io/badge/Tauri-FFC131?style=for-the-badge&logo=tauri&logoColor=white)](https://tauri.app/)
  [![Svelte](https://img.shields.io/badge/Svelte-FF3E00?style=for-the-badge&logo=svelte&logoColor=white)](https://svelte.dev/)
  [![Vite](https://img.shields.io/badge/Vite-646CFF?style=for-the-badge&logo=vite&logoColor=white)](https://vitejs.dev/)
</div>

---

## 🌟 О проекте

**JARVIS** — это современный голосовой ассистент, созданный с упором на приватность и скорость работы. В отличие от большинства современных решений, Jarvis не отправляет ваши голосовые команды в облако. Все нейросети работают локально на вашем компьютере.

### Ключевые особенности:
- 🔒 **100% Оффлайн** — не требует подключения к интернету для распознавания речи.
- 🛡️ **Приватность** — ваши данные остаются только на вашем устройстве.
- ⚡ **Скорость** — мгновенный отклик благодаря нативному ядру на Rust.
- 🔄 **Автообновления** — встроенная система бесшовных обновлений (Tauri Updater).
- 🎨 **Минималистичный дизайн** — удобный и лёгкий интерфейс на Svelte.

---

## 🧠 Технологии под капотом

Проект использует передовые локальные нейросети для обработки голоса:

*   **Распознавание речи (STT):**
    *   [Vosk Speech Recognition Toolkit](https://github.com/alphacep/vosk-api) — быстрый и точный движок.
*   **Активация голосом (Wake Word):**
    *   [Picovoice Porcupine](https://github.com/Picovoice/porcupine) — высокоточный детектор ключевых слов.
    *   [Rustpotter](https://github.com/GiviMAD/rustpotter) — открытая альтернатива (в разработке).
*   **Интерфейс и Ядро:**
    *   **Backend:** Rust + Tauri (максимальная производительность и минимальное потребление ОЗУ).
    *   **Frontend:** Svelte + Vite (быстрый и отзывчивый UI).

---

## 🌍 Поддерживаемые языки

В данный момент ассистент полностью поддерживает **Русский язык** (интерфейс, распознавание речи, команды).
В планах добавление Украинского и Английского языков.

---

## 🚀 Установка и использование

Самый простой способ начать использовать JARVIS — скачать готовый установщик из раздела Releases:

1. Перейдите на страницу [Releases](https://github.com/120spartacus120-hash/jarvis/releases/latest).
2. Скачайте файл `JARVIS_..._x64-setup.exe`.
3. Запустите установщик и следуйте инструкциям.
4. Программа автоматически будет проверять наличие обновлений при каждом запуске!

---

## 🛠️ Сборка из исходников (Для разработчиков)

Если вы хотите собрать проект самостоятельно или поучаствовать в разработке:

### Требования:
- [Rust](https://www.rust-lang.org/tools/install) (stable)
- [Node.js](https://nodejs.org/) (v20+)
- Инструменты сборки C++ для Windows (Visual Studio Build Tools)

### Инструкция:
1. Клонируйте репозиторий:
   ```bash
   git clone https://github.com/120spartacus120-hash/jarvis.git
   cd jarvis
   ```
2. Установите зависимости фронтенда:
   ```bash
   cd frontend
   npm install
   cd ..
   ```
3. Запустите проект в режиме разработки:
   ```bash
   npm run tauri dev --prefix frontend
   ```
4. Для сборки готового `.exe` файла:
   ```bash
   npm run tauri build --prefix frontend
   ```

---

## 📜 Лицензия

Проект распространяется под лицензией [Attribution-NonCommercial-ShareAlike 4.0 International](https://creativecommons.org/licenses/by-nc-sa/4.0/).
Подробности в файле `LICENSE.txt`.
