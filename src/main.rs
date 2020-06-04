use std::env;

fn main()
{
    let args: Vec<String> = env::args().collect();

    for arg in args.iter() {
        println!("{}", arg);
    }

    // cargo run src/main.rs arg1 arg2
}
