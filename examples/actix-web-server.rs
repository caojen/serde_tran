//!
//! In this example, we will create an actix-web server, and then use [serde_tran::Json] to
//! 1. accept Json body
//! 2. throw Json response
//!
//! The example creates an endpoint, POST '/login'.
//!
//! HTTP client sends username and password with `application/json` content-type, (see [LoginRequest])
//! and then the HTTP client will return `application/json` response. (see [HelloWorldResponse])
//!
//! In production, **usually** we (as the server side) return something like this
//! (see [MyResponse], the difference is, we replace the `data` field with a fixed struct [serde_tran::Json], instead of type `T`):
//!
//! ```json
//! {
//!     "data": {
//!         "message": "Hello, world!"
//!     },
//!     "code": 200, // the http code
//!     "error": null
//! }
//! ```
//!
//! Access this endpoint by `curl`:
//! use a new terminal and run (username = "serde_tran:username", password = "serde_tran:password:123456"):
//! ```bash
//! curl -X POST http://127.0.0.1:8080/login -H "Content-Type: application/json" -d '{"f":"base64","v":"NgAAAAAAAAATAAAAAAAAAHNlcmRlX3RyYW46dXNlcm5hbWUTAAAAAAAAAHNlcmRlX3RyYW46cGFzc3dvcmRnmz7nMG94SA"}'
//!
//! > {"data":{"f":"base64","v":"FQAAAAAAAAANAAAAAAAAAEhlbGxvLCB3b3JsZCEgHUu5szXXFQ"},"code":201,"error":null}
//! ```
//!
//! > the body of this request is from [generate_login_request_body]
//!

use data_structure::*;

#[test]
fn generate_login_request_body() {
    // generate the login request body, with username and password
    let req = LoginRequest {
        username: String::from("serde_tran:username"),
        password: String::from("serde_tran:password"),
    };

    let json = serde_tran::to_json(&req).unwrap();
    let s = json.to_string().unwrap();

    println!("{}", &s);
}

/// Just create the HTTP server and start to listening at 0.0.0.0:8080
#[tokio::main]
pub async fn main() -> anyhow::Result<()> {
    println!("starting server");

    start_server::start("0.0.0.0", 8080).await
}


// this is how we defined structs in this example
mod data_structure {
    use serde::{Deserialize, Serialize};
    use serde_tran::Json;

    /// All Http Request is [serde_tran::Json]
    pub type Request = Json;

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct LoginRequest {
        pub username: String,
        pub password: String,
    }

    /// [MyResponse] is the HTTP response body struct, which we throw to the HTTP client.
    #[derive(Serialize, Debug, Clone)]
    pub struct MyResponse {
        /// Note: use [serde_tran::Json] here
        pub data: serde_tran::Json,
        /// HTTP status code
        pub code: u16,
        /// Note: in this example, just return None ( json `null` )
        pub error: Option<()>,
    }

    impl MyResponse {
        /// new a [Self].
        ///
        /// note: the first argument (`my_data`) is your custom struct (maybe [HelloWorldResponse])
        pub fn new<T: Serialize>(my_data: &T, code: u16) -> Self {
            Self {
                data: serde_tran::to_json(my_data).unwrap(), // in practice, you need to handle this error. (consider using [anyhow::Result])
                code,
                error: None,
            }
        }
    }

    /// A simple struct that just return { "message": "Hello, world!" }
    #[derive(Serialize, Debug, Clone)]
    pub struct HelloWorldResponse {
        pub message: &'static str,
    }
}

// methods that we handle HTTP actix-web server
mod start_server {
    use actix_web::web;
    use actix_web::HttpResponse;

    use super::*;

    pub async fn start(host: &str, port: u16) -> anyhow::Result<()> {
        actix_web::HttpServer::new(|| {
            actix_web::App::new()
                .service(web::resource("/login").route(web::post().to(login)))
        })
            .bind(format!("{}:{}", host, port))?
            .run()
            .await?;

        unreachable!()
    }

    async fn login(body: web::Json<Request>) -> HttpResponse {
        println!("{:?}", &body);

        // decode body into username and password:
        let body: LoginRequest = body.to_value().unwrap(); // note: handle this error in production
        println!("username = {}, password = {}", &body.username, &body.password);

        let hello_world = HelloWorldResponse {
            message: "Hello, world!",
        };

        let resp = MyResponse::new(&hello_world, 201);
        HttpResponse::Accepted().json(&resp)
    }
}
