mod server;
mod config;
mod args;
mod paste;
mod error;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    server::app::start_paste_eater().await?;

    Ok(())
}