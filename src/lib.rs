mod data_struct;
pub mod selector;
pub mod yggtorrent_client;
pub mod yggtorrent_params;
mod tracker_list;


#[cfg(test)]
mod tests {
    use crate::yggtorrent_params::{YggCategory, YggOrder, YggOrderElement, YggParams, YggSubCategory};
    use crate::yggtorrent_client::YggClient;

    #[tokio::test]
    async fn it_works() {
        let mut client = YggClient::new("admin".to_string(), "adminadmin".to_string()).await;
        let mut options = YggParams::default();
        options.category = YggCategory::Ebook;
        options.subcategory = YggSubCategory::Livres;
        options.order = Some((YggOrder::Ascendant, YggOrderElement::TelechargementComplet));
        let result = client.search("rust", Some(options)).await;

        for torrent in result.clone() {
            println!("{torrent}");
        }

        println!("{:?}", client.download_torrent(result.last().unwrap().clone(), "./torrent/test.torrent".to_string()).await.unwrap()); //error login
        println!("{}", YggClient::create_magnet_link(result.last().unwrap().clone()));
        
    }
}
