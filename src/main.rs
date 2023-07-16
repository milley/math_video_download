use std::{path::Path, env};

use math_vedio_download::{file_input::read_lines, downloader::download_file};


fn check_file_exists(path: &str) -> bool {
    Path::new(path).exists()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} input_file", args[0].to_string());
        panic!("Usage: math_video_download input_file");
    }
    let file_path = &args[1];

    let class_urls = read_lines(file_path);
    for class_url in class_urls {
        let client = reqwest::Client::new();
        let response = reqwest::blocking::get(
            class_url
        )
        .unwrap()
        .text()
        .unwrap();

        let document = scraper::Html::parse_document(&response);

        let title_selector = scraper::Selector::parse("div.allbox").unwrap();
        let title = document.select(&title_selector).map(|x| x.inner_html()).next().unwrap();
        println!("title: {}", title);

        let div_selector = scraper::Selector::parse("div.box").unwrap();
        let divs = document.select(&div_selector).next().unwrap();
        let video_selector = scraper::Selector::parse("video").unwrap();
        for video in divs.select(&video_selector) {
            let download_url = video.value().attr("src").unwrap();
            println!("download_url: {:?}", download_url);

            let download_filename = "./download/".to_owned() + title.as_str() + ".mp4";
            println!("download_filename: {:?}", download_filename);

            if check_file_exists(download_filename.as_str()) {
                println!("{} is already downloaded", download_filename);
                continue;
            }
            download_file(&client, &download_url, &download_filename).await.unwrap()
        }
    }

    Ok(())
}
