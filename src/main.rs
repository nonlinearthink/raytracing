mod example;

use clap::Parser;

#[derive(Parser, Debug)]
struct Opts {
    #[clap(short, long, num_args=1..)]
    program: Vec<String>,
}

fn main() {
    let opts = Opts::parse();
    for t in &opts.program {
        if t == "1_13" {
            example::example_1_13();
        } else if t == "1_14" {
            example::example_1_14();
        } else {
            println!("No example matched");
        }
    }
    if (&opts.program).len() == 0 {
        println!("Try:");
        println!("\tcargo run -- --program 1_13 1_14");
    }
}
