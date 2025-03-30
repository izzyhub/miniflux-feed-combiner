use axum::{routing::get, Router};

use miniflux_api::{
    models::{EntryStatus, OrderBy},
    MinifluxApi,
};
use rss::{ChannelBuilder, Item, ItemBuilder};
use url::Url;
mod error;
use error::Result;

#[axum::debug_handler]
async fn combined_feed() -> Result<String> {
    let miniflux_url: Url =
        Url::parse(&std::env::var("MINIFLUX_URL").expect("MINIFLUX_URL not set"))
            .expect("Bad miniflux URL");
    let site_url: Url =
        Url::parse(&std::env::var("SITE_URL").expect("SITE_URL not set")).expect("Bad site URL");
    let miniflux_api_key: String =
        std::env::var("MINIFLUX_API_KEY").expect("MINIFLUX_API_KEY not set");

    let client = reqwest::Client::new();

    let miniflux = MinifluxApi::new_from_token(&miniflux_url, miniflux_api_key);
    let entries = miniflux
        .get_entries(
            Some(EntryStatus::Unread),
            None,
            Some(100),
            Some(OrderBy::PublishedAt),
            None,
            None,
            None,
            None,
            None,
            None,
            &client,
        )
        .await?;

    let mut channel = ChannelBuilder::default()
        .link(site_url)
        .title("Miniflux combined feed")
        .build();

    let items: Vec<Item> = entries
        .into_iter()
        .map(|entry| {
            ItemBuilder::default()
                .title(entry.title)
                .link(entry.url)
                .content(entry.content)
                .author(entry.author)
                .build()
        })
        .collect();

    channel.set_items(items);

    Ok(channel.to_string())
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(combined_feed));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
