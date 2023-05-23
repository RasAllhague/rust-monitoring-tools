use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "monitoring-cli",
    about = "Tool for gathering and sending system information via commandline."
)]
pub enum Opt {
    Error {
        #[structopt(short, long)]
        message: String,
        #[structopt(short, long)]
        api_key: String,
        #[structopt(short, long)]
        id: u32,
        #[structopt(short, long)]
        profile_key: String,
    },
    Single {
        #[structopt(short, long)]
        api_key: String,
        #[structopt(short, long)]
        id: u32,
        #[structopt(short, long)]
        profile_key: String,
    },
    Service {
        #[structopt(short, long)]
        api_key: String,
        #[structopt(short, long)]
        sleep_seconds: u64,
        #[structopt(short, long)]
        id: u32,
        #[structopt(short, long)]
        profile_key: String,
    },
}
