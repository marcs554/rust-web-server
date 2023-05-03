use rocket::serde::Serialize;


#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct GeneralData {
    content: String,
    mask: u32,
    is_reacheable: bool,
    type_crc: char
}

impl GeneralData {
    pub fn add(_content: String, _mask: u32, _is_reacheable: bool, _type_crc: char) -> GeneralData {
        return GeneralData { 
            content: _content,
            mask: _mask,
            is_reacheable: _is_reacheable, 
            type_crc: _type_crc 
        }
    }
}