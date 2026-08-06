# How to run on iOS

1. open `mobile/benilla.xcodeproj` inside `XCode`
2. On files tab inside left sidebar click on `benilla` (first item)
3. Click `Signing & Capabilities`
4. Change your `Team` and `Bundle Identifier`
5. Click "Run" (both device & simulator work)
6. launch app (it will probably crash as no Data files exist at this stage, but it also create config example)
7. put `Data` folder on your device (open Files, navigate to `benilla` put it there)
   Data is inside `WoW/Data`
   _folder names are case-sensetive_
8. put `assets` folder to your device (same as with `Data`)
   Data is inside `crates/benilla-app/assets`
9. edit `config.json` inside `benilla` app dir (change server address, username and password)

# Joystick

LICENSE: https://github.com/SergioRibera/virtual_joystick/blob/main/LICENSE-MIT
