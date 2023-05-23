use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "monitoring-cli", about = "Tool for gathering and sending system information via commandline.")]
pub enum Opt {
    Single {
        #[structopt(short, long)]
        api_key: String,
        #[structopt(short, long)]
        profile_id: u32,
    },
    Service {
        #[structopt(short, long)]
        api_key: String,
        #[structopt(short, long)]
        sleep_seconds: u64,
        #[structopt(short, long)]
        profile_id: u32,
    }
}