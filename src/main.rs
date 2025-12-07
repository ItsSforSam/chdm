use clap::Parser;

mod cli;

#[tokio::main]
async fn main() {
    ctrlc::set_handler(|| log::debug!("SIGINT caught and ignored")).unwrap();
    let p = cli::CLI::parse();
    println!("Value of CLI: {:#?}",p);
}
