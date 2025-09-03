use actix_web::{
    dev::{ServiceRequest},
};

use uuid::Uuid;

pub fn is_valid_uuid(value: &str) -> bool {
    if value.parse::<Uuid>().is_ok() {
        return true;
    }

    if value.len() == 32 && value.chars().all(|c| c.is_ascii_hexdigit()) {
        return true;
    }

    false
}

pub fn get_id_from_header(req: &ServiceRequest, header_name: &str) -> Option<String> {
    let header_value = req.headers().get(header_name)?;
    let id_str = header_value.to_str().ok()?;

    if id_str.trim().is_empty() {
        None
    } else {
        Some(id_str.to_string())
    }
}

pub fn validate_id(id: &Option<String>, validator: Option<fn(&str) -> bool>) -> bool {
    match (id, validator) {
        (Some(id_str), Some(validate_fn)) => validate_fn(id_str),
        (Some(_), None) => true,
        (None, _) => false,
    }
}

pub fn generate_new_id() -> String {
    Uuid::new_v4().simple().to_string()
}
