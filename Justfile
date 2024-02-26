clean:
	rm -rf build
	rm -rf data/modules/QuickShell
	rm -rf content/docs/types/QuickShell
	rm -rf data/modules/QuickShell.Wayland
	rm -rf content/docs/types/QuickShell.Wayland

typedocs: clean
	cd typegen && cargo build
	mkdir -p build/types/types
	./typegen/target/debug/typegen gentypes ../src/core/module.md build/types/types/QuickShell.json
	./typegen/target/debug/typegen gentypes ../src/wayland/module.md build/types/types/QuickShell.Wayland.json
	sh -c './typegen/target/debug/typegen gendocs ../src/core/module.md data/modules/QuickShell content/docs/types/QuickShell types/* build/types/types/*'
	sh -c './typegen/target/debug/typegen gendocs ../src/wayland/module.md data/modules/QuickShell.Wayland content/docs/types/QuickShell.Wayland types/* build/types/types/*'

serve: typedocs
	hugo server --buildDrafts --disableFastRender
