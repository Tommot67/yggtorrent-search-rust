use chrono::{Duration, NaiveDateTime, Utc};

#[derive(Debug, Clone)]
pub struct YggCookie {
    data: String,
    date_expire: NaiveDateTime
}

impl YggCookie {
    pub(crate) fn new() -> YggCookie {
        YggCookie { data: String::from("none"), date_expire: NaiveDateTime::from(Utc::now().naive_utc()) }
    }

    pub(crate) fn parse(self: &mut YggCookie, data: &str) {

        let datas = data.split(";")
            .collect::<Vec<&str>>();

        self.data = datas[0].to_string();

        let expires: Vec<&str> = datas
            .iter()
            .filter(|&&s| s.contains("Expires") || s.contains("expires"))
            .copied()
            .collect();

        self.date_expire = NaiveDateTime::parse_from_str(&*expires.first().unwrap().split("=").collect::<Vec<&str>>()[1].trim_start().replace("-", " "), "%a, %d %b %Y %H:%M:%S GMT").unwrap();
    }

    pub(crate) fn check_validity(&self) -> bool {
        // add to have 30 sec margin
        self.date_expire.gt(&((Utc::now() +  Duration::seconds(30)).naive_utc()))
    }

    pub(crate) fn get_cookie(&self) -> Result<&str, &str> {
        if self.check_validity() {
            Ok(self.data.as_str())
        } else {
            Err("Le cookie n'est plus valid.")
        }
    }

}
