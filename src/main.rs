use std::path::{PathBuf, Path};
use rocket::{fs::NamedFile, response::status::NotFound};

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "hello world!"
}

#[get("/?<category>")]
async fn film(category: &str) -> Result<NamedFile, NotFound<String>> {
    let index_file: PathBuf = "./src/public/index.html".into();
    let index_path = Path::new("public/").join(index_file);

    let horror_file: PathBuf = "./src/public/horror.html".into();
    let horror_path = Path::new("public/").join(horror_file);

    let romance_file: PathBuf = "./src/public/romance.html".into();
    let romance_path = Path::new("public/").join(romance_file);
    
    let sliceoflife_file: PathBuf = "./src/public/sliceoflife.html".into();
    let sliceoflife_path = Path::new("public/").join(sliceoflife_file);

    match category {
        "horror" => NamedFile::open(&horror_path).await.map_err(|e| NotFound(e.to_string())),
        "romance" => NamedFile::open(&romance_path).await.map_err(|e| NotFound(e.to_string())),
        "sliceOfLife" => NamedFile::open(&sliceoflife_path).await.map_err(|e| NotFound(e.to_string())),
        _ => NamedFile::open(&index_path).await.map_err(|e| NotFound(e.to_string()))
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/film", routes![film])
}