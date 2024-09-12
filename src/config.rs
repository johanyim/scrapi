use serde::{Deserialize, Serialize};
use serde_yaml::Value;

use crate::{model::Endpoint, prelude::*};
use std::{fs::File, io::BufReader};

use crate::model::Api;
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Config {
    pub state: Value,
    pub host: String,
    pub endpoints: Vec<Endpoint>,
}

impl TryFrom<File> for Config {
    type Error = Error;
    fn try_from(value: File) -> Result<Self> {
        let rdr = BufReader::new(value);

        let something: Config = serde_yaml::from_reader(rdr)?;
        return Ok(something);

        // println!("{something:#?}");
        //
        // let json_str = something.serialize(serde_json::value::Serializer).unwrap();
        //
        // let json = serde_json::to_string(&json_str).unwrap();
        // println!("json: {json}");
    }
}
