# APP INFO
app-name = JARVIS
app-description = Голосовой ассистент

# TRAY MENU
tray-restart = Перезапустить
tray-settings = Настройки
tray-exit = Выход
tray-tooltip = JARVIS - Голосовой ассистент
tray-language = Язык
tray-voice = Голос
tray-wake-word = Движок wake-word
tray-noise-suppression = Шумоподавление
tray-vad = Детекция голоса (VAD)
tray-gain-normalizer = Нормализация громкости

# HEADER
header-commands = КОМАНДЫ
header-settings = НАСТРОЙКИ

# UPDATES
update-available = Доступно обновление { $version }
update-install = Обновить
update-later = Позже
update-installing = Установка обновления…
update-error = Ошибка обновления

# SEARCH
search-placeholder = Введите команду вручную или произнесите «Джарвис» ...

# MAIN PAGE
assist-search-label = Быстрая команда
assist-running-hint = Скажите «Джарвис» или введите команду выше.
assistant-not-running = Ассистент не запущен
assistant-offline-hint = Можно настроить команды и микрофон, не запуская ассистента.
btn-start = Запустить ассистента
btn-starting = Запуск...

# STATUS
status-disconnected = Отключен
status-standby = Ожидание
status-listening = Слушаю...
status-processing = Обработка...

# STATS
stats-microphone = МИКРОФОН
stats-neural-networks = НЕЙРОСЕТИ
stats-resources = РЕСУРСЫ
stats-system-default = Системный
stats-not-selected = Не выбран
stats-loading = Загрузка...

# FOOTER
footer-tagline = JARVIS · офлайн голосовой ассистент

# SETTINGS
settings-title = Настройки
settings-general = Основные
settings-devices = Устройства
settings-neural-networks = Нейросети
settings-audio = Аудио
settings-recognition = Распознавание
settings-about = О программе
settings-language = Язык
settings-language-interface = Язык интерфейса
settings-language-desc = Меню, кнопки и уведомления программы.
settings-language-restart-note = После смены языка программа перезапустится автоматически.
settings-tab-weather = Погода
settings-microphone = Микрофон
settings-microphone-desc = Его будет слушать ассистент.
settings-mic-default = По умолчанию (Система)
settings-autostart-label = Автозапуск
settings-autostart-desc = Запускать JARVIS при входе в Windows.
settings-voice = Голос ассистента
settings-voice-desc =
    Не все команды работают со всеми звуковыми пакетами.
    Кликните, чтобы прослушать как звучит голос.
settings-wake-word-engine = Движок активации
settings-wake-word-desc = Выберите нейросеть для распознавания активационной фразы.
settings-stt-engine = Распознавание речи
settings-intent-engine = Определение намерения
settings-intent-engine-desc = Выберите нейросеть для распознавания команд.
settings-noise-suppression = Шумоподавление
settings-noise-suppression-desc = Уменьшает фоновый шум. Может негативно влиять на распознавание.
settings-vad = Определение голоса (VAD)
settings-vad-desc = Пропускает тишину, экономит ресурсы CPU.
settings-gain-normalizer = Нормализация громкости
settings-gain-normalizer-desc = Автоматически регулирует уровень громкости.
settings-api-keys = API Ключи
settings-save = Сохранить
settings-cancel = Отмена
settings-back = Назад
settings-enabled = Включено
settings-disabled = Отключено

# settings - beta notice
settings-beta-title = БЕТА версия!
settings-beta-desc = Часть функций может работать некорректно.
settings-beta-feedback = Сообщайте обо всех найденных багах в
settings-beta-bot = наш телеграм бот
settings-open-logs = Открыть папку с логами

# settings - picovoice
settings-attention = Внимание!
settings-picovoice-warning = Эта нейросеть работает не у всех!
settings-picovoice-waiting = Мы ждем официального патча от разработчиков.
settings-picovoice-key-desc = Введите сюда свой ключ Picovoice. Он выдается бесплатно при регистрации в
settings-picovoice-key = Ключ Picovoice

# settings - vosk
settings-auto-detect = Авто-определение
settings-vosk-model = Модель распознавания речи (Vosk)
settings-vosk-model-desc =
    Выберите модель Vosk для распознавания речи.
    Вы можете скачать модели здесь: https://alphacephei.com/vosk/models
settings-models-not-found = Модели не найдены
settings-models-hint = Поместите модели Vosk в папку resources/vosk

# settings - openai
settings-openai-key = Ключ OpenAI
settings-openai-not-supported = В данный момент ChatGPT не поддерживается. Он будет добавлен в ближайших обновлениях.

