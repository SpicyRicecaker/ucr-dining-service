use awc::http::header;
use std::error::Error;
use ucr_dining_service::Config;

use actix_web::{error, post, web, App, HttpResponse, HttpServer, Responder, middleware::Logger};

#[post("/curl")]
async fn curl(config: web::Json<Config>) -> Result<impl Responder, Box<dyn Error>> {
    // client code from https://docs.rs/awc/latest/awc/ & discovered on actix github
    let client = awc::Client::default();

    // make request to actual server now
    let req = client.get(&config.url);

    let mut res = req.send().await?;

    // need to parse
    let body = res.body().await?;

    let mut builder = HttpResponse::build(res.status());

    builder.append_header((header::ACCESS_CONTROL_ALLOW_ORIGIN, "*"));

    Ok(builder.body(body))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server at http://127.0.0.1:8080");
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    HttpServer::new(|| {
        App::new()
            .service(curl)
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
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
