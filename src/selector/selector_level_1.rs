use lazy_static::lazy_static;
use scraper::{Selector};

lazy_static! {
    pub(crate) static ref LINK_TORRENT_PAGE_SELECTOR: Selector = Selector::parse("#torrent_name").unwrap(); //href
    pub(crate) static ref LINK_TORREN_PAGE_ATTRIBUT: &'static str =  "href";
    pub(crate) static ref LINK_NEXT_PAGE_SELECTOR: Selector = Selector::parse("#middle > main > div > div > section:nth-child(4) > ul > li:nth-child(10) > a").unwrap();
    pub(crate) static ref LINK_NEXT_PAGE_ATTRIBUT: &'static str =  "href";
}