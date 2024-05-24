use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::sync::Arc;
use futures::executor;
use percent_encoding::{NON_ALPHANUMERIC, utf8_percent_encode};
use reqwest::Client;

use crate::data_struct::yggtorrent_cookie::YggCookie;

use scraper::{Html, Selector};
use tokio::task;
use tokio::task::{JoinHandle, spawn_blocking};
use crate::data_struct::yggtorrent_ratio::YggRatio;
use crate::data_struct::yggtorrent_result::{HtmlContent, YggResult, YggResultFile};
use crate::yggtorrent_params::YggParams;
use crate::selector::get_data;
use crate::selector::selector_level_1::*;
use crate::tracker_list::TRACKERS;

const WEBSITE_BASE_URL: &'static str = "https://www.ygg.re/";
const USER_AGENT: &'static str = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/125.0.0.0 Safari/537.36";

#[derive(Debug, Clone)]
pub struct YggClient {
    username: String,
    password: String,
    cookies: Vec<YggCookie>,
    last_url: String,
    result: Vec<YggResult>,
    ratio: YggRatio,
}

#[allow(dead_code)]

impl YggClient {

    pub async fn new(username: String, password: String) -> YggClient {
        let mut temp = YggClient {username, password, cookies: Vec::new() , last_url: "".to_string() , result: vec![], ratio: YggRatio::default() };
        temp.login().await.expect("TODO: panic message");
        temp
    }

    pub fn change_username(&mut self, username: String) {
        self.username = username;
        executor::block_on(self.login()).expect("TODO: panic message");
    }

    pub async fn change_password(&mut self, password: String) {
        self.password = password;
        self.login().await.expect("TODO: panic message");
    }

