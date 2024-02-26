clean:
	rm -rf build
	rm -rf data/modules/Quickshell
	rm -rf content/docs/types/Quickshell
	rm -rf data/modules/Quickshell.Wayland
	rm -rf content/docs/types/Quickshell.Wayland

typedocs: clean
	cd typegen && cargo build
	mkdir -p build/types/types
	./typegen/target/debug/typegen gentypes ../src/core/module.md build/types/types/Quickshell.json
	./typegen/target/debug/typegen gentypes ../src/wayland/module.md build/types/types/Quickshell.Wayland.json
	sh -c './typegen/target/debug/typegen gendocs ../src/core/module.md data/modules/Quickshell content/docs/types/Quickshell types/* build/types/types/*'
	sh -c './typegen/target/debug/typegen gendocs ../src/wayland/module.md data/modules/Quickshell.Wayland content/docs/types/Quickshell.Wayland types/* build/types/types/*'

serve: typedocs
	hugo server --buildDrafts --disableFastRender
