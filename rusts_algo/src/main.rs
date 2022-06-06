use rusts_algo::{Algo, Args, runner};
use clap::Parser;


fn main() {
    let args:Args = Args::parse();
    let algo:Algo = Algo::from_str(args.algo.as_str());
    runner::exec(algo);
}
