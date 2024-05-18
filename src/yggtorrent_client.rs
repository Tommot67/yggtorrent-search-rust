use std::fs;
use std::path::Path;
use percent_encoding::{NON_ALPHANUMERIC, utf8_percent_encode};

use crate::data_struct::yggtorrent_cookie::YggCookie;

use scraper::{Html, Selector};
use bytes::Bytes;
use ureq::Agent;
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
    website: &'static str,
    username: String,
    password: String,
    cookies: Vec<YggCookie>,
    last_url: String,
    result: Vec<YggResult>,
    ratio: YggRatio,
    client: Agent,
}

unsafe impl Send for YggClient {

}

unsafe impl Sync for YggClient {

}

#[allow(dead_code)]

impl YggClient {

    pub fn new(username: String, password: String) -> YggClient {
        let mut temp = YggClient {website: WEBSITE_BASE_URL,  username, password, cookies: Vec::new() , last_url: "".to_string() , result: vec![], ratio: YggRatio::default() , client:  Agent::new() };
        temp.login().expect("Can't login");
        temp
    }

    pub async fn change_username(&mut self, username: String) {
        self.username = username;
        self.login().expect("Can't'login");
    }

    pub async fn change_password(&mut self, password: String) {
        self.password = password;
        self.login().expect("Can't login");
    }

    pub fn login(&mut self) -> Result<String, String> {

        let temp = self.website.to_string() + "auth/process_login";
        let login_url = temp.as_str();

        let binding1 = self.username.clone();
        let binding2 = self.password.clone();
        let params = vec![
            ("id", binding1.as_str()),
            ("pass", binding2.as_str()),
        ];

        let query_login = self.client
            .post(login_url)
            .set("sec-ch-ua", r#"Google Chrome";v="125", "Chromium";v="125", "Not.A/Brand";v="24"#)
            .set("User-Agent", USER_AGENT)
            .send_form(&params)
            .unwrap();

        let status  = query_login.status();
        if status != 200 {
            return Err("Status code is ".to_string() + status.to_string().as_str());
        }

        let cookie = query_login.header("set-cookie");
        println!("{:?}", cookie.unwrap());
        if cookie.is_some() {
            let mut temp = YggCookie::new();
            temp.parse(cookie.unwrap());
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

        let html = self.client.
            get(self.website).set("User-Agent", USER_AGENT).set("Cookie", &self.work_cookies().unwrap()).call().unwrap().into_string().unwrap();

        let document = Html::parse_document(html.as_str());

        let ratio = YggRatio::scrape(document);

        self.ratio = ratio.clone();

        Ok(ratio)
    }

    pub fn search(&mut self, name: &str, options: Option<YggParams>) -> Vec<YggResult> {

        let mut search_url = self.website.to_string() + "engine/search?name=" + name + "&do=search";

        if options.is_some() {
            let options = options.unwrap();
            search_url = options.concat_to_url(search_url.as_str()).to_string();
        }
        else {
            let options = YggParams::default();
            search_url = options.concat_to_url(search_url.as_str()).to_string();
        }

        if self.last_url != search_url {
            self.last_url = search_url.to_string();
            self.result.clear();
            self.scrape_level_1(search_url);
        }

        self.result.clone()

    }

    pub fn download_torrent(&mut self, torrent: YggResult, path: String) -> Result<(), String> {

        let bytes =  self.client.
            get(torrent.download_link()).set("User-Agent", USER_AGENT).set("Cookie", &self.work_cookies().unwrap()).call().unwrap().into_string().unwrap();

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
            self.login().expect("Login ERROR");
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

    fn scrape_level_1(&mut self, url: String) {
        let mut page = 0;
        let mut local_url = url.clone();
        loop {

            let html = self.client.
                get(&*local_url).set("User-Agent", USER_AGENT).set("Cookie", &self.work_cookies().unwrap()).call().unwrap().into_string().unwrap();


            let document = Html::parse_document(html.as_str());

            let elements = document.select(&LINK_TORRENT_PAGE_SELECTOR);


            if elements.clone().next().is_some() {
                page += 1;
                local_url = format!("{}&page={}", url.clone(), page * 50).to_string();

                for element in elements {
                    self.scrape_level_2(get_data(element, LINK_TORRENT_PAGE_ATTRIBUT.clone()));
                }
            } else {
                break;
            }
        }
    }

    fn scrape_level_2(&mut self, url: String)  {

        let html = self.client.
            get(&*url.clone()).set("User-Agent", USER_AGENT).set("Cookie", &*self.work_cookies().unwrap()).call().unwrap().into_string().unwrap();

        let document = Html::parse_document(html.as_str());

        let mut yggresult = YggResult::scrape(document);

        yggresult.set_files(self.scrape_level_3(*yggresult.id()).unwrap());

        self.result.push(yggresult);

    }

    fn scrape_level_3(&mut self, id: u64) -> Result<Vec<YggResultFile>,()> {
        let mut result: Vec<YggResultFile> = Vec::new();

        let url = format!("{}engine/get_files?torrent={}",self.website.to_string() , id);
        let temp = self.client.
            get(&*url.clone()).set("User-Agent", USER_AGENT).set("Cookie", &*self.work_cookies().unwrap()).call().unwrap().into_string().unwrap();

        let content: HtmlContent = serde_json::from_str(&temp).unwrap();

        let html = content.html.replace("\\u003C", "<").replace("\\u003E", ">").replace("\r", "").replace("\n","").replace("\t", "");

        let document = Html::parse_fragment(&*html);

        for element in document.select(&Selector::parse("table > tbody > tr").unwrap()) {
            result.push(YggResultFile::scrape(element));
        }

        Ok(result)

    }

}
