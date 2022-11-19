#[macro_use] extern crate rocket;
mod paste_id;
use paste_id::PasteId;

use rocket::tokio::fs::File;
use rocket::Request;
use rocket::response::content;
use std::fs;

// main page
#[get("/")]
fn index() -> &'static str {
    "-> szk
rust-based server for text (and file) storage.

-> usage
POST a string:
    curl --data-binary 'hi there!' https://szk.onrender.com
    echo \"hi there!\" | curl --data-binary @- https://szk.onrender.com

POST a file:
    cat file.txt | curl --data-binary @- https://szk.onrender.com

GET contents:
    curl /<id>

View in browser:
    /view/<id>

-> features
    syntax highlighting in /view mode

-> about
    source: https://github.com/guiszk/szk
    tutorial: https://rocket.rs/v0.5-rc/guide/pastebin-tutorial
    github: https://github.com/guiszk
    "
}

// say hi!
#[get("/hello/<name>")]
fn sayhi(name: &str) -> String {
    format!("hi, {}!", name)
}

// view contents
#[get("/view/<id>")]
async fn display(id: PasteId<'_>) -> content::RawHtml<String> {
    let cont = fs::read_to_string(id.file_path()).expect("LogRocket: Should have been able to read the file");
    let a = format!("
    <script src=\"https://cdn.jsdelivr.net/gh/google/code-prettify@master/loader/run_prettify.js\"></script>
    <link rel=\"stylesheet\" href=\"https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.3.1/styles/atom-one-dark.min.css\">
    <script src=\"https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.3.1/highlight.min.js\"></script>
    <script>hljs.initHighlightingOnLoad();</script>
    <style>
        body {{
            background-color: #1C1B22;
        }}  
    </style>
    <pre><code>{}</code></pre>
    ", cont);
    content::RawHtml(a)
}

// view contents
#[get("/<id>")]
async fn retrieve(id: PasteId<'_>) -> Option<File> {
    File::open(id.file_path()).await.ok()
}

use rocket::data::{Data, ToByteUnit};
use rocket::http::uri::Absolute;

const ID_LENGTH: usize = 4;
const HOST: Absolute<'static> = uri!("https://szk.onrender.com");

// post contents
#[post("/", data = "<paste>")]
async fn upload(paste: Data<'_>) -> std::io::Result<String> {
    let id = PasteId::new(ID_LENGTH);
    paste.open(128.kibibytes()).into_file(id.file_path()).await?;
    Ok(uri!(HOST, retrieve(id)).to_string()+"\n")
}

//delete contents
// better to not allow deletion for now
/* #[delete("/<id>")]
async fn delete(id: PasteId<'_>) -> String {
    fs::remove_file(id.file_path()).ok();
    format!("Deleted {}\n", id.file_path().display())
} */

// exceptions
#[catch(404)]
fn not_found(req: &Request) -> String {
    format!("404: '{}' is not a valid path.", req.uri())
}

#[catch(500)]
fn server_error(req: &Request) -> String {
    format!("whoops! internal error! {}", req.uri())
}

// run
#[launch]
fn rocket() -> _ {
    rocket::build()
    .mount("/", routes![index, retrieve, upload, sayhi, display, delete])
    .register("/", catchers![not_found, server_error])
}