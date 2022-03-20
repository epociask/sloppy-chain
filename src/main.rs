mod core;


fn main() {
    let b = core::new_block(0);

    println!("{}", b.header.height);
}