# COMMANDS PAGE
commands-title = Команды
commands-list-desc = Выберите команду или добавьте свою.
commands-search = Поиск команд...
commands-add-new = Добавить команду
commands-new-title = Новая команда
commands-edit-title = Редактирование команды
commands-new-desc = Укажите название, программу и фразы для JARVIS.
commands-new-desc-pick = Сначала выберите тип команды.
commands-type-placeholder = Выберите
commands-type-required = Выберите тип команды.
commands-name-label = Название команды
commands-name-placeholder = Например: Chrome
commands-name-required = Введите название команды.
commands-type-label = Тип команды
commands-type-desc = Что будет делать JARVIS при распознавании фразы.
commands-type-open-program = Открыть программу
commands-type-open-website = Открыть сайт
commands-type-volume-control = Управление громкостью
commands-card-volume-user-desc = Громче и тише на { $percent }%
commands-volume-phrases-required = Добавьте хотя бы одну фразу для громче или тише.
commands-website-url = Адрес сайта
commands-website-url-hint = Ссылка откроется в браузере по умолчанию. Можно без https:// — добавится автоматически.
commands-website-required = Укажите адрес сайта.
commands-website-invalid = Некорректный адрес. Пример: https://yandex.ru
commands-phrase-placeholder-website = Например: открой яндекс
commands-card-website-desc = { $site }
commands-type-dialog = Диалог
commands-dialog-user-line = Что вы скажете JARVIS
commands-dialog-user-line-hint = Фраза после «Джарвис», которую распознает ассистент.
commands-dialog-user-line-placeholder = Например: как дела
commands-dialog-jarvis-line = Что ответит JARVIS
commands-dialog-jarvis-line-hint = Имя своего звука (est, th_n_n_noe), звук из пакета (ok1, thanks) или полный путь к файлу.
commands-dialog-jarvis-line-placeholder = th_n_n_noe или ok1
commands-response-sound-label = Звук ответа JARVIS
commands-response-sound-hint = Свои звуки хранятся в папке custom_sounds. Укажите имя файла без пути или нажмите «Добавить звук».
commands-response-sound-folder = Папка звуков: { $path }
commands-response-sound-pick = Добавить звук
commands-response-sound-placeholder = th_n_n_noe или ok1
commands-response-sound-required = Укажите звук ответа или оставьте ok1 по умолчанию.
commands-dialog-user-required = Введите фразу, которую скажете JARVIS.
commands-dialog-jarvis-required = Введите ответ JARVIS.
commands-card-dialog-desc = { $user } → { $jarvis }
commands-program-path = Путь к программе
commands-program-path-hint = Ярлык (.lnk) или программа (.exe).
commands-program-pick = Выбрать файл
commands-program-required = Укажите путь к программе.
commands-phrase-placeholder-custom = Например: открой хром
commands-delete = Удалить команду
commands-delete-confirm = Удалить эту команду?
commands-tap-to-edit = нажмите для редактирования
commands-card-thanks = Спасибо
commands-card-thanks-desc = JARVIS ответит голосом, когда вы его похвалите.
commands-card-shutdown = Выключение
commands-card-shutdown-desc = JARVIS завершит работу ассистента.
commands-card-weather = Погода
commands-card-weather-desc = JARVIS расскажет погоду в выбранном городе.
commands-card-weather-city = Город: { $city }
commands-volume-steps-title = Громкость за одну команду
commands-volume-steps-hint = Одно значение для прибавления и убавления. В Windows шаг системы — 2%.
commands-volume-percent-label = Процентов
commands-volume-preview-title = Будет меняться на
commands-volume-preview-up = громче +{ $percent }%
commands-volume-preview-down = тише −{ $percent }%
commands-volume-up-phrases-title = Фразы «громче»
commands-volume-down-phrases-title = Фразы «тише»
commands-volume-up-hint = Например: прибавь звук, сделай громче.
commands-volume-down-hint = Например: убавь звук, сделай потише.
commands-phrase-placeholder-volume-up = Например: прибавь звук
commands-phrase-placeholder-volume-down = Например: убавь звук
commands-weather-desc = Фразы для погоды и город. «Какая погода» — сейчас, «погода завтра» — на завтра. После смены города нажмите «Сохранить».
settings-weather-title = Настройки погоды
commands-weather-city-label = Город
commands-weather-city-hint = Москва, Челябинск или Якутск — у каждого свой прогноз. После сохранения перезапуск не нужен: ассистент подхватит город сразу.
commands-weather-city-placeholder = Москва
commands-weather-city-required = Укажите город.
commands-phrase-placeholder-weather = Например: какая погода
commands-thanks-desc = Добавьте фразы благодарности — JARVIS ответит репликой «thanks».
commands-shutdown-desc = Фразы для выключения JARVIS.
commands-shutdown-hint = JARVIS завершит работу ассистента после этой фразы.
commands-phrase-placeholder-shutdown = Например: выключись
commands-configured = Настроено
commands-not-configured = Не настроено
commands-builtin = Встроенная
commands-card-greeting = Приветствие
commands-card-greeting-desc = JARVIS ответит одной из фраз приветствия.
commands-greeting-desc = Добавьте фразы приветствия — JARVIS ответит случайной репликой.
commands-greeting-hint = Скажите «Джарвис», затем одну из этих фраз.
commands-phrase-placeholder-greeting = Например: привет
commands-phrases-count = { $count } фраз
commands-back = Назад к командам
commands-phrases-title = Ваши фразы
commands-hint = Сначала скажите «Джарвис», затем одну из этих фраз.
commands-phrase-placeholder-thanks = Например: спасибо
commands-add-phrase = Добавить
commands-remove-phrase = Удалить
commands-no-phrases = Добавьте хотя бы одну фразу.
commands-phrases-optional = Необязательно. Можно оставить пустым — ИИ сам поймёт разные формулировки (например «открой ютубчик»).
commands-saved = Команды сохранены

# ERRORS
error-generic = Произошла ошибка
error-connection = Ошибка подключения
error-not-found = Не найдено

# NOTIFICATIONS
notification-saved = Настройки сохранены!
notification-error = Ошибка
notification-assistant-started = Ассистент запущен
notification-assistant-stopped = Ассистент остановлен

# SLOTS EXTRACTION
settings-slot-engine = Извлечение параметров
settings-slot-engine-desc = Извлекает параметры из голосовых команд (напр. название города, число).
settings-gliner-model = Модель GLiNER ONNX
settings-gliner-model-desc =
    Выберите вариант модели.
    Квантизированные модели (int8, uint8) быстрее, но менее точны.
settings-gliner-models-hint = Модели GLiNER не найдены.

# ETC
search-error-not-running = Ассистент не запущен
search-error-failed = Не удалось выполнить команду
settings-no-voices = Голоса не найдены