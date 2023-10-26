use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// The path to the configuration file
    ///        
    /// Running as a client or a server is automatically determined according to the
    /// configuration file.
    #[clap(name = "config")]
    pub config_path: Option<String>,

    /// run as server mode
    #[clap(long, short)]
    pub server: bool,

    /// run as client mode
    #[clap(long, short)]
    pub client: bool,
}

pub fn get_args() -> Cli {
    Cli::parse()
}

fn main() {
    let _args = get_args();
    println!("{:?}", _args);
}
