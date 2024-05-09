use std::collections::HashMap;
use std::io::Error;
use std::fs;
use std::path::Path;
use async_recursion::async_recursion;
use getset::Getters;
use percent_encoding::{CONTROLS, NON_ALPHANUMERIC, utf8_percent_encode};

use crate::data_struct::yggtorrent_cookie::YggCookie;

use reqwest;
use reqwest::{Client, StatusCode};
use scraper::{Html, Selector};
use undetected_chromedriver::{Chrome, UndetectedChrome, UndetectedChromeUsage};
use bytes::Bytes;
use crate::data_struct::yggtorrent_ratio::YggRatio;
use crate::data_struct::yggtorrent_result::{HtmlContent, YggResult, YggResultFile};
use crate::yggtorrent_params::YggParams;
use crate::selector::get_data;
use crate::selector::selector_level_1::*;
use crate::tracker_list::TRACKERS;

const WEBSITE_BASE_URL: &'static str = "https://www3.yggtorrent.cool/";
const USER_AGENT: &'static str = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/124.0.0.0 Safari/537.36";

#[derive(Debug, Clone, Getters)]
pub struct YggClient {
    website: &'static str,
    username: String,
    password: String,
    clearence_cookie: Vec<YggCookie>,
    #[getset(get = "pub")]
    last_url: String,
    #[getset(get = "pub")]
    result: Vec<YggResult>,
    #[getset(get = "pub")]
    ratio: YggRatio,
    client: Client,
}

impl YggClient {

    pub async fn new(username: String, password: String) -> YggClient {
        let mut temp = YggClient {website: WEBSITE_BASE_URL,  username, password, clearence_cookie: Vec::new() , last_url: "".to_string() , result: Vec::new(), ratio: YggRatio::default() , client:  Client::new() };
        temp.get_clearence().await;
        temp
    }

    pub async fn change_username(&mut self, username: String) {
        self.username = username;
        self.login().await.expect("Can't'login");
    }

    pub async fn change_password(&mut self, password: String) {
        self.password = password;
        self.login().await.expect("Can't login");
    }

    #[async_recursion]
    pub async fn login(&mut self) -> Result<String, String> {

        let temp = self.website.to_string() + "user/login";
        let login_url = temp.as_str();

        let mut params = HashMap::new();
        params.insert("id", self.username.clone());
        params.insert("pass", self.password.clone());

        let query_login = self.client
            .post(login_url)
            .header("Cookie", self.work_clearence().await.unwrap())
            .header("User-Agent", USER_AGENT)
            .form(&params)
            .send()
            .await
            .unwrap();

        let status  = query_login.status();
        if status != StatusCode::OK {
            return Err("Status code is ".to_string() + status.as_str());
        }

        let mut temp = "User ".to_owned();
        temp.push_str(&*self.username);
        temp.push_str(" login");

        Ok(temp)
    }

    pub(crate) async fn get_clearence(&mut self) {

        let mut temp = UndetectedChrome::new(UndetectedChromeUsage::CLOUDFLAREBYPASSER).await;
        temp.bypass_cloudflare(self.website).await.unwrap();

        let webdriver =  temp.borrow();

        match webdriver.get_all_cookies().await {
            Ok(cookies) => {
                self.clearence_cookie.clear();
                for cookie in cookies {
                    let mut yggcookie = YggCookie::new();
                    yggcookie.parse(cookie.to_string().as_str());
                    self.clearence_cookie.push(yggcookie);
                }
            },
            Err(e) => {
                println!("Error: {}", e);
                temp.kill().await;
            }
        }
        temp.kill().await;
        
        dbg!(self.login().await);
    }

    pub(crate) fn create_clearence_cookie(&self) -> Result<String, &str> {

        let mut temp: String = "".to_string();

        for yggcookie in &self.clearence_cookie {
            let data = yggcookie.get_cookie();

            if data.is_err() {
                return Err("Invalid cookie");
            }

            temp.push_str(data.unwrap());
            temp.push_str("; ");
        }

        temp.truncate(temp.len() - 2); //remove last "; "

        Ok(temp.to_owned())
    }

    pub(crate) async fn work_clearence(&mut self) -> Result<String, ()> {
        let mut cook = self.create_clearence_cookie();

        if cook.is_err() {
            self.get_clearence().await;
            cook = self.create_clearence_cookie();
        }

        if cook.is_err() {
            return Err(());
        }

        Ok(cook.unwrap())
    }

