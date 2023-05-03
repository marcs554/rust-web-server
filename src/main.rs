#[macro_use] extern crate rocket;
extern crate model;

mod view;

use crate::view::{static_files, gets};
use rocket::{Config, config::LogLevel};
use std::net::{IpAddr, Ipv4Addr};

use rocket_dyn_templates::Template;


#[launch]
fn rocket() -> _ {

    let config = Config {
        log_level: LogLevel::Normal,
        address: IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
        port: 8889,
        ..Config::debug_default()
    };

    rocket::build()
    .attach(Template::fairing())
    .mount("/", routes![
        gets::index, 
        gets::data,
        static_files::file,
        ])
    .register("/", catchers![gets::not_found])
    .configure(config)

}
