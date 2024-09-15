typegen_bin := env_var_or_default('TYPEGEN', './typegen/target/debug/typegen')
src_path := env_var_or_default('SRC_PATH', '../src')

build_typegen := if typegen_bin == './typegen/target/debug/typegen' { "true" } else { "false" }

clean:
	rm -rf public
	rm -rf build
	rm -rf data/modules/Quickshell
	rm -rf content/docs/types/Quickshell
	rm -rf data/modules/Quickshell.Io
	rm -rf content/docs/types/Quickshell.Io
	rm -rf data/modules/Quickshell.Wayland
	rm -rf content/docs/types/Quickshell.Wayland
	rm -rf data/modules/Quickshell.DBusMenu
	rm -rf content/docs/types/Quickshell.DBusMenu
	rm -rf data/modules/Quickshell.Services.SystemTray
	rm -rf content/docs/types/Quickshell.Services.SystemTray
	rm -rf data/modules/Quickshell.Services.Pipewire
	rm -rf content/docs/types/Quickshell.Services.Pipewire
	rm -rf data/modules/Quickshell.Services.Mpris
	rm -rf content/docs/types/Quickshell.Services.Mpris
	rm -rf data/modules/Quickshell.Services.Pam
	rm -rf content/docs/types/Quickshell.Services.Pam
	rm -rf data/modules/Quickshell.Services.Greetd
	rm -rf content/docs/types/Quickshell.Services.Greetd
	rm -rf data/modules/Quickshell.Services.UPower
	rm -rf content/docs/types/Quickshell.Services.UPower
	rm -rf data/modules/Quickshell.Services.Notifications
	rm -rf content/docs/types/Quickshell.Services.Notifications
	rm -rf data/modules/Quickshell.Hyprland
	rm -rf content/docs/types/Quickshell.Hyprland
	rm -rf data/modules/Quickshell.Widgets
	rm -rf content/docs/types/Quickshell.Widgets

buildtypegen:
	({{build_typegen}} && cd typegen && cargo build) || true

typedocs: clean buildtypegen
	mkdir -p build/types/types
	{{typegen_bin}} gentypes {{src_path}}/core/module.md build/types/types/Quickshell.json
	{{typegen_bin}} gentypes {{src_path}}/io/module.md build/types/types/Quickshell.Io.json
	{{typegen_bin}} gentypes {{src_path}}/wayland/module.md build/types/types/Quickshell.Wayland.json
	{{typegen_bin}} gentypes {{src_path}}/dbus/dbusmenu/module.md build/types/types/Quickshell.DBusMenu.json
	{{typegen_bin}} gentypes {{src_path}}/services/status_notifier/module.md build/types/types/Quickshell.Services.SystemTray.json
	{{typegen_bin}} gentypes {{src_path}}/services/pipewire/module.md build/types/types/Quickshell.Services.Pipewire.json
	{{typegen_bin}} gentypes {{src_path}}/services/mpris/module.md build/types/types/Quickshell.Services.Mpris.json
	{{typegen_bin}} gentypes {{src_path}}/services/pam/module.md build/types/types/Quickshell.Services.Pam.json
	{{typegen_bin}} gentypes {{src_path}}/services/greetd/module.md build/types/types/Quickshell.Services.Greetd.json
	{{typegen_bin}} gentypes {{src_path}}/services/upower/module.md build/types/types/Quickshell.Services.UPower.json
	{{typegen_bin}} gentypes {{src_path}}/services/notifications/module.md build/types/types/Quickshell.Services.Notifications.json
	{{typegen_bin}} gentypes {{src_path}}/wayland/hyprland/module.md build/types/types/Quickshell.Hyprland.json
	{{typegen_bin}} gentypes {{src_path}}/widgets/module.md build/types/types/Quickshell.Widgets.json
	sh -c '{{typegen_bin}} gendocs {{src_path}}/core/module.md data/modules/Quickshell content/docs/types/Quickshell types/* build/types/types/*'
	sh -c '{{typegen_bin}} gendocs {{src_path}}/io/module.md data/modules/Quickshell.Io content/docs/types/Quickshell.Io types/* build/types/types/*'
	sh -c '{{typegen_bin}} gendocs {{src_path}}/wayland/module.md data/modules/Quickshell.Wayland content/docs/types/Quickshell.Wayland types/* build/types/types/*'
	sh -c '{{typegen_bin}} gendocs {{src_path}}/dbus/dbusmenu/module.md data/modules/Quickshell.DBusMenu content/docs/types/Quickshell.DBusMenu types/* build/types/types/*'
	sh -c '{{typegen_bin}} gendocs {{src_path}}/services/status_notifier/module.md data/modules/Quickshell.Services.SystemTray content/docs/types/Quickshell.Services.SystemTray types/* build/types/types/*'
	sh -c '{{typegen_bin}} gendocs {{src_path}}/services/pipewire/module.md data/modules/Quickshell.Services.Pipewire content/docs/types/Quickshell.Services.Pipewire types/* build/types/types/*'
	sh -c '{{typegen_bin}} gendocs {{src_path}}/services/mpris/module.md data/modules/Quickshell.Services.Mpris content/docs/types/Quickshell.Services.Mpris types/* build/types/types/*'
	sh -c '{{typegen_bin}} gendocs {{src_path}}/services/pam/module.md data/modules/Quickshell.Services.Pam content/docs/types/Quickshell.Services.Pam types/* build/types/types/*'
	sh -c '{{typegen_bin}} gendocs {{src_path}}/services/greetd/module.md data/modules/Quickshell.Services.Greetd content/docs/types/Quickshell.Services.Greetd types/* build/types/types/*'
	sh -c '{{typegen_bin}} gendocs {{src_path}}/services/upower/module.md data/modules/Quickshell.Services.UPower content/docs/types/Quickshell.Services.UPower types/* build/types/types/*'
	sh -c '{{typegen_bin}} gendocs {{src_path}}/services/notifications/module.md data/modules/Quickshell.Services.Notifications content/docs/types/Quickshell.Services.Notifications types/* build/types/types/*'
	sh -c '{{typegen_bin}} gendocs {{src_path}}/wayland/hyprland/module.md data/modules/Quickshell.Hyprland content/docs/types/Quickshell.Hyprland types/* build/types/types/*'
	sh -c '{{typegen_bin}} gendocs {{src_path}}/widgets/module.md data/modules/Quickshell.Widgets content/docs/types/Quickshell.Widgets types/* build/types/types/*'

serve: typedocs
	hugo server --buildDrafts --disableFastRender

build: typedocs
	hugo
