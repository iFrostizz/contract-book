use clap::Parser;

#[derive(Parser, Debug)]
pub struct Utils {
    #[clap(short, long, help = "reset db")]
    pub reset: bool,
}
