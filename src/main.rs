use scraper::{Html, Selector};
    

async fn get_request(url: &str) -> Result<reqwest::Response, reqwest::Error> {
    let resp = reqwest::get(url).await;
    resp
}

#[tokio::main]
async fn main(){
    let url = "https://dictionary.cambridge.org/dictionary/english/table";

    // TODO: Proper error handling
    let resp = get_request(url).await.unwrap();
    assert!(resp.status().is_success());

    let body = resp.text().await.unwrap();

    // Parse as document
    let fragment = Html::parse_document(&body);

    let descs = Selector::parse("#page-content > div.page > div:nth-child(1) > div.link > div > div.di-body > div > div > div:nth-child(1) > div.pos-body > div:nth-child(1) > div.sense-body.dsense_b > div:nth-child(1) > div.hflxrev.hdf-xs.hdb-s.hdf-l > div.hflx1 > div.ddef_h > div").unwrap();

    // println!("{:#?}", descs);

    for desc in fragment.select(&descs) {
	let desc_txt = desc.text().collect::<Vec<_>>();
	println!("{:#?}", desc_txt.join(""));
    }

}

