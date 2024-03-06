use std::fmt::Display;

use actix_web::{post, web::Data, App, HttpServer, Responder, ResponseError};

use crate::anony_muse_client::AnonyMuseClient;

#[derive(Debug)]
struct Error(anyhow::Error);

impl From<anyhow::Error> for Error {
    fn from(value: anyhow::Error) -> Self {
        Self(value)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl ResponseError for Error {}

type Result<T> = std::result::Result<T, Error>;

#[post("/encode")]
async fn encode(body: String, client: Data<AnonyMuseClient>) -> Result<impl Responder> {
    Ok(client.encode(&body)?)
}

#[post("/decode")]
async fn decode(body: String, client: Data<AnonyMuseClient>) -> Result<impl Responder> {
    Ok(client.decode(&body)?)
}

pub(crate) async fn run(port: u16) -> anyhow::Result<()> {
    let anony_muse_client = Data::new(AnonyMuseClient);

    HttpServer::new(move || {
        App::new()
            .app_data(anony_muse_client.clone())
            .service(encode)
            .service(decode)
    })
    .bind(("::", port))?
    .run()
    .await?;

    Ok(())
}
