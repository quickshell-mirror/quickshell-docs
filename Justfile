clean:
	rm -rf build
	rm -rf data/modules/QuickShell
	rm -rf content/docs/types/QuickShell

typedocs: clean
	cd typegen && cargo build
	mkdir -p build/types/types
	./typegen/target/debug/typegen gentypes ../src/cpp/module.md build/types/types/QuickShell.json
	sh -c './typegen/target/debug/typegen gendocs ../src/cpp/module.md data/modules/QuickShell content/docs/types/QuickShell types/* build/types/types/*'

serve: typedocs
	hugo server --buildDrafts --disableFastRender
