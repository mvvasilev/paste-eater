use std::path::{PathBuf, Path};

use rocket::{post, get, response::status::Custom, http::Status, State, serde::json::Json, delete, fs::NamedFile};
use serde_derive::{Serialize, Deserialize};

use crate::paste::{PasteHandler, PasteOutput};

#[derive(Serialize, Deserialize)]
pub struct PasteInput {
    encrypted: bool,
    language: String,
    data: String
}

// #[get("/<file..>", rank = 10)]
// pub fn serve_frontend(file: PathBuf) -> Option<NamedFile> {
//     //NamedFile::open(Path::new("static/").join(file)).ok().or_else(|| NamedFile::open(Path::new("static/index.html")).ok())
//     //NamedFile::open(Path::new("paste-eater-frontend/build").join(file)).await.ok()
// }

#[post("/paste", data = "<paste>", rank = 2)]
pub fn create_paste(paste: Json<PasteInput>, paste_handler: &State<PasteHandler>) -> Custom<String> {
    match paste_handler.create_new_paste(paste.encrypted, &paste.language, &paste.data) {
        Ok(uid) => Custom(Status::Ok, uid),
        Err(e) => Custom(Status::InternalServerError, format!("{}", e)),
    }
}

#[derive(Serialize, Deserialize)]
pub struct PasteResponse {
    pub paste: Option<PasteOutput>,
    pub error: Option<String>
}

#[get("/paste/<uid>", rank = 2)]
pub fn get_paste(uid: String, paste_handler: &State<PasteHandler>) -> Custom<Json<PasteResponse>> {
    match paste_handler.fetch_raw_paste(uid) {
        Ok(paste) => Custom(Status::Ok, Json(PasteResponse { paste: Some(paste), error: None })),
        Err(e) => Custom(Status::InternalServerError, Json(PasteResponse { paste: None, error: Some(format!("{}", e)) })),
    }
}

#[delete("/paste/<uid>", rank = 2)]
pub fn delete_paste(uid: String, paste_handler: &State<PasteHandler>) -> Custom<String> {
    match paste_handler.delete_paste(uid) {
        Ok(_) => Custom(Status::Ok, "".to_string()),
        Err(e) => Custom(Status::InternalServerError, format!("{}", e))
    }
}