use std::path::{PathBuf, Path};

use clap::Parser;
use rocket::{routes, fairing::{Fairing, Info, Kind}, http::Header, Request, Response, fs::{FileServer, relative, NamedFile}, get};

use crate::{server::endpoints, args::Args, config::ConfigurationHandler, paste::PasteHandler, error::PasteEaterError};

pub async fn start_paste_eater() -> Result<(), rocket::Error> {
    let paste_handler = match create_paste_handler() {
        Ok(ph) => ph,
        Err(e) => {
            panic!("{}", e);
        },
    };

    let _rocket = rocket::build()
    .manage(paste_handler)
    .attach(Cors)
    .mount("/api", routes![endpoints::create_paste, endpoints::get_paste, endpoints::delete_paste])
    .mount("/", routes![serve_frontend])
    .mount("/static", FileServer::from(relative!("/paste-eater-frontend/build/static")).rank(10))
    .launch()
    .await?;

    Ok(())
}

#[get("/<file..>", rank = 9)]
async fn serve_frontend(file: PathBuf) -> Option<NamedFile> {
    if file.extension().is_some() {
        return NamedFile::open(Path::new(relative!("/paste-eater-frontend/build/")).join(file)).await.ok();
    }

    NamedFile::open(Path::new(relative!("/paste-eater-frontend/build/index.html"))).await.ok()
}

fn create_paste_handler() -> Result<PasteHandler, PasteEaterError> {
    let args = Args::parse();

    let config_handler = ConfigurationHandler::new_with_args(&args)?;

    Ok(PasteHandler::new(config_handler))
}

pub struct Cors;

#[rocket::async_trait]
impl Fairing for Cors {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, PATCH, OPTIONS"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}