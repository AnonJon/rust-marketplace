
use rocket::response::content::Json;
use std::path::{Path, PathBuf};
use rocket::fs::NamedFile;


#[get("/")]
pub fn index() -> Json<&'static str> {
    Json("{
        'status': 'success',
        'message': 'Working!'
    }")
}

#[get("/<file>")]
pub async fn home(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).await.ok()
}