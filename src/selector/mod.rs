use scraper::ElementRef;
pub(crate) mod selector_level_1;
pub(crate) mod selector_level_2;
pub(crate) mod selector_level_3;
pub(crate) mod selector_get_ratio;

pub(crate) fn get_data(element: ElementRef, attribut: &'static str) -> String {
    if attribut != "text" {
        element.attr(attribut).unwrap().to_string()
    }
    else {
        element.text().next().unwrap().to_string()
    }
}