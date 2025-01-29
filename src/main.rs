use std::{error::Error, fs::File, process};

use rand::{seq::SliceRandom, SeedableRng};
use rand_chacha::ChaCha8Rng;

use nimiq_vrf::{DiscreteDistribution, Rng, VrfEntropy, VrfUseCase};

fn main() {
    // Read arguments from the command line
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 4 {
        println!("Usage: {} <round> <block_number> <block_hash>", args[0]);
        process::exit(1);
    }
    let round = args[1].parse().unwrap();
    let block_number = args[2].parse().unwrap();
    let block_hash = &args[3];

    println!("Round: {}", round);
    println!("Block number: {}", block_number);
    println!("Block hash: {}\n", block_hash);

    if let Err(err) = run(round, block_number, block_hash) {
        println!("{}", err);
        process::exit(1);
    }
}

fn run(round: u8, block_number: u32, block_hash: &str) -> Result<(), Box<dyn Error>> {
    // Initialize prize list
    let mut prize_list = Vec::new();

    match round {
        1 => {
            prize_list.resize(3, "3M NIM");
            prize_list.resize(3 + 8, "1.5M NIM");
            prize_list.resize(3 + 8 + 21, "500k NIM");
        }
        2 => {
            prize_list.resize(3, "3M NIM");
            prize_list.resize(3 + 8, "1.5M NIM");
            prize_list.resize(3 + 8 + 22, "500k NIM");
        }
        3 => {
            prize_list.resize(4, "3M NIM");
            prize_list.resize(4 + 9, "1.5M NIM");
            prize_list.resize(4 + 9 + 22, "500k NIM");
        }
        _ => {
            println!("Invalid round number");
            process::exit(1);
        }
    }

    // Initialize a random number generator with the block number as the seed
    let mut prize_list_rng = ChaCha8Rng::seed_from_u64(block_number as u64);
    // Shuffle the prize list with the seeded random number generator
    prize_list.shuffle(&mut prize_list_rng);

    let mut prestakers: Vec<(String, f32)> = Vec::new();

    // Populate the eligible prestakers list from a CSV file
    let file = File::open(format!("./prestakers-round{}.csv", round)).map_err(|e| {
        format!(
            "Cannot open file {}: {}",
            format!("./prestakers-round{}.csv", round),
            e,
        )
    })?;
    let mut reader = csv::Reader::from_reader(file);
    for result in reader.records() {
        let record = result?;
        prestakers.push((
            record.get(0).unwrap().to_string(), // First field is the address
            record.get(2).unwrap().parse().unwrap(), // Third field are the points
        ));
    }

    println!("Eligible prestakers: {}", prestakers.len());

    // Initialize a random number generator with the block hash as the seed/entropy
    let hash = hex::decode(block_hash).unwrap();
    let mut entropy = [0u8; 32];
    assert_eq!(hash.len(), entropy.len());
    entropy.copy_from_slice(&hash);
    let lottery_seed = VrfEntropy(entropy);
    let mut lottery_rng = lottery_seed.rng(VrfUseCase::RewardDistribution);

    println!("\nAddress, Prize");

    // Loop through the prize list and pick a winner for each prize
    for prize in prize_list {
        let winner = pick_winner(&prestakers, &mut lottery_rng);
        println!("{}, {}", winner, prize);

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
