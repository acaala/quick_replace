
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(index = 1)]
    from: String,

    #[arg(index = 2)]
    to: String,

    #[arg(index = 3)]
    path: String,
}
fn main() {
    let args = Args::parse();

    qrep::run(args.from, args.to, args.path);
}