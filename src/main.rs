#[macro_use] extern crate rocket;
use rocket::Request;
use rocket::http::Status;
use rocket_contrib::templates::Template;

#[get("/")]
fn index() -> Template {
    Template::render("template-name", &context)
}

#[get("/algorithms")]
fn algorithms() -> Template {
    "Hello, world!"
}

#[get("/cryptography")]
fn cryptography() -> Template {
    "Hello, world!"
}

#[get("/data-structures")]
fn data_structures() -> Template {
    "Hello, world!"
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


