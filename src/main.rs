use std::{error::Error, fs::File, process};

use rand::{seq::SliceRandom, SeedableRng};
use rand_chacha::ChaCha8Rng;

use nimiq_vrf::{DiscreteDistribution, Rng, VrfEntropy, VrfUseCase};

// Replace these with the values from the selected on-chain block
const BLOCK_NUMBER: u64 = 7557700;
const BLOCK_HASH: &str = "6bede69541d2cf748f023866d533deaf3819212e40e2af51012c6e1e825f6d94";

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
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

    let mut prestakers: Vec<(String, f32)> = Vec::new();

    // Populate the eligible prestakers list from a CSV file
    let file = File::open("./prestakers.csv")?;
    let mut reader = csv::Reader::from_reader(file);
    for result in reader.records() {
        let record = result?;
        prestakers.push((
            record.get(0).unwrap().to_string(), // First field is the address
            record.get(2).unwrap().parse().unwrap(), // Third field are the points
        ));
    }

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

    Ok(())
}

// Pick a winner from a list of eligible prestakers and a stateful random number generator
fn pick_winner<R>(data: &Vec<(String, f32)>, rng: &mut R) -> String
where
    R: Rng,
{
    // Initialize two vectors to store the prestaker addresses and their respective points
    let size = data.len();
    let mut prestaker_addresses = Vec::with_capacity(size);
    let mut prestaker_points = Vec::with_capacity(size);

    for (address, points) in data {
        prestaker_addresses.push(address);

        // Some points are .5 which are rounded up
        prestaker_points.push((*points).round() as u64);
    }

    // Create a discrete distribution from the prestaker points
    let lookup = DiscreteDistribution::new(&prestaker_points);

    // Pick the winner by sampling from the discrete distribution with the given random number generator
    let index = lookup.sample(rng);

    // Return the address of the winner
    prestaker_addresses[index].clone()
}
