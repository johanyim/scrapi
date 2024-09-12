use std::ops::Deref;

use serde::{
    ser::{SerializeStruct, SerializeStructVariant},
    Deserialize, Serialize,
};
use serde_yaml::Value;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Api {
    pub to: String,
    pub returns: Value,
    pub status: Option<u16>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Endpoint {
    Get(Api),
    Post(Api),
    Put(Api),
    Delete(Api),
}

impl Deref for Endpoint {
    type Target = Api;

    fn deref(&self) -> &Self::Target {
        match self {
            Endpoint::Get(api) => api,
            Endpoint::Post(api) => api,
            Endpoint::Put(api) => api,
            Endpoint::Delete(api) => api,
        }
    }
}

// #[ignore]
#[cfg(test)]
mod test {
    use std::{
        fs::{File, OpenOptions},
        io::{Read, Write},
    };

    use crate::config::Config;

    use super::*;
    #[test]
    fn shape() {
        let mut file = File::open("./example.yaml").expect("config.yaml not found");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("could not read contents of file");

        let mut config: Value = serde_yaml::from_str(&contents).expect("Could not turn to yaml");
        println!("Config as Value: \n {config:#?}");
        // config.state = "This".into();
    }

    #[test]
    fn deserialize_change_serialize() {
        let mut file = File::open("./example.yaml").expect("config.yaml not found");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("could not read contents of file");

        let mut config: Config = serde_yaml::from_str(&contents).expect("Could not turn to yaml");
        config.state = "This".into();

        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open("../new_example.yaml")
            .expect("writing failed");

        // println!("config: {config:?}");
        let new_contents = serde_yaml::to_string(&config).unwrap();

        file.write_all(new_contents.as_bytes()).unwrap();
        assert_eq!(contents, new_contents);
    }
}
