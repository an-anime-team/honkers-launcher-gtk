# 1.0.1

Changes from upstream:
- added creation of wine/dxvk folders at start if needed
- fixed ability to use system wine to run the game
- updated components (wine/dxvk) system
- reworked DXVKs UI components to support different builds
- fixed thread issues when calling `MainApp::update_state`
- updated core library; now launcher will continue downloading
  of partially downloaded files
- added downloading speed limiter (`config.json` -> `launcher.speed_limit`)
- added `Config::try_get_selected_dxvk_info` method;
  now launcher loads currently applied dxvk version from the wine prefix files
- added initial updates pre-downloading support (from 1.0.3 core)
- removed patch-related buttons
- changed FSR description

# 1.0.0

- updated core library
- changed default game folder name to BH3

*(0.1.0 considered as alpha and not listed here)*
