use structopt::StructOpt;

#[derive(StructOpt)]
pub struct Args {
    #[structopt(short, long, default_value="info", global = true)]
    pub log_level: log::LevelFilter,
    pub url: String
}
