use std::borrow::Cow;

use anyhow::{anyhow, bail, Context};
use regex::Regex;
use serde::Deserialize;

use crate::typespec;

#[derive(Deserialize, Debug)]
pub struct ModuleInfoHeader {
	pub name: String,
	pub description: String,
	pub headers: Vec<String>,
}

#[derive(Debug)]
pub struct ModuleInfo<'a> {
	pub header: ModuleInfoHeader,
	pub details: &'a str,
}

pub fn parse_module(text: &str) -> anyhow::Result<ModuleInfo> {
	let Some((mut header, mut details)) = text.split_once("-----") else {
		bail!("could not split module header");
	};

	header = header.trim();
	details = details.trim();

	let header = toml::from_str::<ModuleInfoHeader>(header)
    .context("parsing module info header")?;

	Ok(ModuleInfo {
		header,
		details,
	})
}

#[derive(Debug)]
pub struct ClassInfo<'a> {
	pub type_: ClassType,
	pub name: &'a str,
	pub qml_name: Option<&'a str>,
	pub superclass: Option<&'a str>,
	pub singleton: bool,
	pub uncreatable: bool,
	pub comment: Option<&'a str>,
	pub properties: Vec<Property<'a>>,
	pub invokables: Vec<Invokable<'a>>,
}

#[derive(Debug)]
pub enum ClassType {
	Object,
	Gadget,
}

#[derive(Debug, Clone, Copy)]
pub struct Property<'a> {
	pub type_: &'a str,
	pub name: &'a str,
	pub comment: Option<&'a str>,
	pub readable: bool,
	pub writable: bool,
	pub default: bool,
}

#[derive(Debug, Clone)]
pub struct Invokable<'a> {
	pub name: &'a str,
	pub ret: &'a str,
	pub comment: Option<&'a str>,
	pub params: Vec<InvokableParam<'a>>,
}

#[derive(Debug, Clone, Copy)]
pub struct InvokableParam<'a> {
	pub name: &'a str,
	pub type_: &'a str,
}

#[derive(Debug)]
pub struct EnumInfo<'a> {
	pub namespace: &'a str,
	pub enum_name: &'a str,
	pub qml_name: &'a str,
	pub comment: Option<&'a str>,
	pub variants: Vec<Variant<'a>>,
}

#[derive(Debug, Clone, Copy)]
pub struct Variant<'a> {
	pub name: &'a str,
	pub comment: Option<&'a str>,
}

pub struct Parser {
	pub class_regex: Regex,
	pub macro_regex: Regex,
	pub property_regex: Regex,
	pub fn_regex: Regex,
	pub fn_param_regex: Regex,
	pub defaultprop_classinfo_regex: Regex,
	pub enum_regex: Regex,
	pub enum_variant_regex: Regex,
}

#[derive(Debug)]
pub struct ParseContext<'a> {
	pub classes: Vec<ClassInfo<'a>>,
	pub enums: Vec<EnumInfo<'a>>,
}

impl Default for ParseContext<'_> {
	fn default() -> Self {
		Self {
			classes: Vec::new(),
			enums: Vec::new(),
		}
	}
}

