use reqwest;
use scraper::{Html, Selector};

#[allow(unused_variables)]
#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let response = requestor().await?;

    // parse the html
    let parsed_html = Html::parse_document(&response);

    // select `img` html tags
    let selector = Selector::parse("img").unwrap();

    for selected_node in parsed_html.select(&selector).skip(2).take(5) {
        let chapter_values = selected_node.value().attr("alt").unwrap();
        println!("Chapter: {}", chapter_values);

        // split the `chapter_values` in `chapter_name: String` and `chapter_number: i32`
    }

    Ok(())
}

async fn requestor() -> Result<String, reqwest::Error> {
    let response = reqwest::get("https://onepiecechapters.com/")
        .await?
        .text()
        .await?;

    Ok(response)
}

#[allow(dead_code)]
async fn notify() {
    unimplemented!()
}
