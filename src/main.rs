#![cfg_attr(feature = "daemon", windows_subsystem = "windows")]

use std::env;

mod desktop;
mod downloader;
mod lockscreen;
mod provider;

pub fn main() {
    let args: Vec<String> = env::args().collect();

    let target = args
        .get(1)
        .expect("Missing target: `desktop`, `lockscreen`, or `both`");

    let mut path = args
        .get(2)
        .expect("Missing image path or provider")
        .to_owned();

    let use_provider = path.starts_with("provider:");

    if use_provider {
        // Using provider.
        let provider = path.strip_prefix("provider:").unwrap().to_owned();

        // Using Unsplash topic image provider.
        if provider.starts_with("unsplash") {
            let topic = provider.strip_prefix("unsplash-");
            path = provider::provide_unsplash_topic_image(topic);
        }

        // `path` remains unchanged. Invalid provider.
        if path.starts_with("provider:") {
            panic!("Invalid provider `{}`", provider)
        }
    }

    match &target[..] {
        "desktop" => unsafe { desktop::set_image(&path) },
        "lockscreen" => lockscreen::set_image(&path),
        "both" => {
            unsafe { desktop::set_image(&path) };
            lockscreen::set_image(&path)
        }
        other => panic!("Invalid target `{}`", other),
    }

    if use_provider {
        // Cleanup.
        downloader::remove_file(&path)
    }
}
