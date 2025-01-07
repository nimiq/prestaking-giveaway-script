use std::vec;

use rand::seq::SliceRandom;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

use nimiq_vrf::{DiscreteDistribution, Rng, VrfEntropy, VrfUseCase};

// Replace these with the values from the selected on-chain block
const BLOCK_NUMBER: u64 = 7557700;
const BLOCK_HASH: &str = "6bede69541d2cf748f023866d533deaf3819212e40e2af51012c6e1e825f6d94";

fn main() {
    // Initialize prize list
    let mut prize_list = Vec::with_capacity(100);

    // Add 10 x 3M NIM prizes
    prize_list.resize(10, "3M NIM");

    // Add 25 x 1.5M NIM prizes
    prize_list.resize(10 + 25, "1.5M NIM");

    // Add 65 x 500k NIM prizes
    prize_list.resize(10 + 25 + 65, "500k NIM");

    // Initialize a random number generator with the block number as the seed
    let mut prize_list_rng = ChaCha8Rng::seed_from_u64(BLOCK_NUMBER);
    // Shuffle the prize list with the seeded random number generator
    prize_list.shuffle(&mut prize_list_rng);

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

    // Initialize a random number generator with the block hash as the seed/entropy
    let hash = hex::decode(BLOCK_HASH).unwrap();
    let mut entropy = [0u8; 32];
    assert_eq!(hash.len(), entropy.len());
    entropy.copy_from_slice(&hash);
    let lottery_seed = VrfEntropy(entropy);
    let mut lottery_rng = lottery_seed.rng(VrfUseCase::RewardDistribution);

    // Loop through the prize list and pick a winner for each prize
    for prize in prize_list {
        let winner = pick_winner(&prestakers, &mut lottery_rng);
        println!("Winner: {} - {}", winner, prize);

        // Remove the winner from the prestakers list to ensure one address can only win one prize
        prestakers.retain(|x| x.0 != winner);
    }
}

// Pick a winner from a list of eligible prestakers and a stateful random number generator
fn pick_winner<'a, R>(data: &Vec<(&'a str, u64)>, rng: &mut R) -> &'a str
where
    R: Rng,
{
    // Initialize two vectors to store the prestaker addresses and their respective points
    let size = data.len();
    let mut prestaker_addresses = Vec::with_capacity(size);
    let mut prestaker_points = Vec::with_capacity(size);

    for (address, coin) in data {
        prestaker_addresses.push(address);
        prestaker_points.push(*coin);
    }

    // Create a discrete distribution from the prestaker points
    let lookup = DiscreteDistribution::new(&prestaker_points);

    // Pick the winner by sampling from the discrete distribution with the given random number generator
    let index = lookup.sample(rng);

    // Return the address of the winner
    prestaker_addresses[index]
}
