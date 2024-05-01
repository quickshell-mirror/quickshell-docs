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
	rm -rf data/modules/Quickshell.Service.SystemTray
	rm -rf content/docs/types/Quickshell.Service.SystemTray

buildtypegen:
	({{build_typegen}} && cd typegen && cargo build) || true

typedocs: clean buildtypegen
	mkdir -p build/types/types
	{{typegen_bin}} gentypes {{src_path}}/core/module.md build/types/types/Quickshell.json
	{{typegen_bin}} gentypes {{src_path}}/io/module.md build/types/types/Quickshell.Io.json
	{{typegen_bin}} gentypes {{src_path}}/wayland/module.md build/types/types/Quickshell.Wayland.json
	{{typegen_bin}} gentypes {{src_path}}/dbus/dbusmenu/module.md build/types/types/Quickshell.DBusMenu.json
	{{typegen_bin}} gentypes {{src_path}}/services/status_notifier/module.md build/types/types/Quickshell.Service.SystemTray.json
	sh -c '{{typegen_bin}} gendocs {{src_path}}/core/module.md data/modules/Quickshell content/docs/types/Quickshell types/* build/types/types/*'
	sh -c '{{typegen_bin}} gendocs {{src_path}}/io/module.md data/modules/Quickshell.Io content/docs/types/Quickshell.Io types/* build/types/types/*'
	sh -c '{{typegen_bin}} gendocs {{src_path}}/wayland/module.md data/modules/Quickshell.Wayland content/docs/types/Quickshell.Wayland types/* build/types/types/*'
	sh -c '{{typegen_bin}} gendocs {{src_path}}/dbus/dbusmenu/module.md data/modules/Quickshell.DBusMenu content/docs/types/Quickshell.DBusMenu types/* build/types/types/*'
	sh -c '{{typegen_bin}} gendocs {{src_path}}/services/status_notifier/module.md data/modules/Quickshell.Service.SystemTray content/docs/types/Quickshell.Service.SystemTray types/* build/types/types/*'

serve: typedocs
	hugo server --buildDrafts --disableFastRender

build: typedocs
	hugo
