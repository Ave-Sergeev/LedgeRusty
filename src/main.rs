use crate::service::blockchain::Blockchain;

mod service;

fn main() {
    let difficulty: usize = 1;
    let mut blockchain: Blockchain = Blockchain::new(difficulty);

    Blockchain::add_block(&mut blockchain);
    Blockchain::get_all_blocks(&mut blockchain)
}
