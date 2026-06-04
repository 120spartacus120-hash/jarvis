# ### APP INFO
app-name = JARVIS
app-description = Голосовий асистент

# ### TRAY MENU
tray-restart = Перезапустити
tray-settings = Налаштування
tray-exit = Вихід
tray-tooltip = JARVIS - Голосовий асистент
tray-language = Мова
tray-voice = Голос
tray-wake-word = Рушій детекції
tray-noise-suppression = Шумозаглушення
tray-vad = Детекцiя голосу (VAD)
tray-gain-normalizer = Нормалізація гучності

# ### HEADER
header-commands = КОМАНДИ
header-settings = НАЛАШТУВАННЯ

# ### SEARCH
search-placeholder = Введіть команду вручну або скажіть «Джарвіс» ...

# ### MAIN PAGE
assistant-not-running = АСИСТЕНТ НЕ ЗАПУЩЕНО
assistant-offline-hint = Налаштувати його можна не запускаючи.
btn-start = ЗАПУСТИТИ
btn-starting = ЗАПУСК...

# ### STATUS
status-disconnected = Відключено
status-standby = Очікування
status-listening = Слухаю...
status-processing = Обробка...

# ### STATS
stats-microphone = МІКРОФОН
stats-neural-networks = НЕЙРОМЕРЕЖІ
stats-resources = РЕСУРСИ
stats-system-default = Системний
stats-not-selected = Не вибрано
stats-loading = Завантаження...

# ### FOOTER
footer-tagline = JARVIS · офлайн голосовий асистент

# ### SETTINGS
settings-title = Налаштування
settings-general = Основні
settings-devices = Пристрої
settings-neural-networks = Нейромережі
settings-audio = Аудіо
settings-recognition = Розпізнавання
settings-about = Про програму
settings-language = Мова
settings-microphone = Мікрофон
settings-microphone-desc = Його буде слухати асистент.
settings-mic-default = За замовчуванням (Система)
settings-autostart-label = Автозапуск
settings-autostart-desc = Запускати JARVIS під час входу в Windows.
settings-voice = Голос асистента
settings-voice-desc =
    Не всі команди працюють з усіма звуковими пакетами.
    Натисніть, щоб прослухати як звучить голос.
settings-wake-word-engine = Рушій активації
settings-wake-word-desc = Виберіть нейромережу для розпізнавання активаційної фрази.
settings-stt-engine = Розпізнавання мовлення
settings-intent-engine = Визначення наміру
settings-intent-engine-desc = Виберіть нейромережу для розпізнавання команд.
settings-noise-suppression = Шумозаглушення
settings-noise-suppression-desc = Зменшує фоновий шум. Може негативно впливати на розпізнавання.
settings-vad = Визначення голосу (VAD)
settings-vad-desc = Пропускає тишу, економить ресурси CPU.
settings-gain-normalizer = Нормалізація гучності
settings-gain-normalizer-desc = Автоматично регулює рівень гучності.
settings-api-keys = API Ключі
settings-save = Зберегти
settings-cancel = Скасувати
settings-back = Назад
settings-enabled = Увімкнено
settings-disabled = Вимкнено

# settings - beta notice
settings-beta-title = БЕТА версія!
settings-beta-desc = Частина функцій може працювати некоректно.
settings-beta-feedback = Повідомляйте про всі знайдені баги в
settings-beta-bot = наш телеграм бот
settings-open-logs = Відкрити папку з логами

# settings - picovoice
settings-attention = Увага!
settings-picovoice-warning = Ця нейромережа працює не у всіх!
settings-picovoice-waiting = Ми чекаємо офіційного патча від розробників.
settings-picovoice-key-desc = Введіть сюди свій ключ Picovoice. Він видається безкоштовно при реєстрації в
settings-picovoice-key = Ключ Picovoice

# settings - vosk
settings-auto-detect = Авто-визначення
settings-vosk-model = Модель розпізнавання мовлення (Vosk)
settings-vosk-model-desc =
    Виберіть модель Vosk для розпізнавання мовлення.
    Ви можете завантажити моделі тут: https://alphacephei.com/vosk/models
