
#[macro_use] extern crate rocket;

mod web;
mod prelude {
    pub use crate::web::{index, home};
}
use crate::prelude::*;

#[launch]
fn rocket() -> _ {
    rocket::build()
    .mount("/", routes![index])
    .mount("/file", routes![home])
}

