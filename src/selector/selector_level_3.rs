use lazy_static::lazy_static;
use scraper::{Selector};

lazy_static! {
    pub(crate) static ref SIZE_FILE_SELECTOR: Selector = Selector::parse("td:nth-child(1)").unwrap();
    pub(crate) static ref SIZE_FILE_ATTRIBUT: &'static str =  "text";
    pub(crate) static ref NAME_FILE_SELECTOR: Selector = Selector::parse("td:nth-child(2)").unwrap();
    pub(crate) static ref NAME_FILE_ATTRIBUT: &'static str =  "text";
}