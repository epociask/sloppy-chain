mod block;

fn main() {
    let b = block::new_block(0);

    println!("{}", b.header.height);
}
