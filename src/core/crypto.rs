use rand::Rng;
// extern crate hasher; // git repository

pub fn generate_random_number() -> u64 {
    let mut rng = rand::thread_rng();
    let rndnum = rng.gen();

    rndnum
}