impl Parser {
	pub fn new() -> Self {
		Self {
			class_regex: Regex::new(r#"(?<comment>(\s*\/\/\/.*\n)+)?\s*class\s+(?<name>\w+)(?:\s*:\s*public\s+(?<super>\w+))?.+?\{(?<body>[\s\S]*?)};"#).unwrap(),
			macro_regex: Regex::new(r#"(?<comment>(\s*\/\/\/.*\n)+)?\s*(?<type>(Q|QML)_\w+)\s*(\(\s*(?<args>.*)\s*\))?;"#).unwrap(),
			property_regex: Regex::new(r#"^\s*(?<type>(\w|::|<|>)+\*?)\s+(?<name>\w+)(\s+(MEMBER\s+(?<member>\w+)|READ\s+(?<read>\w+)|WRITE\s+(?<write>\w+)|NOTIFY\s+(?<notify>\w+)|(?<const>CONSTANT)))+\s*$"#).unwrap(),
			fn_regex: Regex::new(r#"(?<comment>(\s*\/\/\/.*\n)+)?\s*Q_INVOKABLE\s+(?<type>(\w|::|<|>)+\*?)\s+(?<name>\w+)\((?<params>[\s\S]*?)\);"#).unwrap(),
			fn_param_regex: Regex::new(r#"(?<type>(\w|::|<|>)+\*?)\s+(?<name>\w+)(,|$)"#).unwrap(),
			defaultprop_classinfo_regex: Regex::new(r#"^\s*"DefaultProperty", "(?<prop>.+)"\s*$"#).unwrap(),
			enum_regex: Regex::new(r#"(?<comment>(\s*\/\/\/.*\n)+)?\s*namespace (?<namespace>\w+)\s*\{[\s\S]*?(QML_ELEMENT|QML_NAMED_ELEMENT\((?<qml_name>\w+)\));[\s\S]*?enum\s*(?<enum_name>\w+)\s*\{(?<body>[\s\S]*?)\};[\s\S]*?\}"#).unwrap(),
			enum_variant_regex: Regex::new(r#"(?<comment>(\s*\/\/\/.*\n)+)?\s*(?<name>\w+)\s*=\s*\d+,"#).unwrap(),
		}
	}

	pub fn parse_classes<'a>(&self, text: &'a str, ctx: &mut ParseContext<'a>) -> anyhow::Result<()> {
		for class in self.class_regex.captures_iter(text) {
			let comment = class.name("comment").map(|m| m.as_str());
			let name = class.name("name").unwrap().as_str();
			let superclass = class.name("super").map(|m| m.as_str());
			let body = class.name("body").unwrap().as_str();

			let mut classtype = None;
			let mut qml_name = None;
			let mut singleton = false;
			let mut uncreatable = false;
			let mut properties = Vec::new();
			let mut default_property = None;
			let mut invokables = Vec::new();

			(|| {
				for macro_ in self.macro_regex.captures_iter(body) {
					let comment = macro_.name("comment").map(|m| m.as_str());
					let type_ = macro_.name("type").unwrap().as_str();
					let args = macro_.name("args").map(|m| m.as_str());

					(|| {
						match type_ {
							"Q_OBJECT" => classtype = Some(ClassType::Object),
							"Q_GADGET" => classtype = Some(ClassType::Gadget),
							"QML_ELEMENT" => qml_name = Some(name),
							"QML_NAMED_ELEMENT" => qml_name = Some(args.ok_or_else(|| anyhow!("expected name for QML_NAMED_ELEMENT"))?),
							"QML_SINGLETON" => singleton = true,
							"QML_UNCREATABLE" => uncreatable = true,
							"Q_PROPERTY" => {
								let prop = self.property_regex.captures(args.ok_or_else(|| anyhow!("expected args for Q_PROPERTY"))?)
									.ok_or_else(|| anyhow!("unable to parse Q_PROPERTY"))?;

								let member = prop.name("member").is_some();
								let read = prop.name("read").is_some();
								let write = prop.name("write").is_some();
								let constant = prop.name("const").is_some();

								properties.push(Property {
									type_: prop.name("type").unwrap().as_str(),
									name: prop.name("name").unwrap().as_str(),
									comment,
									readable: read || member,
									writable: !constant && (write || member),
									default: false,
								});
							},
							"Q_CLASSINFO" => {
								let classinfo = self.defaultprop_classinfo_regex.captures(args.ok_or_else(|| anyhow!("expected args for Q_CLASSINFO"))?);

								if let Some(classinfo) = classinfo {
									let prop = classinfo.name("prop").unwrap().as_str();
									default_property = Some(prop);
								}
							},
							_ => {},
						}
						Ok::<_, anyhow::Error>(())
					})().with_context(|| format!("while parsing macro `{}`", macro_.get(0).unwrap().as_str()))?;
				}

				for invokable in self.fn_regex.captures_iter(body) {
					let comment = invokable.name("comment").map(|m| m.as_str());
					let type_ = invokable.name("type").unwrap().as_str();
					let name = invokable.name("name").unwrap().as_str();
					let params_raw = invokable.name("params").unwrap().as_str();

					let mut params = Vec::new();

					for param in self.fn_param_regex.captures_iter(params_raw) {
						let type_ = param.name("type").unwrap().as_str();
						let name = param.name("name").unwrap().as_str();

						params.push(InvokableParam {
							type_,
							name,
						});
					}

					invokables.push(Invokable {
						name,
						ret: type_,
						comment,
						params,
					});
				}

				if let Some(prop) = default_property {
					let prop = properties.iter_mut().find(|p| p.name == prop)
						.ok_or_else(|| anyhow!("could not find default property `{prop}`"))?;

					prop.default = true;
				}

				Ok::<_, anyhow::Error>(())
			})().with_context(|| format!("while parsing class `{name}`"))?;

			let Some(type_) = classtype else { continue };

			ctx.classes.push(ClassInfo {
				type_,
				name,
				qml_name,
				superclass,
				singleton,
				uncreatable,
				comment,
				properties,
				invokables,
			});
		}

		Ok(())
	}

	pub fn parse_enums<'a>(&self, text: &'a str, ctx: &mut ParseContext<'a>) -> anyhow::Result<()> {
		for enum_ in self.enum_regex.captures_iter(text) {
			let comment = enum_.name("comment").map(|m| m.as_str());
			let namespace = enum_.name("namespace").unwrap().as_str();
			let enum_name = enum_.name("enum_name").unwrap().as_str();
			let qml_name = enum_.name("qml_name").map(|m| m.as_str()).unwrap_or(namespace);
			let body = enum_.name("body").unwrap().as_str();

			let mut variants = Vec::new();

			for variant in self.enum_variant_regex.captures_iter(body) {
				let comment = variant.name("comment").map(|m| m.as_str());
				let name = variant.name("name").unwrap().as_str();

				variants.push(Variant {
					name,
					comment,
				});
			}

			ctx.enums.push(EnumInfo {
				namespace,
				enum_name,
				qml_name,
				comment,
				variants,
			});
		}

		Ok(())
	}

	pub fn parse<'a>(&self, text: &'a str, ctx: &mut ParseContext<'a>) -> anyhow::Result<()> {
		self.parse_classes(text, ctx)?;
		self.parse_enums(text, ctx)?;

		Ok(())
	}
}

impl ParseContext<'_> {
	pub fn gen_typespec(&self, module: &str) -> typespec::TypeSpec {
		typespec::TypeSpec {
			typemap: self.classes.iter().filter_map(|class| {
				Some(typespec::QmlTypeMapping {
					// filters gadgets
					name: class.qml_name?.to_string(),
					cname: class.name.to_string(),
					module: Some(module.to_string()),
				})
			}).collect(),
			classes: self.classes.iter().filter_map(|class| {
				let (description, details) = class.comment.map(parse_details_desc)
					.unwrap_or((None, None));

				Some(typespec::Class {
					name: class.name.to_string(),
					module: module.to_string(),
					description,
					details,
					// filters gadgets
					superclass: class.superclass?.to_string(),
					singleton: class.singleton,
					uncreatable: class.uncreatable,
					properties: class.properties.iter().map(|p| (*p).into()).collect(),
					functions: class.invokables.iter().map(|f| f.as_typespec()).collect(),
				})
			}).collect(),
			gadgets: self.classes.iter().filter_map(|class| match class.type_ {
				ClassType::Gadget => Some(typespec::Gadget {
					cname: class.name.to_string(),
					properties: class.properties.iter().map(|p| (*p).into()).collect(),
				}),
				_ => None,
			}).collect(),
			enums: self.enums.iter().map(|enum_| {
				let (description, details) = enum_.comment.map(parse_details_desc)
					.unwrap_or((None, None));

				typespec::Enum {
					name: enum_.qml_name.to_string(),
					module: Some(module.to_string()),
					cname: Some(format!("{}::{}", enum_.namespace, enum_.enum_name)),
					description,
					details,
					varaints: enum_.variants.iter().map(|v| (*v).into()).collect(),
				}
			}).collect(),
		}
	}
}

impl From<Property<'_>> for typespec::Property {
	fn from(value: Property) -> Self {
		Self {
			type_: value.type_.to_string(),
			name: value.name.to_string(),
			details: value.comment.map(parse_details),
			readable: value.readable,
			writable: value.writable,
			default: value.default,
		}
	}
}

impl From<Variant<'_>> for typespec::Variant {
	fn from(value: Variant<'_>) -> Self {
		Self {
			name: value.name.to_string(),
			details: value.comment.map(parse_details),
		}
	}
}

impl Invokable<'_> {
	fn as_typespec(&self) -> typespec::Function {
		typespec::Function {
			ret: self.ret.to_string(),
			name: self.name.to_string(),
			details: self.comment.map(parse_details),
			params: self.params.iter().map(|p| (*p).into()).collect(),
		}
	}
}

impl From<InvokableParam<'_>> for typespec::FnParam {
	fn from(value: InvokableParam<'_>) -> Self {
		Self {
			type_: value.type_.to_string(),
			name: value.name.to_string(),
		}
	}
}

