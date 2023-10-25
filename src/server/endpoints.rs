use rocket::{post, get, response::status::Custom, http::Status, State, serde::json::Json};
use serde_derive::{Serialize, Deserialize};

use crate::paste::{PasteHandler, PasteOutput, PasteLanguage};

#[derive(Serialize, Deserialize)]
pub struct PasteInput {
    encrypted: bool,
    language: PasteLanguage,
    data: String
}

#[post("/paste", data = "<paste>")]
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

#[get("/paste/<uid>")]
pub fn get_paste(uid: String, paste_handler: &State<PasteHandler>) -> Custom<Json<PasteResponse>> {
    match paste_handler.fetch_raw_paste(uid) {
        Ok(paste) => Custom(Status::Ok, Json(PasteResponse { paste: Some(paste), error: None })),
        Err(e) => Custom(Status::InternalServerError, Json(PasteResponse { paste: None, error: Some(format!("{}", e)) })),
    }
}