use serde::{Deserialize, Serialize};
use serde_yaml::Value;

#[derive(Debug, Deserialize, Clone)]
pub struct Api {
    #[serde(flatten)]
    pub method: Method,
    pub to: String,
    pub returns: Value,
    pub status: Option<u16>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Method {
    Get,
    Post,
    Patch,
    Put,
    Delete,
}

#[cfg(test)]
mod test {
    use super::*;

    // #[test]
    // fn something() {
    //     let x = Api::Get {
    //         to: "/index".to_string(),
    //         returns: Some(Value::default()),
    //         status: Some(4u16),
    //     }
    //     .serialize(serde_yaml::value::Serializer);
    //     println!("{x:?}");
    // }
}