    pub async fn get_ratio(&mut self) -> Result<YggRatio, ()> {

        let html = self.client.
            get(self.website).header("User-Agent", USER_AGENT).header("Cookie", self.work_clearence().await.unwrap()).send().await.unwrap().text().await.unwrap();

        let document = Html::parse_document(html.as_str());

        fs::write("/home/trott/Documents/Perso_dev/yggtorrent-search/doc/ratio.html", document.html()).unwrap();

        let ratio = YggRatio::scrape(document);

        self.ratio = ratio.clone();

        Ok(ratio)
    }

    pub async fn search(&mut self, name: &str, options: Option<YggParams>) -> Vec<YggResult> {

        let mut search_url = self.website.to_string() + "engine/search?name=" + name + "&do=search";

        if options.is_some() {
            let options = options.unwrap();
            search_url = options.concat_to_url(search_url.as_str()).to_string();

            println!("{}", search_url);
        }

        if self.last_url != search_url {
            self.last_url = search_url.to_string();
            self.result.clear();
            self.scrape_level_1(search_url).await.unwrap();
        }

        self.result.clone()

    }

    async fn scrape_level_1(&mut self, mut url: String) -> Result<(), ()> {

        loop {
            let html = self.client.
                get(url.clone()).header("User-Agent", USER_AGENT).header("Cookie", self.work_clearence().await.unwrap()).send().await.unwrap().text().await.unwrap();

            let document = Html::parse_document(html.as_str());

            for element in document.select(&LINK_TORRENT_PAGE_SELECTOR) {
                self.scrape_level_2(get_data(element, LINK_TORREN_PAGE_ATTRIBUT.clone())).await.unwrap();
            }

            let element = document.select(&LINK_NEXT_PAGE_SELECTOR).next();

            if element.is_some() {
                url = get_data(element.unwrap(), LINK_TORREN_PAGE_ATTRIBUT.clone());
            }
            else {
                return Ok(());
            }
        }
    }

    async fn scrape_level_2(&mut self, url: String) -> Result<(), ()> {

        let html = self.client.
            get(url.clone()).header("User-Agent", USER_AGENT).header("Cookie", self.work_clearence().await.unwrap()).send().await.unwrap().text().await.unwrap();

        let document = Html::parse_document(html.as_str());

        let mut yggresult = YggResult::scrape(document);

        yggresult.set_files(self.scrape_level_3(*yggresult.id()).await.unwrap());

        self.result.push(yggresult);

        Ok(())

    }

    async fn scrape_level_3(&mut self, id: u64) -> Result<Vec<YggResultFile>,()> {
        let mut result: Vec<YggResultFile> = Vec::new();

        let url = format!("https://www3.yggtorrent.cool/engine/get_files?torrent={}", id);
        let temp = self.client.
            get(url.clone()).header("User-Agent", USER_AGENT).header("Cookie", self.work_clearence().await.unwrap()).send().await.unwrap().text().await.unwrap();

        let content: HtmlContent = serde_json::from_str(&temp).unwrap();
        let html = content.html.replace("\\u003C", "<").replace("\\u003E", ">").replace("\r", "").replace("\n","").replace("\t", "");

        let document = Html::parse_fragment(&*html);

        for element in document.select(&Selector::parse("table > tbody > tr").unwrap()) {
            result.push(YggResultFile::scrape(element));
        }

        Ok(result)

    }

    pub async fn download_torrent(&mut self, torrent: YggResult, path: String) -> Result<(), String> {

        let bytes =  self.client.
            get(torrent.download_link()).header("User-Agent", USER_AGENT).header("Cookie", self.work_clearence().await.unwrap()).send().await.unwrap().bytes().await.unwrap();

        if bytes.eq(&Bytes::from_static("Vous devez vous connecter pour télécharger un torrent".as_bytes())) {
            return Err("Please login for download torrent or use magnet".to_string())
        }

        let filename = path.split("/").last().unwrap();
        let replaced_path = path.replace(filename, "");
        let subpath = replaced_path.as_str();

        if !Path::new(subpath).exists() {
            fs::create_dir_all(subpath).expect("Directory structure creation not work!");
        }

        let write = fs::write(path, bytes);

        if write.is_err() {
            return Err(write.err().unwrap().to_string());
        }

        Ok(())
    }

    pub fn create_magnet_link(torrent: YggResult) -> String {

        let mut magnet_url =  format!("magnet:?xt=urn:btih:{}&dn={}", torrent.info_hash(), utf8_percent_encode(torrent.name(), NON_ALPHANUMERIC));

        for tracker in TRACKERS.lock().unwrap().to_vec() {
            magnet_url.push_str(format!("&tr={}", tracker).as_str());
        }

        magnet_url
    }

}