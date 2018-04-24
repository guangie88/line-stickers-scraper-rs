extern crate failure;
extern crate reqwest;
extern crate scraper;
extern crate structopt;
#[macro_use]
extern crate structopt_derive;
#[macro_use]
extern crate vlog;

use scraper::Selector;
use std::io::Read;
use structopt::StructOpt;

type Result<T> = std::result::Result<T, failure::Error>;

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
    // "li.div.span > style: background-image: url('')"
    vlog::set_verbosity_level(conf.verbose as usize);

    let mut resp = reqwest::get(&conf.url)?;
    let mut body = String::new();
    resp.read_to_string(&mut body)?;

    v2!("Status: {}", resp.status());
    v2!("Headers: {}", resp.headers());
    v1!("{}", body);

    Ok(())
}

fn main() {
    let conf = Conf::from_args();

    match run(&conf) {
        Ok(_) => v1!("line-stickers-scraper COMPLETED!"),
        Err(e) => ve0!("{}", e),
    }
}
