use lazy_static::lazy_static;
use scraper::{Selector};

lazy_static! {
    pub(crate) static ref NAME_SELECTOR: Selector = Selector::parse("#title > h1").unwrap();
    pub(crate) static ref NAME_ATTRIBUT: &'static str =  "text";
    pub(crate) static ref KEYWORD_SELECTOR: Selector = Selector::parse("a.term").unwrap();
    pub(crate) static ref KEYWORD_ATTRIBUT: &'static str =  "text";
    pub(crate) static ref SEEDERS_SELECTOR: Selector = Selector::parse("#adv_search_cat > td:nth-child(2) > strong").unwrap();
    pub(crate) static ref SEEDERS_ATTRIBUT: &'static str =  "text";
    pub(crate) static ref LEECHERS_SELECTOR: Selector = Selector::parse("#adv_search_cat > td:nth-child(4) > strong").unwrap();
    pub(crate) static ref LEECHERS_ATTRIBUT: &'static str =  "text";
    pub(crate) static ref COMPLETED_SELECTOR: Selector = Selector::parse("#adv_search_cat > td:nth-child(6) > strong").unwrap();
    pub(crate) static ref COMPLETED_ATTRIBUT: &'static str =  "text";
    pub(crate) static ref SIZE_SELECTOR: Selector = Selector::parse("#informationsContainer > div > table > tbody > tr:nth-child(4) > td:nth-child(2)").unwrap();
    pub(crate) static ref SIZE_ATTRIBUT: &'static str = "text";
    pub(crate) static ref HASH_SELECTOR: Selector = Selector::parse("#informationsContainer > div > table > tbody > tr:nth-child(5) > td:nth-child(2)").unwrap();
    pub(crate) static ref HASH_ATTRIBUT: &'static str = "text";
    pub(crate) static ref UPLOADER_SELECTOR: Selector = Selector::parse("#informationsContainer > div > table > tbody > tr:nth-child(6) > td:nth-child(2) > a").unwrap();
    pub(crate) static ref UPLOADER_ATTRIBUT: &'static str = "text";
    pub(crate) static ref UPLOADED_SELECTOR: Selector = Selector::parse("#informationsContainer > div > table > tbody > tr:nth-child(7) > td:nth-child(2)").unwrap();
    pub(crate) static ref UPLOADED_ATTRIBUT: &'static str = "text";
    pub(crate) static ref DOWNLOAD_LINK_SELECTOR: Selector = Selector::parse("#informationsContainer > div > table > tbody > tr:nth-child(2) > td:nth-child(2) > a").unwrap();
    pub(crate) static ref DOWNLOAD_LINK_ATTRIBUT: &'static str = "href";
}