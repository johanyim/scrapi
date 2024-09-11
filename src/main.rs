mod config;
mod error;
mod model;
mod prelude;
mod routes;
use std::{
    fs::{self, File},
    sync::{Arc, Mutex},
};

use axum::{extract::State, handler::Handler, http::StatusCode, routing::get, Extension, Router};

use config::Config;
use prelude::*;
use serde::Serialize;
// use routes::r1;

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}
#[tokio::main]
async fn main() {
    // Create a parameterized route using a closure
    let filename = "./example.yaml";
    let f = File::open(filename).expect("could not find file");
    let config = Config::try_from(f).unwrap();

    // for endpoint in &config.endpoints {
    //     println!(
    //         "{:?}, {:#?}",
    //         endpoint.method,
    //         endpoint
    //             .returns
    //             .serialize(serde_json::value::Serializer)
    //             .unwrap()
    //     )
    // }

    let mut app = Router::new();

    let state = Arc::new(Mutex::new(AppState {}));

    // create all endpoints
    for endpoint in config.endpoints {
        app = app.route(
            &endpoint.to,
            get({
                move |State(state): State<Arc<Mutex<AppState>>>| async move {
                    println!("state = {:?}", state);
                    (
                        StatusCode::from_u16(endpoint.status.unwrap()).unwrap(),
                        axum::Json(serde_json::json!(&endpoint.returns)),
                    )
                }
            }),
        )
    }

    let app = app.with_state(state);

    // .route("/", get(index));
    let listener = tokio::net::TcpListener::bind(&config.host).await.unwrap();

    println!("listening on {}", &config.host);

    axum::serve(listener, app).await.unwrap()
}

#[derive(Debug, Clone)]
struct AppState {}

async fn index() -> String {
    return "Hello".to_string();
}

#[cfg(test)]
mod test {
    use std::{fs::File, io::BufReader};

    use serde::Serialize;
    use serde_yaml::Value;

    use super::*;

    #[test]
    #[ignore]
    fn it_can_read() {
        let filename = "./example.yaml";
        let f = File::open(filename).expect("could not find file");
        let rdr = BufReader::new(f);

        let something: Value = serde_yaml::from_reader(rdr).unwrap();

        println!("{something:#?}");

        let json_str = something.serialize(serde_json::value::Serializer).unwrap();

        let json = serde_json::to_string(&json_str).unwrap();
        println!("json: {json}");
    }
}
