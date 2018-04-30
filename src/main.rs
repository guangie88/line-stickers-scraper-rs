#![cfg_attr(feature = "cargo-clippy", deny(clippy))]
#![deny(missing_debug_implementations, warnings)]

extern crate cssparser;
#[macro_use]
extern crate failure;
extern crate regex;
extern crate reqwest;
extern crate scraper;
extern crate structopt;
#[macro_use]
extern crate structopt_derive;
#[macro_use]
extern crate vlog;

mod error;

use regex::Regex;
use reqwest::{Client, StatusCode};
use scraper::{Html, Selector};
use std::fs::{create_dir_all, File};
use std::io::{Read, Write};
use std::path::PathBuf;
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

    #[structopt(
        short = "o",
        long = "outdir",
        default_value = "output",
        parse(from_os_str)
    )]
    /// Output directory, created if not present
    outdir: PathBuf,

    #[structopt(short = "v", parse(from_occurrences))]
    /// Verbose flag (-v, -vv, -vvv)
    verbose: u8,
}

fn run(conf: &Conf) -> Result<()> {
    vlog::set_verbosity_level(conf.verbose as usize);

    let image_url_parse_re = Regex::new(IMAGE_URL_PARSE_RE)?;
    let image_num_sub_parse_re = Regex::new(IMAGE_NUM_SUB_PARSE_RE)?;

    create_dir_all(&conf.outdir)?;

    let client = Client::new();

    let mut resp = client.get(&conf.url).send()?;

    let mut body = String::new();
    resp.read_to_string(&mut body)?;

    v1!("Status: {}", resp.status());

    let html = Html::parse_document(&body);

    let sel = Selector::parse(STICKER_PARSE_CSS)
        .map_err(|e| error::ParseError::from(e))?;

    // formulate the image URLs and output names
    let url_output_paths = html.select(&sel)
        .filter_map(|sel| sel.value().attr(STYLE))
        .map(|style| {
            let image_url_cap = image_url_parse_re.captures(style).unwrap();
            image_url_cap[1].to_owned()
        })
        .filter_map(|image_url| {
            let image_id = image_num_sub_parse_re
                .captures(&image_url)
                .and_then(|image_cap| image_cap.get(1))
                .map(|image_id| image_id.as_str().to_owned());

            match image_id {
                Some(image_id) => Some((image_url, image_id)),
                None => {
                    ve0!(
                        "Unable to capture unique ID from image URL: {}",
                        image_url
                    );

                    None
                }
            }
        })
        .map(|(image_url, image_id)| {
            let mut output_name = PathBuf::new();
            output_name.push(image_id);
            output_name.set_extension("png");

            (image_url, output_name)
        });

    for (image_url, output_name) in url_output_paths {
        let output_path = {
            let mut output_path = conf.outdir.clone();
            output_path.push(output_name);
            output_path
        };

        v2!("Downloading {} -> {:?}", image_url, output_path);

        let mut resp = client.get(&image_url).send()?;

        if resp.status() == StatusCode::Ok {
            let mut output_file = File::create(&output_path)?;

            let mut buf = vec![];
            resp.read_to_end(&mut buf)?;
            output_file.write_all(&buf)?;
        } else {
            let mut body = String::new();
            resp.read_to_string(&mut body)?;
            ve0!("Download error: {}", body);
        }
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
