use lazy_static::lazy_static;
use scraper::{Selector};

lazy_static! {
    pub(crate) static ref UPLOADED_RATIO_SELECTOR: Selector = Selector::parse("#top_panel > div.ct > ul > li:nth-child(1) > strong:nth-child(1)").unwrap(); //href
    pub(crate) static ref UPLOADED_RATIO_ATTRIBUT: &'static str =  "text";
    pub(crate) static ref DOWNLOADED_RATIO_SELECTOR: Selector = Selector::parse("#top_panel > div.ct > ul > li:nth-child(1) > strong:nth-child(2)").unwrap();
    pub(crate) static ref DOWNLOADED_RATIO_ATTRIBUT: &'static str =  "text";
    pub(crate) static ref RATIO_SELECTOR: Selector = Selector::parse("#top_panel > div.ct > ul > li:nth-child(2) > a > strong").unwrap();
    pub(crate) static ref RATIO_ATTRIBUT: &'static str =  "text";
}