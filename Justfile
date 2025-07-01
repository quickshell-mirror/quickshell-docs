typegen_bin := env_var_or_default('TYPEGEN', './typegen/target/debug/typegen')
src_path := env_var_or_default('SRC_PATH', '../quickshell/src')

build_typegen := if typegen_bin == './typegen/target/debug/typegen' { "true" } else { "false" }

clean:
	rm -rf public
	rm -rf build
	rm -rf data/modules/*
	rm -rf content/docs/types/*

buildtypegen:
	({{build_typegen}} && cd typegen && cargo build) || true

typedocs: clean buildtypegen
	mkdir -p data/modules
	mkdir -p build/types/types
	mkdir -p content/docs/types
	{{typegen_bin}} fulltypegen {{src_path}} build/types/types data/modules content/docs/types types

serve: typedocs
	hugo server --buildDrafts --disableFastRender

build: typedocs
	hugo
