use std::fmt::{Display, Formatter, write};
use getset::Getters;
use scraper::Html;
use crate::selector::get_data;
use crate::selector::selector_get_ratio::*;

#[derive(Debug, Clone, Default, Getters)]
#[allow(dead_code)]
pub struct YggRatio {
    #[getset(get = "pub")]
    percentage: f32,
    #[getset(get = "pub")]
    uploaded: String,
    #[getset(get = "pub")]
    downloaded: String,
}

impl Display for YggRatio {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Uploaded : {}\t", self.uploaded)?;
        write!(f, "Downloaded : {}", self.downloaded)?;
        write!(f, "\t|\t Ratio : {}", self.percentage)?;
        Ok(())
    }
}

impl YggRatio {

    pub(crate) fn scrape(document: Html) -> YggRatio {
        let mut result = YggRatio::default();
        result.uploaded = get_data(document.select(&UPLOADED_RATIO_SELECTOR).next().unwrap(), &UPLOADED_RATIO_ATTRIBUT).trim_end().trim_start().replace("\n","").to_string();
        result.downloaded = get_data(document.select(&DOWNLOADED_RATIO_SELECTOR).next().unwrap(), &DOWNLOADED_RATIO_ATTRIBUT).trim_end().trim_start().replace("\n","").to_string();
        let binding = get_data(document.select(&RATIO_SELECTOR).next().unwrap(), &RATIO_ATTRIBUT);
        let temp = binding.splitn(2, ":").nth(1).unwrap();
        result.percentage = temp.trim_end().trim_start().parse::<f32>().unwrap();
        result
    }

}