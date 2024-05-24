pub mod data_struct;
pub(crate) mod selector;
pub mod yggtorrent_client;
pub mod yggtorrent_params;
pub(crate) mod tracker_list;

pub use reqwest::blocking::*;

#[cfg(test)]
mod tests {
    use crate::yggtorrent_client::YggClient;
    use crate::yggtorrent_params::{YggCategory, YggOrder, YggOrderElement, YggParams, YggSaison, YggSubCategory};

    #[tokio::test]
    async fn it_works() {

        let mut client = YggClient::new("tommot67".to_string(), "20*PQ-mz".to_string()).await;

        let mut options = YggParams::default();
        options.category = YggCategory::FilmVideo;
        options.subcategory = YggSubCategory::SerieTV;
        options.order = Some((YggOrder::Ascendant, YggOrderElement::TelechargementComplet));
        options.other = Some(options.create_other_serie(Some(vec![YggSaison::Num(1), YggSaison::Num(2)]), None)); //Some(options)
        let result = client.search("HPI", None).await;

        for torrent in result.clone() {
            println!("{torrent}");
        }

        println!("Length : {}", result.len());
    }
}
