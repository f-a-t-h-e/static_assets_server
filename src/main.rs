use rocket::fs::NamedFile;
#[macro_use] extern crate rocket;

#[get("/")]
async fn index() -> Result<NamedFile, std::io::Error> {
    NamedFile::open("static/favicon.png").await
}

#[get("/<project>/<num>")]
async fn get_img(project: String, num: String) -> Result<NamedFile, std::io::Error> {
    NamedFile::open(format!("static/{project}/{num}-dark.webp")).await
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, get_img])
}