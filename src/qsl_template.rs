use crate::qsl_type::QSL;
use askama::Template;

#[derive(Template)]
#[template(path = "template.html")]
pub struct RecordTemplate<'a> {
    pub callsign: &'a str,
    pub records_formal: &'a Vec<QSL>,
    pub records_eyeball: &'a Vec<QSL>,
}

mod filters {
    pub fn display_some<T>(value: &Option<T>, _: &dyn askama::Values) -> askama::Result<String>
    where
        T: std::fmt::Display,
    {
        Ok(match value {
            Some(value) => value.to_string(),
            None => String::new(),
        })
    }
}
