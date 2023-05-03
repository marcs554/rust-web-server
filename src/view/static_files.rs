use std::path::{Path, PathBuf};
use rocket::fs::{NamedFile, relative};

#[get("/<file..>")]
pub async fn file(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new(relative!("static/"))
        .join(file))
        .await
        .ok()
}