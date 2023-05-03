use rocket_dyn_templates::{Template, context};
use rocket::serde::{json::Json};
use model::GeneralData;



#[get("/")]
pub fn index() -> Template {
    Template::render("index", context! {field: "value"})
}

#[catch(404)]
pub fn not_found() -> Template {
    Template::render("404", context! {field: "value"})
}

#[get("/json")]
pub fn data() -> Json<GeneralData> {
    let some_data = GeneralData::add(
        String::from("Content-Type: JSON"), 
        16, 
        true, 
        'L'
    );
    Json(some_data)
}