settings-models-not-found = Моделі не знайдено
settings-models-hint = Помістіть моделі Vosk в папку resources/vosk

# settings - openai
settings-openai-key = Ключ OpenAI
settings-openai-not-supported = Наразі ChatGPT не підтримується. Він буде доданий у наступних оновленнях.

# settings - yandex ai
settings-yandex-key = Ключ Яндекс ШІ
settings-yandex-key-desc = API-ключ із Yandex AI Studio. Потрібен для запитів виду «запитай ...».
settings-yandex-key-placeholder = AQVN...
settings-yandex-model-uri = Model URI
settings-yandex-model-uri-desc = Ідентифікатор моделі, наприклад gpt://<folder-id>/yandexgpt-lite/latest
settings-yandex-api-url = URL API
settings-yandex-api-url-desc = Можна залишити за замовчуванням для стандартного endpoint Yandex Cloud.
settings-yandex-howto = Підказка: у AI Studio відкрийте «Доступ» і створіть API-ключ.
settings-yandex-use = Використання: «Джарвис, запитай яка погода в Києві».
settings-yandex-folder-id = Folder ID
settings-yandex-folder-id-desc = ID каталогу з посилання AI Studio (наприклад b1ga7hv2blpue7a5d4pb). Потрібен для озвучення відповідей.
settings-yandex-tts-voice = Голос озвучення (SpeechKit)
settings-yandex-tts-voice-desc = Ім'я голосу з «Синтез мовлення» латиницею: kirill, filipp, ermil, dmitry.

