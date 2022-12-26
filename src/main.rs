use primitives::block::Block;

fn main() {
    let b = Block::new(0);

    println!("{}", b.header.height);
}