    pub async fn login(&mut self) -> Result<String, String> {

        let temp = WEBSITE_BASE_URL.to_owned() + "auth/process_login";
        let login_url = temp.as_str();

        let mut params = HashMap::new();
        params.insert("id", self.username.clone());
        params.insert("pass", self.password.clone());

        let query_login = Client::new()
              .post(login_url)
              .header("sec-ch-ua", r#"Google Chrome";v="125", "Chromium";v="125", "Not.A/Brand";v="24"#)
              .header("User-Agent", USER_AGENT)
              .form(&params)
              .send()
              .await
              .unwrap();



        let status  = query_login.status();
        if status != 200 {
            return Err("Status code is ".to_string() + status.to_string().as_str());
        }

        let cookie = query_login.headers().get("set-cookie");
        println!("{:?}", cookie.unwrap().clone());
        if cookie.is_some() {
            let mut temp = YggCookie::new();
            temp.parse(cookie.unwrap().to_str().unwrap());
            self.cookies.push(temp);
        }
        else {
            return Err("Not cookie found !".to_string());
        }

        let mut temp = "User ".to_owned();
        temp.push_str(&*self.username);
        temp.push_str(" login");

        Ok(temp)
    }

    pub async fn get_ratio(&mut self) -> Result<YggRatio, ()> {

        let html = Client::new().
            get(WEBSITE_BASE_URL.to_owned()).header("User-Agent", USER_AGENT).header("Cookie", &self.work_cookies().unwrap()).send().await.unwrap().text().await.unwrap();

        let document = Html::parse_document(html.as_str());

        let ratio = YggRatio::scrape(document);

        self.ratio = ratio.clone();

        Ok(ratio)
    }

    pub async fn search(&mut self, name: &str, options: Option<YggParams>) -> Vec<YggResult> {

        let mut search_url = WEBSITE_BASE_URL.to_owned() + "engine/search?name=" + name + "&do=search";

        if options.is_some() {
            let options = options.unwrap();
            search_url = options.concat_to_url(search_url.as_str()).to_string();
        }
        else {
            let options = YggParams::default();
            search_url = options.concat_to_url(search_url.as_str()).to_string();
        }

        println!("{:?}", search_url);

        if self.last_url != search_url {
            self.last_url = search_url.to_string();
            self.result.clear();
            self.scrape_level_1(search_url).await;
        }

        self.result.clone()

    }

    pub fn download_torrent(&mut self, torrent: YggResult, path: String) -> Result<(), String> {

        let bytes = executor::block_on(async {
            Client::new().
                get(torrent.download_link()).header("User-Agent", USER_AGENT).header("Cookie", &self.work_cookies().unwrap()).send().await.unwrap().bytes().await.unwrap()
        });

        if bytes.eq(&"Vous devez vous connecter pour télécharger un torrent".as_bytes().to_vec()) {
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

    pub fn get_last_url(&self) -> String {
        self.last_url.clone()
    }

    pub fn get_last_result(&self) -> Vec<YggResult> {
        self.result.clone()
    }

    pub fn get_last_ratio(&self) -> YggRatio {
        self.ratio.clone()
    }

    fn work_cookies(&mut self) -> Result<String, ()> {
        let mut cook = self.create_cookies();

        if cook.is_err() {
            executor::block_on(self.login()).expect("TODO: panic message");
            cook = self.create_cookies();
        }

        if cook.is_err() {
            return Err(());
        }

        Ok(cook.unwrap())
    }

    fn create_cookies(&self) -> Result<String, &str> {

        let mut temp: String = "".to_string();

        for yggcookie in &self.cookies {
            let data = yggcookie.get_cookie();

            if data.is_err() {
                return Err("Invalid cookie");
            }

            temp.push_str(data.unwrap());
            temp.push_str("; ");
        }

        //temp.truncate(temp.len() - 2); //remove last "; " //work without

        Ok(temp.to_owned())
    }

    async fn scrape_level_1(&mut self, url: String) {
        let mut page = 0;
        let mut local_url = url.clone();

        let cookie = Arc::new(self.work_cookies().unwrap());

        let mut handles: Vec<JoinHandle<YggResult>> = vec![];

        loop {

            let html = Client::new().
                get(&*local_url).header("User-Agent", USER_AGENT).header("Cookie", cookie.as_str()).send().await.unwrap().text().await.unwrap();


            let document = Html::parse_document(html.as_str());

            let elements = document.select(&LINK_TORRENT_PAGE_SELECTOR);


            if elements.clone().next().is_some() {
                page += 1;
                local_url = format!("{}&page={}", url.clone(), page * 50).to_string();

                for element in elements {
                    let element_data = get_data(element, LINK_TORRENT_PAGE_ATTRIBUT.clone());
                    let cookie_clone = Arc::clone(&cookie);

                    let handle: JoinHandle<YggResult> = tokio::spawn( async move {
                        Self::scrape_level_2(element_data, cookie_clone.as_str()).await
                    });
                    handles.push(handle);
                }
            } else {
                break;
            }
        }

        for handle in handles {
            match handle.await {
                Ok(data) => self.result.push(data),
                Err(_) => {}
            }
        }
    }

    async fn scrape_level_2(url: String, cookie: &str) -> YggResult  {

        let html = reqwest::Client::new().
            get(&*url.clone()).header("User-Agent", USER_AGENT).header("Cookie", cookie).send().await.unwrap().text().await.unwrap();

        let document = Html::parse_document(html.as_str());

        let mut yggresult = YggResult::scrape(document);

        yggresult.set_files(Self::scrape_level_3(*yggresult.id(), cookie).await.unwrap());

        yggresult

    }

    async fn scrape_level_3(id: u64, cookie: &str ) -> Result<Vec<YggResultFile>,()> {
        let mut result: Vec<YggResultFile> = Vec::new();

        let url = format!("{}engine/get_files?torrent={}",WEBSITE_BASE_URL.to_owned() , id);
        let temp = reqwest::Client::new().
            get(&*url.clone()).header("User-Agent", USER_AGENT).header("Cookie", cookie).send().await.unwrap().text().await.unwrap();

        let content: HtmlContent = serde_json::from_str(&temp).unwrap();

        let html = content.html.replace("\\u003C", "<").replace("\\u003E", ">").replace("\r", "").replace("\n","").replace("\t", "");

        let document = Html::parse_fragment(&*html);

        for element in document.select(&Selector::parse("table > tbody > tr").unwrap()) {
            result.push(YggResultFile::scrape(element));
        }

        Ok(result)

    }

}
