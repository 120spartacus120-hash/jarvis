# ### APP INFO
app-name = JARVIS
app-description = Voice Assistant

# ### TRAY MENU
tray-restart = Restart
tray-settings = Settings
tray-exit = Exit
tray-tooltip = JARVIS - Voice Assistant
tray-language = Language
tray-voice = Voice
tray-wake-word = Wake Word Engine
tray-noise-suppression = Noise Suppression
tray-vad = Voice Activity Detection
tray-gain-normalizer = Gain Normalizer

# ### HEADER
header-commands = COMMANDS
header-settings = SETTINGS

# ### UPDATES
update-available = Update available: { $version }
update-install = Update
update-later = Later
update-downloading = Downloading… { $percent }%
update-error = Update failed

# ### SEARCH
search-placeholder = Enter a command manually or say «Jarvis» ...

# ### MAIN PAGE
assistant-not-running = ASSISTANT NOT RUNNING
assistant-offline-hint = You can configure it without starting.
btn-start = START
btn-starting = STARTING...

# ### STATUS
status-disconnected = Disconnected
status-standby = Standby
status-listening = Listening...
status-processing = Processing...

# ### STATS
stats-microphone = MICROPHONE
stats-neural-networks = NEURAL NETWORKS
stats-resources = RESOURCES
stats-system-default = System Default
stats-not-selected = Not selected
stats-loading = Loading...

# ### FOOTER
footer-tagline = JARVIS · offline voice assistant

# ### SETTINGS
settings-title = Settings
settings-general = General
settings-devices = Devices
settings-neural-networks = Neural Networks
settings-audio = Audio
settings-recognition = Recognition
settings-about = About
settings-language = Language
settings-microphone = Microphone
settings-microphone-desc = The assistant will listen to this microphone.
settings-mic-default = Default (System)
settings-autostart-label = Autostart
settings-autostart-desc = Launch JARVIS when Windows starts.
settings-voice = Assistant voice
settings-voice-desc =
    Not all commands work with all sound packs.
    Click to listen the preview of sound.
settings-wake-word-engine = Wake word engine
settings-wake-word-desc = Choose the engine for wake word recognition.
settings-stt-engine = Speech recognition
settings-intent-engine = Intent recognition
settings-intent-engine-desc = Select neural network for command recognition.
settings-noise-suppression = Noise suppression
settings-noise-suppression-desc = Reduces background noise. May negatively affect recognition.
settings-vad = Voice detection (VAD)
settings-vad-desc = Skips silence, saves CPU resources.
settings-gain-normalizer = Gain normalizer
settings-gain-normalizer-desc = Automatically adjusts volume level.
settings-api-keys = API Keys
settings-save = Save
settings-cancel = Cancel
settings-back = Back
settings-enabled = Enabled
settings-disabled = Disabled

# settings - beta notice
settings-beta-title = BETA version!
settings-beta-desc = Some features may not work correctly.
settings-beta-feedback = Report all bugs to
settings-beta-bot = our Telegram bot
settings-open-logs = Open logs folder

# settings - picovoice
settings-attention = Attention!
settings-picovoice-warning = This neural network doesn't work for everyone!
settings-picovoice-waiting = We are waiting for an official patch from the developers.
settings-picovoice-key-desc = Enter your Picovoice key here. It is issued for free upon registration at
settings-picovoice-key = Picovoice Key

# settings - vosk
settings-auto-detect = Auto-detect
settings-vosk-model = Speech recognition model (Vosk)
settings-vosk-model-desc =
    Select Vosk model for speech recognition.
    You can download models here: https://alphacephei.com/vosk/models
settings-models-not-found = Models not found
settings-models-hint = Place Vosk models in resources/vosk folder

# settings - openai
settings-openai-key = OpenAI Key
settings-openai-not-supported = ChatGPT is not currently supported. It will be added in future updates.

