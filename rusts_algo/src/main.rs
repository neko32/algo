use clap::Parser;
use rusts_algo::{runner, Algo, Args};

fn main() {
    let args: Args = Args::parse();
    let algo: Algo = Algo::from_str(args.algo.as_str());
    runner::exec(algo);
}
