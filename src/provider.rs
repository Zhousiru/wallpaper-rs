use regex::Regex;

use crate::downloader;

fn get_unsplash_topic_image_url(html: &str) -> &str {
    let re = Regex::new(r#"(?mi)<source srcset="(.*?)\?.*?/>"#).unwrap();
    re.captures(html).unwrap().extract::<1>().1[0]
}

pub fn provide_unsplash_topic_image(topic: Option<&str>) -> String {
    let mut url = String::from("https://unsplash.com");

    if topic != None {
        url.push_str("/t/");
        url.push_str(topic.unwrap());
    }

    let html = downloader::get_text(&url);
    let image_url = get_unsplash_topic_image_url(&html);

    downloader::download_image(image_url)
}
