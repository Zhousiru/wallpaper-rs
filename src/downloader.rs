use std::{fs, io::Cursor, time::Duration};

fn get_client() -> reqwest::blocking::Client {
    return reqwest::blocking::ClientBuilder::new()
        .user_agent("wallpaper-rs")
        .timeout(Duration::new(10, 0))
        .build()
        .unwrap();
}

pub fn download_image(url: &str) -> String {
    let data = get_client().get(url).send().unwrap().bytes().unwrap();

    let data_cursor = Cursor::new(&data);
    let format = image::io::Reader::new(data_cursor)
        .with_guessed_format()
        .unwrap()
        .format()
        .unwrap();
    let format_str = format.extensions_str()[1];

    let path = std::env::current_exe()
        .unwrap()
        .parent()
        .unwrap()
        .join(format!("tmp-download.{format_str}"))
        .to_str()
        .unwrap()
        .to_string();

    fs::write(&path, data).unwrap();

    path
}

pub fn get_text(url: &str) -> String {
    get_client().get(url).send().unwrap().text().unwrap()
}

pub fn remove_file(path: &str) {
    fs::remove_file(path).unwrap()
}
