use crate::service::repl::run_loop;

mod service;

fn main() {
    println!("Use the `> help` command to view available commands.");
    run_loop()
}
