use std::{collections::HashMap, path::Path};

use anyhow::{anyhow, Context};

mod outform;
mod parse;
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

			let parser = parse::Parser::new();
			let mut ctx = parse::ParseContext::new(&module.header.name);

			texts
				.iter()
				.map(|(header, text)| {
					parser
						.parse(&text, &mut ctx)
						.with_context(|| format!("while parsing module header `{header}`"))
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
