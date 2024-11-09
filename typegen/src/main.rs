use std::{collections::HashMap, path::Path};

use anyhow::{anyhow, Context};

mod outform;
mod parse;
mod reformat;
mod resolver;
mod typespec;

fn main() -> anyhow::Result<()> {
	let args = std::env::args().collect::<Vec<_>>();

	match args.get(1).map(|v| v as &str) {
		Some("gentypes") => {
			let modinfo = args.get(2).expect("expected module file");
			let outpath = args.get(3).expect("expected output path");
			let path = Path::new(modinfo);
			let dir = path.parent().unwrap();
			let text = std::fs::read_to_string(path).expect("failed to read module file");
			let module = parse::parse_module(&text)?;

			let texts = module
				.header
				.headers
				.iter()
				.map(|header| {
					let text = std::fs::read_to_string(dir.join(header)).with_context(|| {
						format!(
							"failed to read module header `{header}` at {:?}",
							dir.join(header)
						)
					})?;

					Ok::<_, anyhow::Error>((header, text))
				})
				.collect::<Result<HashMap<_, _>, _>>()?;

			let qml_texts = module
				.header
				.qml_files
				.iter()
				.map(|file| {
					let text = std::fs::read_to_string(dir.join(file)).with_context(|| {
						format!(
							"failed to read module qml file `{file}` at {:?}",
							dir.join(file)
						)
					})?;

					Ok::<_, anyhow::Error>((file, text))
				})
				.collect::<Result<HashMap<_, _>, _>>()?;

			let header_parser = parse::CppParser::new();
			let qml_parser = parse::QmlParser::new();
			let mut ctx = parse::ParseContext::new(&module.header.name);

			texts
				.iter()
				.map(|(header, text)| {
					header_parser
						.parse(&text, &mut ctx)
						.with_context(|| format!("while parsing module header `{header}`"))
				})
				.collect::<Result<_, _>>()?;

			qml_texts
				.iter()
				.map(|(file, text)| {
					qml_parser
						.parse(&file, &text, &mut ctx)
						.with_context(|| format!("while parsing module qml file `{file}`"))
				})
				.collect::<Result<_, _>>()?;

			let typespec = ctx.gen_typespec(&module.header.name);

			let text = serde_json::to_string_pretty(&typespec).unwrap();

			std::fs::write(outpath, text).context("saving typespec")?;
		},
		Some("gendocs") => {
			let modinfo = args.get(2).expect("expected module file");
			let datapath = args.get(3).expect("expected datapath");
			let templatepath = args.get(4).expect("expected templatepath");

			let text = std::fs::read_to_string(modinfo).expect("failed to read module file");
			let module = parse::parse_module(&text)?;

			let mut typespec = typespec::TypeSpec::default();

			for path in &args[5..] {
				let text = std::fs::read_to_string(&path)
					.with_context(|| anyhow!("attempting to read {path}"))?;

				let ts = serde_json::from_str::<typespec::TypeSpec>(&text)
					.with_context(|| anyhow!("attempting to parse {path}"))?;

				typespec.typemap.extend(ts.typemap);
				typespec.classes.extend(ts.classes);
				typespec.gadgets.extend(ts.gadgets);
				typespec.enums.extend(ts.enums);
			}

			let types = resolver::resolve_types(&module.header.name, typespec)?;

			let datapath = Path::new(datapath);
			let templatepath = Path::new(templatepath);
			std::fs::create_dir_all(datapath)?;
			std::fs::create_dir_all(templatepath)?;

			for (name, info) in types {
				let json = serde_json::to_string_pretty(&info).unwrap();
				let datapath = datapath.join(format!("{name}.json"));
				std::fs::write(&datapath, json)
					.with_context(|| format!("while writing {datapath:?}"))?;

				let template = format!(
					"+++
title = \"{name}\"
hidetitle = true
+++

{{{{< qmltype module=\"{module}\" type=\"{name}\" >}}}}
",
					name = name,
					module = module.header.name
				);

				let templatepath = templatepath.join(format!("{name}.md"));
				std::fs::write(&templatepath, template)
					.with_context(|| format!("while writing {templatepath:?}"))?;
			}

			let index = outform::ModuleIndex {
				name: module.header.name.to_string(),
				description: module.header.description,
				details: module.details.to_string(),
			};

			let datapath = datapath.join("index.json");
			let json = serde_json::to_string_pretty(&index).unwrap();
			std::fs::write(&datapath, json)
				.with_context(|| format!("while writing {datapath:?}"))?;

			let template = format!(
				"+++
title = \"{name}\"
hidetitle = true
+++

{{{{< qmlmodule module=\"{name}\" >}}}}
",
				name = module.header.name
			);

			let templatepath = templatepath.join(format!("_index.md"));
			std::fs::write(&templatepath, template)
				.with_context(|| format!("while writing {templatepath:?}"))?;
		},
		_ => {
			panic!("typegen invoked without mode");
		},
	}

	Ok(())
}
