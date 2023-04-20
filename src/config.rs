use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct Config {
    #[structopt(short, long)]
    pub base_url: String,

    #[structopt(short, long, default_value = "/")]
    pub start_path: String,

    #[structopt(short, long, default_value = "output.json")]
    pub output: String,
}

impl Config {
    pub fn from_args() -> Self {
        Config::from_args()
    }
}