#[macro_use] extern crate rocket;
extern crate rocket;
extern crate rocket_dyn_templates;

use rocket::Request;
use rocket::http::Status;
use rocket::response::content::Json;
use rocket::serde::Serialize;
use rocket_dyn_templates::handlebars::Handlebars;
use rocket_dyn_templates::Template;

#[get("/")]
fn index() {
    #[derive(Serialize)]
    struct Context {
        title: String,
        expl: String
    }
    let context = Context {
        title: String::from("Rust Portfolio"),
        expl: String::from("loremipsum")
    };
    let mut hb = Handlebars::new();
    assert!(hb.register_template_file("index", "index.hbs").is_ok());

    hb.render_template("index", &context);
}

#[get("/algorithms")]
fn algorithms() -> () {
  
}

#[get("/cryptography")]
fn cryptography() -> () {
  
}

#[get("/data-structures")]
fn data_structures() -> () {
  
}

#[catch(404)]
fn not_found(req: &Request) {

 }

 #[catch(default)]
fn default_catcher(status: Status, request: &Request) { 

}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
                    .mount("/", routes![algorithms])
                    .mount("/", routes![cryptography])
                    .mount("/", routes![data_structures])
                    .register("/", catchers![default_catcher])
                    .register("/", catchers![not_found])
                    .attach(Template::fairing())

}


