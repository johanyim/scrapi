use serde::{Deserialize, Serialize};
use serde_yaml::Value;

use crate::prelude::*;
use std::{fs::File, io::BufReader};

use crate::model::Api;
#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub endpoints: Vec<Api>,
    pub host: String,
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
