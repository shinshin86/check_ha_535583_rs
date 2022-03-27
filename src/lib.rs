use chrono::{DateTime, Duration, FixedOffset};
use rss::{Channel, Item};
use std::cmp::Ordering;

fn fetch_feed_items() -> Result<Vec<Item>, Box<dyn std::error::Error>> {
    let body =
        reqwest::blocking::get("https://a.hatena.ne.jp/shinshin86/rss?gid=535583")?.bytes()?;
    let channel = Channel::read_from(&body[..])?;

    Ok(channel.into_items())
}

pub struct FeedItem {
    pub title: String,
    pub link: String,
    pub date: DateTime<FixedOffset>,
}

pub fn get_ha_53583() -> Vec<FeedItem> {
    let items = fetch_feed_items().unwrap();
    let mut feed_items: Vec<FeedItem> = items
        .iter()
        .map(|item| FeedItem {
            title: item.title().unwrap().to_string(),
            link: item.link().unwrap().to_string(),
            date: DateTime::parse_from_rfc3339(
                &item.dublin_core_ext().unwrap().dates()[0].to_string(),
            )
            .unwrap(),
        })
        .collect();

    feed_items.sort_by(|a, b| {
        let duration: Duration = a.date - b.date;
        if duration.num_milliseconds() > 0 {
            Ordering::Less
        } else if duration.num_milliseconds() == 0 {
            Ordering::Equal
        } else {
            Ordering::Greater
        }
    });

    feed_items
}
