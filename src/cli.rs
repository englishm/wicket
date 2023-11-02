use clap::Parser;
use url::Url;

#[derive(Parser, Clone, Debug)]
pub struct Config {
    #[arg(long, default_value = "https://localhost:4443")]
    pub server: Url,
}
