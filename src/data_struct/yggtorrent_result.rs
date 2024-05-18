use std::fmt::{Debug, Display, Formatter};
use chrono::NaiveDateTime;
use getset::Getters;
use scraper::{ElementRef, Html};
use serde::{Deserialize, Serialize};
use crate::selector::get_data;
use crate::selector::selector_level_2::*;
use crate::selector::selector_level_3::*;


#[derive(Debug, Default, Clone, Getters, Serialize, Deserialize)]
pub struct YggResult {
    #[getset(get = "pub")]
    id: u64,
    #[getset(get = "pub")]
    name: String,
    #[getset(get = "pub")]
    keywords: Vec<String>,
    #[getset(get = "pub")]
    seeders: u32,
    #[getset(get = "pub")]
    leechers: u32,
    #[getset(get = "pub")]
    completed: u32,
    #[getset(get = "pub")]
    size: String,
    #[getset(get = "pub")]
    info_hash: String,
    #[getset(get = "pub")]
    uploader: String,
    #[getset(get = "pub")]
    uploaded: NaiveDateTime,
    #[getset(get = "pub")]
    download_link: String,
    #[getset(get = "pub")]
    files: Vec<YggResultFile>,
    #[getset(get = "pub")]
    nfo_link: String,
}

impl YggResult {
    pub(crate) fn scrape(document: Html) -> YggResult {
        let mut result = YggResult::default();
        result.name= get_data(document.select(&NAME_SELECTOR).next().unwrap(), &NAME_ATTRIBUT).trim_start().trim_end().to_string();
        for keyword in document.select(&KEYWORD_SELECTOR) {
            result.keywords.push(get_data(keyword, &KEYWORD_ATTRIBUT));
        }
        result.seeders = get_data(document.select(&SEEDERS_SELECTOR).next().unwrap(), &SEEDERS_ATTRIBUT).replace(" ", "").parse::<u32>().unwrap();
        result.leechers = get_data(document.select(&LEECHERS_SELECTOR).next().unwrap(), &LEECHERS_ATTRIBUT).replace(" ", "").parse::<u32>().unwrap();
        result.completed = get_data(document.select(&COMPLETED_SELECTOR).next().unwrap(), &COMPLETED_ATTRIBUT).replace(" ", "").parse::<u32>().unwrap();
        result.size = get_data(document.select(&SIZE_SELECTOR).next().unwrap(), &SIZE_ATTRIBUT);
        result.info_hash = get_data(document.select(&HASH_SELECTOR).next().unwrap(), &HASH_ATTRIBUT);
        let temp = document.select(&UPLOADER_SELECTOR).next();
        if temp.is_some() {
            result.uploader = get_data(temp.unwrap(), &UPLOADER_ATTRIBUT);
        }
        else {
            result.uploader = "Pirate Anonyme".to_string();
        }
        result.uploaded = NaiveDateTime::parse_from_str(get_data(document.select(&UPLOADED_SELECTOR).next().unwrap(), &UPLOADED_ATTRIBUT).trim_start().trim_end(), "%d/%m/%Y %H:%M").unwrap_or(NaiveDateTime::default());
        result.download_link = get_data(document.select(&DOWNLOAD_LINK_SELECTOR).next().unwrap(), &DOWNLOAD_LINK_ATTRIBUT);
        result.id = result.download_link.splitn(2, '=').nth(1).unwrap().parse::<u64>().unwrap();
        result.nfo_link = format!("https://www3.yggtorrent.cool/engine/get_nfo?torrent={}", result.id);

        result
    }
}

impl YggResult {
    pub(crate) fn set_files(&mut self, files: Vec<YggResultFile>) {
        self.files = files;
    }
}

impl Display for YggResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ID: {}\n", self.id)?;
        write!(f, "Name: {}\n", self.name)?;
        write!(f, "Keywords: {:?}\n", self.keywords)?;
        write!(f, "Seeders: {}\n", self.seeders)?;
        write!(f, "Leechers: {}\n", self.leechers)?;
        write!(f, "Completed: {}\n", self.completed)?;
        write!(f, "Size: {}\n", self.size)?;
        write!(f, "Info Hash: {}\n", self.info_hash)?;
        write!(f, "Uploader: {}\n", self.uploader)?;
        write!(f, "Uploaded: {}\n", self.uploaded)?;
        write!(f, "Download Link: {}\n", self.download_link)?;
        write!(f, "NFO Link: {}\n", self.nfo_link)?;
        write!(f, "File in torrent:\n")?;
        for file in &self.files {
            std::fmt::Display::fmt(&file, f).unwrap();
        }
        // You can add more fields as needed
        Ok(())
    }
}

#[derive(Debug, Default, Clone, Getters, Serialize, Deserialize)]
pub struct YggResultFile {
    #[getset(get = "pub")]
    size: String,
    #[getset(get = "pub")]
    filename: String,
}

impl YggResultFile {
    pub(crate) fn scrape(element: ElementRef) -> YggResultFile {
        let mut result = YggResultFile::default();
        result.size = get_data(element.select(&SIZE_FILE_SELECTOR).next().unwrap(), &SIZE_FILE_ATTRIBUT).trim_start().trim_end().to_string();
        result.filename = get_data(element.select(&NAME_FILE_SELECTOR).next().unwrap(), &NAME_FILE_ATTRIBUT).trim_start().trim_end().to_string();
        result
    }
}

impl Display for YggResultFile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "\t\tsize: {}\t", self.size)?;
        write!(f, "|\tname: {}\n", self.filename)?;
        Ok(())
    }
}

#[derive(Deserialize, Debug)]
pub(crate) struct HtmlContent {
    pub(crate) html: String,
}


