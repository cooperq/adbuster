use scraper::{Html, Selector};
use reqwest;

fn main() {
    // Set the search query
    let query: &str = "mac antivirus protection";

    // Create a reqwest client
    let client: reqwest::blocking::Client = reqwest::blocking::Client::new();

    // Send the search request to Google
    let response: reqwest::blocking::Response = client.get(&format!("https://www.google.com/search?q={}", query))
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; rv:78.0) Gecko/20100101 Firefox/78.0")
        .send()
        .expect("Failed to send request");

    // Read the response body
    let body: String = response.text().expect("Failed to read response body");

    // Parse the HTML response
    let document: Html = Html::parse_document(&body);

    // Define the CSS selector for sponsored search results
    let sponsored_selector: Selector = Selector::parse("div[data-text-ad]").unwrap();

    // Find and extract the URLs for sponsored search results
    let sponsored_urls: Vec<String> = document
        .select(&sponsored_selector)
        .filter_map(|element| {
            let link = element
                .select(&Selector::parse("a").unwrap())
                .next()
                .and_then(|a| a.value().attr("href"));

            link.map(|l| l.to_string())
        })
        .collect();

    // Print the sponsored URLs
    for url in sponsored_urls {
        println!("{}", url);
    }
}