# ### COMMANDS PAGE
commands-title = Команди
commands-list-desc = Оберіть команду або додайте свою.
commands-search = Пошук команд...
commands-add-new = Додати команду
commands-new-title = Нова команда
commands-edit-title = Редагування команди
commands-new-desc = Вкажіть назву, тип дії, програму та фрази для JARVIS.
commands-name-label = Назва команди
commands-name-placeholder = Наприклад: Chrome
commands-name-required = Введіть назву команди.
commands-type-label = Тип команди
commands-type-desc = Що робитиме JARVIS після розпізнавання фрази.
commands-type-open-program = Відкрити програму
commands-type-open-website = Відкрити сайт
commands-website-url = Адреса сайту
commands-website-url-hint = Посилання відкриється у браузері за замовчуванням. Можна без https:// — додасться автоматично.
commands-website-required = Вкажіть адресу сайту.
commands-website-invalid = Некорректна адреса. Приклад: https://google.com
commands-phrase-placeholder-website = Наприклад: відкрий ютуб
commands-card-website-desc = { $site }
commands-type-dialog = Діалог
commands-dialog-user-line = Що ви скажете JARVIS
commands-dialog-user-line-hint = Фраза після «Джарвис», яку розпізнає асистент.
commands-dialog-user-line-placeholder = Наприклад: як справи
commands-dialog-jarvis-line = Що відповість JARVIS
commands-dialog-jarvis-line-hint = Ім'я свого звуку (est, th_n_n_noe), звук з паку (ok1, thanks) або повний шлях до файлу.
commands-dialog-jarvis-line-placeholder = th_n_n_noe або ok1
commands-response-sound-label = Звук відповіді JARVIS
commands-response-sound-hint = Свої звуки зберігаються в папці custom_sounds. Вкажіть ім'я файлу або натисніть «Додати звук».
commands-response-sound-folder = Папка звуків: { $path }
commands-response-sound-pick = Додати звук
commands-response-sound-placeholder = th_n_n_noe або ok1
commands-response-sound-required = Вкажіть звук відповіді або залиште ok1 за замовчуванням.
commands-dialog-user-required = Введіть фразу, яку скажете JARVIS.
commands-dialog-jarvis-required = Введіть відповідь JARVIS.
commands-card-dialog-desc = { $user } → { $jarvis }
commands-program-path = Шлях до програми
commands-program-path-hint = Ярлик (.lnk) або програма (.exe).
commands-program-pick = Обрати файл
commands-program-required = Вкажіть шлях до програми.
commands-phrase-placeholder-custom = Наприклад: відкрий хром
commands-delete = Видалити команду
commands-delete-confirm = Видалити цю команду?
commands-tap-to-edit = натисніть для редагування
commands-card-thanks = Дякую
commands-card-thanks-desc = JARVIS відповість голосом, коли ви його похвалите.
commands-card-shutdown = Вимкнення
commands-card-shutdown-desc = JARVIS завершить роботу асистента.
settings-weather-title = Налаштування погоди
commands-card-weather = Погода
commands-card-weather-desc = JARVIS розповість погоду у вибраному місті.
commands-card-weather-city = Місто: { $city }
commands-weather-desc = Фрази для погоди та місто. «Яка погода» — сьогодні, «погода завтра» — на завтра. Після зміни міста збережіть.
commands-weather-city-label = Місто
commands-weather-city-hint = Москва, Челябінськ або Якутськ — у кожного свій прогноз. Після збереження асистент одразу підхопить місто.
commands-weather-city-required = Вкажіть місто.
commands-phrase-placeholder-weather = Наприклад: яка погода
commands-type-volume-control = Керування гучністю
commands-card-volume-user-desc = Голосніше і тихіше на { $percent }%
commands-volume-phrases-required = Додайте хоча б одну фразу для голосніше або тихіше.
commands-volume-steps-title = Гучність за одну команду
commands-volume-steps-hint = Одне значення для додавання та зменшення. У Windows крок системи — 2%.
commands-volume-percent-label = Відсотків
commands-volume-preview-title = Змінюватиметься на
commands-volume-preview-up = голосніше +{ $percent }%
commands-volume-preview-down = тихіше −{ $percent }%
commands-volume-up-phrases-title = Фрази «гучніше»
commands-volume-down-phrases-title = Фрази «тихіше»
commands-volume-up-hint = Наприклад: додай звук, зроби голосніше.
commands-volume-down-hint = Наприклад: зменш звук, зроби тихіше.
commands-phrase-placeholder-volume-up = Наприклад: додай звук
commands-phrase-placeholder-volume-down = Наприклад: зменш звук
commands-thanks-desc = Додайте фрази подяки — JARVIS відповість реплікою «thanks».
commands-shutdown-desc = Фрази для вимкнення JARVIS.
commands-shutdown-hint = JARVIS завершить роботу асистента після цієї фрази.
commands-phrase-placeholder-shutdown = Наприклад: вимкнись
commands-configured = Налаштовано
commands-not-configured = Не налаштовано
commands-phrases-count = { $count } фраз
commands-back = Назад до команд
commands-phrases-title = Ваші фрази
commands-hint = Спочатку скажіть «Джарвис», потім одну з цих фраз.
commands-phrase-placeholder-thanks = Наприклад: дякую
commands-add-phrase = Додати
commands-remove-phrase = Видалити
commands-no-phrases = Додайте хоча б одну фразу.
commands-saved = Команди збережено

# ### ERRORS
error-generic = Сталася помилка
error-connection = Помилка підключення
error-not-found = Не знайдено

# ### NOTIFICATIONS
notification-saved = Налаштування збережено!
notification-error = Помилка
notification-assistant-started = Асистент запущено
notification-assistant-stopped = Асистент зупинено

# SLOTS EXTRACTION
settings-slot-engine = Витяг параметрів
settings-slot-engine-desc = Витягує параметри з голосових команд (напр. назва міста, число).
settings-gliner-model = Модель GLiNER ONNX
settings-gliner-model-desc = 
    Оберіть варіант моделі.
    Квантизовані моделі (int8, uint8) швидші, але менш точні.
settings-gliner-models-hint = Моделі GLiNER не знайдено.

# ETC
search-error-not-running = Асистент не запущено
search-error-failed = Не вдалося виконати команду
settings-no-voices = Голоси не знайдено