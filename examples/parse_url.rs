fn main() {
    let response = reqwest::blocking::get(
        "https://www.hxedu.com.cn/hxedu/w/inputVideo.do?qid=5a79a018764b6543017650769fc75dd1",
    )
    .unwrap()
    .text()
    .unwrap();

    let document = scraper::Html::parse_document(&response);

    //    let title_selector = scraper::Selector::parse("div.allbox").unwrap();

    //let mut titles = document.select(&title_selector).map(|x| x.inner_html());
    //println!("{:?}", titles.next().unwrap());

    let div_selector = scraper::Selector::parse("div.box").unwrap();
    let divs = document.select(&div_selector).next().unwrap();
    //println!("{:?}", divs.value());
    let video_selector = scraper::Selector::parse("video").unwrap();
    for video in divs.select(&video_selector) {
        //println!("{:?}", video.value());
        let download_url = video.value().attr("src").unwrap();
        println!("{:?}", download_url);
    }
}