fn parse_details(text: &str) -> String {
	let mut seen_content = false;
	let mut callout = false;

	let mut str = text.lines()
    .map(|line| {
			line.trim()
				.strip_prefix("///")
				.map(|line| line.strip_prefix(' ').unwrap_or(line))
				.unwrap_or(line)
		})
    .filter(|line| {
			let any = !line.is_empty();
			let filter = any || seen_content;
			seen_content |= any;
			filter
		})
    .map(|line| {
			match callout {
				true => {
					if line.starts_with('>') {
						Cow::Borrowed(line[1..].strip_prefix(' ').unwrap_or(&line[1..]))
					} else {
						callout = false;
						Cow::Owned(format!("{{{{< /callout >}}}}\n{line}"))
					}
				}
				false => {
					if line.starts_with("> [!") {
						let code = line[4..].split_once(']');

						if let Some((code, line)) = code {
							let code = code.to_lowercase();
							callout = true;
							return Cow::Owned(format!("{{{{< callout type=\"{code}\" >}}}}\n{line}"))
						}
					}

					return Cow::Borrowed(line);
				}
			}
		})
    .fold(String::new(), |accum, line| accum + line.as_ref() + "\n");

	if callout {
		str += "\n{{< /callout >}}";
	}

	str
}

fn parse_details_desc(text: &str) -> (Option<String>, Option<String>) {
	let details = parse_details(text);
	if details.starts_with('!') {
		match details[1..].split_once('\n') {
			Some((desc, details)) => (Some(desc.strip_prefix(' ').unwrap_or(desc).to_string()), Some(details.to_string())),
			None => (Some(details[1..].strip_prefix(' ').unwrap_or(&details[1..]).to_string()), None),
		}
	} else {
		(None, Some(details))
	}
}
