use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "monitoring-cli", about = "Tool for gathering and sending system information via commandline.")]
pub enum Opt {
    Single {
        #[structopt(short, long)]
        api_key: String,
    },
    Service {
        #[structopt(short, long)]
        api_key: String,
        #[structopt(short, long)]
        sleep_seconds: u64,
    }
}