# ### COMMANDS PAGE
commands-title = Commands
commands-list-desc = Choose a command or add your own.
commands-search = Search commands...
commands-add-new = Add command
commands-new-title = New command
commands-edit-title = Edit command
commands-new-desc = Set a name, action type, program path, and voice phrases.
commands-name-label = Command name
commands-name-placeholder = For example: Chrome
commands-name-required = Enter a command name.
commands-type-label = Command type
commands-type-desc = What JARVIS will do when it hears a phrase.
commands-type-open-program = Open program
commands-type-open-website = Open website
commands-type-volume-control = Volume control
commands-card-volume-user-desc = Louder and quieter by { $percent }%
commands-volume-phrases-required = Add at least one phrase for louder or quieter.
commands-website-url = Website URL
commands-website-url-hint = Opens in your default browser. You can omit https:// — it will be added automatically.
commands-website-required = Enter a website URL.
commands-website-invalid = Invalid URL. Example: https://google.com
commands-phrase-placeholder-website = For example: open youtube
commands-card-website-desc = { $site }
commands-type-dialog = Dialog
commands-dialog-user-line = What you say to JARVIS
commands-dialog-user-line-hint = The phrase JARVIS listens for after the wake word.
commands-dialog-user-line-placeholder = e.g. how are you
commands-dialog-jarvis-line = What JARVIS replies
commands-dialog-jarvis-line-hint = Custom sound name (est, th_n_n_noe), voice pack sound (ok1, thanks), or full file path.
commands-dialog-jarvis-line-placeholder = th_n_n_noe or ok1
commands-response-sound-label = JARVIS reply sound
commands-response-sound-hint = Custom sounds live in the custom_sounds folder. Enter the file name or tap Add sound.
commands-response-sound-folder = Sounds folder: { $path }
commands-response-sound-pick = Add sound
commands-response-sound-placeholder = th_n_n_noe or ok1
commands-response-sound-required = Enter a reply sound or leave ok1 as default.
commands-dialog-user-required = Enter the phrase you will say to JARVIS.
commands-dialog-jarvis-required = Enter JARVIS reply.
commands-card-dialog-desc = { $user } → { $jarvis }
commands-program-path = Program path
commands-program-path-hint = Shortcut (.lnk) or program (.exe).
commands-program-pick = Choose file
commands-program-required = Set the program path.
commands-phrase-placeholder-custom = For example: open chrome
commands-delete = Delete command
commands-delete-confirm = Delete this command?
commands-tap-to-edit = tap to edit
commands-card-thanks = Thanks
commands-card-thanks-desc = JARVIS replies with a voice line when you praise him.
commands-card-shutdown = Shutdown
commands-card-shutdown-desc = JARVIS stops the assistant.
commands-card-weather = Weather
commands-card-weather-desc = JARVIS reports the weather for your city.
commands-card-weather-city = City: { $city }
commands-volume-steps-title = Volume per command
commands-volume-steps-hint = One value for both louder and quieter. Windows system step is 2%.
commands-volume-percent-label = Percent
commands-volume-preview-title = Will change by
commands-volume-preview-up = louder +{ $percent }%
commands-volume-preview-down = quieter −{ $percent }%
commands-volume-up-phrases-title = “Louder” phrases
commands-volume-down-phrases-title = “Quieter” phrases
commands-volume-up-hint = For example: turn up the volume, make it louder.
commands-volume-down-hint = For example: turn down the volume, make it quieter.
commands-phrase-placeholder-volume-up = For example: turn up the volume
commands-phrase-placeholder-volume-down = For example: turn down the volume
commands-weather-desc = Weather phrases and city. “What's the weather” is today; add “weather tomorrow” for the next day. Save after changing the city.
settings-weather-title = Weather settings
commands-weather-city-label = City
commands-weather-city-hint = Moscow, Chelyabinsk, or Yakutsk — each has its own forecast. Save to apply without restarting the assistant.
commands-weather-city-placeholder = Moscow
commands-weather-city-required = Enter a city.
commands-phrase-placeholder-weather = For example: what's the weather
commands-thanks-desc = Add thank-you phrases — JARVIS will play a "thanks" voice line.
commands-shutdown-desc = Phrases to shut down JARVIS.
commands-shutdown-hint = JARVIS will stop the assistant after this phrase.
commands-phrase-placeholder-shutdown = For example: shut down
commands-configured = Configured
commands-not-configured = Not configured
commands-phrases-count = { $count } phrases
commands-back = Back to commands
commands-phrases-title = Your phrases
commands-hint = Say "Jarvis" first, then one of these phrases.
commands-phrase-placeholder-thanks = For example: thank you
commands-add-phrase = Add
commands-remove-phrase = Remove
commands-no-phrases = Add at least one phrase.
commands-saved = Commands saved

# ### ERRORS
error-generic = An error occurred
error-connection = Connection error
error-not-found = Not found

# ### NOTIFICATIONS
notification-saved = Settings saved!
notification-error = Error
notification-assistant-started = Assistant started
notification-assistant-stopped = Assistant stopped

# SLOTS EXTRACTION
settings-slot-engine = Slot extraction
settings-slot-engine-desc = Extract parameters from voice commands (e.g. city name, number).
settings-gliner-model = GLiNER ONNX model
settings-gliner-model-desc =
    Select model variant.
    Smaller quantized models (int8, uint8) are faster but less accurate.
settings-gliner-models-hint = No GLiNER models found.

# ETC
search-error-not-running = Assistant is not running
search-error-failed = Failed to execute command
settings-no-voices = No voices found