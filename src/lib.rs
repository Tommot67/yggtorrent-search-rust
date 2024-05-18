pub mod data_struct;
pub(crate) mod selector;
pub mod yggtorrent_client;
pub mod yggtorrent_params;
pub(crate) mod tracker_list;


#[cfg(test)]
mod tests {
    use crate::yggtorrent_client::YggClient;
    use crate::yggtorrent_params::{YggCategory, YggOrder, YggOrderElement, YggParams, YggSubCategory};

    #[tokio::test]
    async fn it_works() {
        let mut client = YggClient::new("tommot67".to_string(), "20*PQ-mz".to_string());
        let mut options = YggParams::default();
        options.category = YggCategory::Ebook;
        options.subcategory = YggSubCategory::Livres;
        options.order = Some((YggOrder::Ascendant, YggOrderElement::TelechargementComplet));
        let result = client.search("rust", Some(options));
        for torrent in result.clone() {
            println!("{torrent}");
        }

        println!("{:?}", client.download_torrent(result.last().unwrap().clone(), "./torrent/test.torrent".to_string()).unwrap()); //error login
        println!("{}", YggClient::create_magnet_link(result.last().unwrap().clone()));
    }
}
