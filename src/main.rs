#[macro_use] extern crate rocket;
use rocket::Request;
use rocket::http::Status;
use rocket_contrib::templates::Template;

#[get("/")]
fn index() -> Template {
    "Hello, world!"
}

#[get("/algorithms")]
fn algorithms() -> Template {
    "Hello, world!"
}

#[get("/cryptography")]
fn algorithms() -> Template {
    "Hello, world!"
}

#[get("/data-structures")]
fn algorithms() -> Template {
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
                    .mount("/simple", routes![algorithms])
                    .register("/", catchers![default_catcher])
                    .register("/", catchers![not_found])
                    .attach(Template::fairing())

}


