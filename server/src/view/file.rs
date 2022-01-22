use rocket::fs::TempFile;
use std::{path::{PathBuf}};
use std::time::{SystemTime, UNIX_EPOCH};
use rocket::form::{Form};

#[derive(FromForm)]
pub struct UploadForm<'v> {
    file: TempFile<'v>,
}

#[post("/upload", format="multipart/form-data", data = "<form>")]
pub async fn upload(mut form: Form<UploadForm<'_>>) -> std::io::Result<String> {
    let file = &mut form.file;
    let file_name = file.raw_name().unwrap().as_str().unwrap().to_owned();
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let new_file_name = format!("{}-{}", file_name, since_the_epoch.as_millis().to_string());
    let mut path: PathBuf = [new_file_name].iter().collect();
    path.set_extension(file.content_type().unwrap().extension().unwrap().as_str());
    match file.persist_to(path.as_path()).await {
        Ok(_) => {
            Ok(format!("Success! {}", path.to_str().unwrap()))
        },
        Err(e) => {
            Err(e)
        }
    }
}
