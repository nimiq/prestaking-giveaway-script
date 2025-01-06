use std::vec;

use rand::SeedableRng;
use rand::seq::SliceRandom;
use rand_chacha::ChaCha8Rng;

use nimiq_vrf::{DiscreteDistribution, VrfSeed, VrfUseCase};

const BLOCK_NUMBER: u64 = 7557700;
const BLOCK_HASH: &str = "6bede69541d2cf748f023866d533deaf3819212e40e2af51012c6e1e825f6d94";

fn main() {
    println!("Hello, world!");

    let mut prizes = Vec::with_capacity(100);
    for _ in 0..10 {
        prizes.push("3M NIM");
    }
    for _ in 0..25 {
        prizes.push("1.5M NIM");
    }
    for _ in 0..65 {
        prizes.push("500k NIM");
    }

    let mut rng = ChaCha8Rng::seed_from_u64(BLOCK_NUMBER as u64);
    prizes.shuffle(&mut rng);
    println!("{:?}", prizes);

    // TODO: Read from csv file
    let mut prestakers = vec![
        ("a", 1u64), ("b", 2), ("c", 3), ("d", 4), ("e", 5),
        ("f", 6), ("g", 7), ("h", 8), ("i", 9), ("j", 10),
        ("k", 11), ("l", 12), ("m", 13), ("n", 14), ("o", 15),
        ("p", 16), ("q", 17), ("r", 18), ("s", 19), ("t", 20),
        ("u", 21), ("v", 22), ("w", 23), ("x", 24), ("y", 25),
        ("z", 26), ("aa", 27), ("ab", 28), ("ac", 29), ("ad", 30),
        ("ae", 31), ("af", 32), ("ag", 33), ("ah", 34), ("ai", 35),
        ("aj", 36), ("ak", 37), ("al", 38), ("am", 39), ("an", 40),
        ("ao", 41), ("ap", 42), ("aq", 43), ("ar", 44), ("as", 45),
        ("at", 46), ("au", 47), ("av", 48), ("aw", 49), ("ax", 50),
        ("ay", 51), ("az", 52), ("ba", 53), ("bb", 54), ("bc", 55),
        ("bd", 56), ("be", 57), ("bf", 58), ("bg", 59), ("bh", 60),
        ("bi", 61), ("bj", 62), ("bk", 63), ("bl", 64), ("bm", 65),
        ("bn", 66), ("bo", 67), ("bp", 68), ("bq", 69), ("br", 70),
        ("bs", 71), ("bt", 72), ("bu", 73), ("bv", 74), ("bw", 75),
        ("bx", 76), ("by", 77), ("bz", 78), ("ca", 79), ("cb", 80),
        ("cc", 81), ("cd", 82), ("ce", 83), ("cf", 84), ("cg", 85),
        ("ch", 86), ("ci", 87), ("cj", 88), ("ck", 89), ("cl", 90),
        ("cm", 91), ("cn", 92), ("co", 93), ("cp", 94), ("cq", 95),
        ("cr", 96), ("cs", 97), ("ct", 98), ("cu", 99), ("cv", 100),
    ];

    for i in 0..prizes.len() {
        let prestakers_ref = &prestakers;
        let winner = pick_winner(prestakers_ref.to_vec());
        println!("Winner: {} - {}, {} left", winner, prizes[i], prestakers_ref.len() - 1);

        prestakers.retain(|x| x.0 != winner);
    }
}

fn pick_winner(data: Vec<(&str, u64)>) -> &str {
    let size = data.len();
    let mut prestaker_addresses = Vec::with_capacity(size);
    let mut prestaker_points = Vec::with_capacity(size);

    for (address, coin) in data {
        prestaker_addresses.push(address);
        prestaker_points.push(coin);
    }

    let seed = VrfSeed::default();
    let mut rng = seed.rng(VrfUseCase::ValidatorSlotSelection);

    let lookup = DiscreteDistribution::new(&prestaker_points);

    let index = lookup.sample(&mut rng);
    prestaker_addresses[index]
}
