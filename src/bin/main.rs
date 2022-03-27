use check_ha_535583_rs::{get_ha_53583, FeedItem};

fn main() {
    let feed_items: Vec<FeedItem> = get_ha_53583();

    for item in feed_items {
        println!("====================================================");
        println!("Title: {}", item.title);
        println!("Link: {}", item.link);
        println!("Date: {}", item.date);
    }
}
