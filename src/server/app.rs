use clap::Parser;
use rocket::routes;

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
    .mount("/api", routes![endpoints::create_paste, endpoints::get_paste])
    .launch()
    .await?;

    Ok(())
}

fn create_paste_handler() -> Result<PasteHandler, PasteEaterError> {
    let args = Args::parse();

    let config_handler = ConfigurationHandler::new_with_args(&args)?;

    Ok(PasteHandler::new(config_handler))
}