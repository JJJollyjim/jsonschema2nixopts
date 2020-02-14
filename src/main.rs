use serde::{
    de::{self, Deserializer, MapAccess, Visitor},
    Deserialize,
};
use std::collections::{BTreeMap, BTreeSet};
use std::{fmt, io};

#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
#[serde(rename_all = "lowercase")]
enum Value {
    #[serde(rename_all = "camelCase")]
    Object {
        properties: Option<BTreeMap<String, Value>>,
        #[serde(default)]
        #[serde(deserialize_with = "deserialize_aps")]
        additional_properties: Option<Box<Value>>,
        #[serde(default)]
        required: BTreeSet<String>,
    },
    Array {
        items: Box<Value>,
    },
    Integer,
    Number,
    Boolean,
    String {
        enumm: Option<Box<[String]>>,
    },
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Value::*;
        match self {
            Object {
                properties: Some(properties),
                additional_properties: None,
                required,
            } => {
                write!(
                    f,
                    "types.submodule {{\ndescription = ''\n\n'';\noptions = {{\n"
                )?;
                for (key, val) in properties {
                    if required.contains(key) {
                        write!(f, "{} = {};", key, val)?;
                    } else {
                        write!(f, "{} = types.nullOr ({});", key, val)?;
                    }
                }
                write!(f, "}};\n}}")
            }
            Object {
                properties: None,
                additional_properties: Some(ap),
                ..
            } => write!(f, "types.attrsOf ({})", ap),
            Object {
                properties: None, ..
            } => write!(f, "types.attrs"),
            Object {
                properties: Some(_),
                additional_properties: Some(_),
                ..
            } => {
                unimplemented!();
            }
            Array { items } => write!(f, "types.listOf ({})", items),
            Integer => write!(f, "types.int"),
            Number => {
                // TODO
                write!(f, "types.int")
            }
            Boolean => write!(f, "types.bool"),
            String { enumm: None } => write!(f, "types.str"),
            String { enumm: Some(enumm) } => {
                write!(f, "types.enum [")?;
                for item in enumm.iter() {
                    write!(f, "{:?}", item)?;
                }
                write!(f, "]")
            }
        }
    }
}

struct AdditionalPropertiesVisitor;

impl<'de> Visitor<'de> for AdditionalPropertiesVisitor {
    type Value = Option<Box<Value>>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a valid additionalProperties value")
    }

    fn visit_bool<E: de::Error>(self, _: bool) -> Result<Self::Value, E> {
        Ok(None)
    }

    fn visit_map<M: MapAccess<'de>>(self, m: M) -> Result<Self::Value, M::Error> {
        Ok(Some(Box::new(Value::deserialize(
            de::value::MapAccessDeserializer::new(m),
        )?)))
    }
}

fn deserialize_aps<'de, D: Deserializer<'de>>(d: D) -> Result<Option<Box<Value>>, D::Error> {
    d.deserialize_any(AdditionalPropertiesVisitor)
}

fn main() {
    let v: Value = serde_yaml::from_reader(io::stdin().lock()).unwrap();
    println!("{}", nixpkgs_fmt::reformat_string(&v.to_string()));
}
