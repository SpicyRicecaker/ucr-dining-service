use awc::http::header;
use serde::{Deserialize, Serialize};
use std::error::Error;

use actix_web::{
    error, get, http::StatusCode, middleware::Logger, web, App, HttpResponse, HttpServer, Responder,
};

#[get("/")]
async fn get() -> Result<String, Box<dyn Error>> {
    Ok(String::from("hello world"))
}

#[derive(Debug, Serialize, Deserialize)]
struct Out {
    lothian: Option<String>,
    glasgow: Option<String>,
}

#[get("/dininghalls")]
async fn fetch() -> Result<impl Responder, Box<dyn Error>> {
    // client code from https://docs.rs/awc/latest/awc/ & discovered on actix github
    let client = awc::Client::default();

    let mut out = Out {
        lothian: None,
        glasgow: None,
    };

    for (i, name) in ["https://foodpro.ucr.edu/foodpro/shortmenu.asp?sName=University+of+California%2C+Riverside+Dining+Services&locationNum=02&locationName=Lothian+Residential+Restaurant&naFlag=1", "https://foodpro.ucr.edu/foodpro/shortmenu.asp?sName=University%20of%20California%2C%20Riverside%20Dining%20Services&locationNum=03&locationName=Glasgow&naFlag=1"].into_iter().enumerate() {
        let req = client.get(name);
        // make request to actual server now
        let mut res = req.send().await?;

        if res.status() != StatusCode::from_u16(200).unwrap() {
            return Ok(HttpResponse::build(res.status()).body(res.body().await?));
        } else {
            let bytes = &res.body().await?;
            let text = std::str::from_utf8(bytes).unwrap().to_string();
            match i {
                0 => {
                    out.lothian = Some(text);
                }
                1 => {
                    out.glasgow = Some(text);
                }
                _ => {unreachable!()}
            }
        }
    }

    let mut builder = HttpResponse::build(StatusCode::from_u16(200).unwrap());

    builder.append_header((header::ACCESS_CONTROL_ALLOW_ORIGIN, "*"));

    Ok(builder.body(serde_json::to_string(&out).unwrap()))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = if let Ok(port) = std::env::var("PORT") {
        port.parse::<u16>().unwrap()
    } else {
        8080
    };

    println!("Starting server at http://127.0.0.1:8080");
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    HttpServer::new(|| {
        App::new()
            .service(get)
            .service(fetch)
            .wrap(Logger::new("%a %r %s"))
            .app_data(web::JsonConfig::default().error_handler(|err, _req| {
                error::InternalError::from_response(
                    "",
                    HttpResponse::BadRequest()
                        .content_type("application/json")
                        .body(format!(r#"{{"error":"{}"}}"#, err)),
                )
                .into()
            }))
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
