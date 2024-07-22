use std::collections::HashMap;

use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ModuleIndex {
	pub name: String,
	pub description: String,
	pub details: String,
}

#[derive(Debug, Serialize)]
pub struct TypeInfo {
	pub name: String,
	pub module: String,
	#[serde(flatten)]
	pub details: TypeDetails,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "lowercase")]
#[serde(tag = "type")]
pub enum TypeDetails {
	Class(ClassInfo),
	Enum(EnumInfo),
}

#[derive(Debug, Serialize)]
pub struct ClassInfo {
	#[serde(rename = "super")]
	pub superclass: Type,
	pub description: Option<String>,
	pub details: Option<String>,
	#[serde(skip_serializing_if = "Vec::is_empty")]
	pub flags: Vec<Flag>,
	pub properties: HashMap<String, Property>,
	pub functions: Vec<Function>,
	pub signals: HashMap<String, Signal>,
	pub variants: HashMap<String, Variant>,
}

#[derive(Debug, Serialize)]
pub struct Property {
	#[serde(rename = "type")]
	pub type_: PropertyType,
	pub details: Option<String>,
	#[serde(skip_serializing_if = "Vec::is_empty")]
	pub flags: Vec<Flag>,
}

#[derive(Debug, Serialize)]
pub enum PropertyType {
	#[serde(rename = "gadget")]
	Gadget(HashMap<String, PropertyType>),
	#[serde(untagged)]
	Type(Type),
}

#[derive(Debug, Serialize)]
pub struct Function {
	pub ret: Type,
	pub name: String,
	pub id: String,
	pub details: Option<String>,
	pub params: Vec<Parameter>,
}

#[derive(Debug, Serialize)]
pub struct Signal {
	pub name: String,
	pub details: Option<String>,
	pub params: Vec<Parameter>,
}

#[derive(Debug, Serialize)]
pub struct Parameter {
	pub name: String,
	#[serde(rename = "type")]
	pub type_: Type,
}

#[derive(Debug, Serialize)]
pub struct EnumInfo {
	pub description: Option<String>,
	pub details: Option<String>,
	pub variants: HashMap<String, Variant>,
}

#[derive(Debug, Serialize)]
pub struct Variant {
	pub details: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct Type {
	#[serde(rename = "type")]
	pub type_: TypeSource,
	pub module: String,
	pub name: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub of: Option<Box<Type>>,
}

impl Type {
	pub fn resolve(module: Option<&str>, name: &str) -> Self {
		let (src, module) = match module {
			None => (TypeSource::Qt, "qml".to_string()),
			Some(name) if name.starts_with("qml.") => (TypeSource::Qt, name.to_string()),
			Some(name) => (TypeSource::Local, name.to_string()),
		};

		Type {
			type_: src,
			module,
			name: name.to_string(),
			of: None,
		}
	}

	pub fn unknown() -> Type {
		Type {
			type_: TypeSource::Unknown,
			module: "".to_string(),
			name: "".to_string(),
			of: None,
		}
	}
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum TypeSource {
	Qt,
	Local,
	Unknown,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Flag {
	Default,
	Readonly,
	Writeonly,
	Singleton,
	Uncreatable,
	Enum,
}
