use clap::Parser;

#[derive(Parser, Clone, Debug)]
pub struct Config {
    #[arg(long, default_value = "https://localhost:4443")]
    pub server: String,
}
