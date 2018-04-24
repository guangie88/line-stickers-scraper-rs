extern crate failure;
extern crate regex;
extern crate reqwest;
extern crate scraper;
extern crate structopt;
#[macro_use]
extern crate structopt_derive;
#[macro_use]
extern crate vlog;

use regex::Regex;
use reqwest::Client;
use scraper::{Html, Selector};
use std::io::Read;
use std::path::PathBuf;
use std::str::FromStr;
use structopt::StructOpt;

type Result<T> = std::result::Result<T, failure::Error>;

const STICKER_PARSE_CSS: &str = "li > div > span[style]";
const STYLE: &str = "style";

const IMAGE_URL_PARSE_RE: &str =
    r".*background-image:url\((.+?);compress=true\).*";
const IMAGE_NUM_SUB_PARSE_RE: &str = r"/(\d+)/";

#[derive(StructOpt, Debug)]
#[structopt(name = "line-stickers-scraper-conf")]
/// Configuration for line-stickers-scraper
struct Conf {
    /// LINE Sticker URL to scrap from
    /// e.g. <https://store.line.me/stickershop/product/1111425/en>
    url: String,

    #[structopt(short = "v", parse(from_occurrences))]
    /// Verbose flag (-v, -vv, -vvv)
    verbose: u8,
}

fn run(conf: &Conf) -> Result<()> {
    vlog::set_verbosity_level(conf.verbose as usize);

    let image_url_parse_re = Regex::new(IMAGE_URL_PARSE_RE)?;
    let image_num_sub_parse_re = Regex::new(IMAGE_NUM_SUB_PARSE_RE)?;

    let client = Client::new();

    let mut resp = client.get(&conf.url).send()?;
    let mut body = String::new();
    resp.read_to_string(&mut body)?;

    v1!("Status: {}", resp.status());

    let html = Html::parse_document(&body);
    let sel = Selector::parse(STICKER_PARSE_CSS).unwrap();

    let url_output_paths = html.select(&sel)
        .filter_map(|sel| sel.value().attr(STYLE))
        .map(|style| {
            let cap = image_url_parse_re.captures(style).unwrap();
            cap[1].to_owned()
        })
        .map(|image_url| {
            let output_path = {
                let cap = image_num_sub_parse_re
                    .captures(&image_url)
                    .unwrap();

                let mut p = PathBuf::from_str(&cap[1]).unwrap();
                p.set_extension("png");
                p
            };

            (image_url, output_path)
        });

    for (image_url, output_path) in url_output_paths {
        v1!("{} -> {:?}", image_url, output_path);
    }

    Ok(())
}

fn main() {
    let conf = Conf::from_args();

    match run(&conf) {
        Ok(_) => v1!("line-stickers-scraper COMPLETED!"),
        Err(e) => ve0!("{}", e),
    }
}
