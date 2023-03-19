use rand::Rng;

pub fn roll_dice(die_faces: usize) -> usize {
    rand::thread_rng().gen_range(1..=die_faces)
}

pub fn generate_outcome_frequencies(rolls: usize, number_of_dice: usize, die_faces: usize) -> Vec<u8> {
    let mut outcome_frequencies: Vec<u8> = vec![0; die_faces * number_of_dice];

    for _ in 1..=rolls {
        let mut sum: usize = 0;
        for _ in 0..number_of_dice {
            sum += roll_dice(die_faces);
        }
        outcome_frequencies[sum - 1] += 1;
    }

    outcome_frequencies
}

pub fn print_outcomes(outcome_frequencies: Vec<u8>, number_of_dice: usize, die_faces: usize) {
    let mut i = number_of_dice;
    let max_sum: usize = die_faces * number_of_dice;
    let max_digits: usize = (max_sum as f64).log10() as usize + 1;
    for freq in outcome_frequencies.iter().skip(number_of_dice - 1) {
        print!("{:max_digits$}: ", i);
        i += 1;
        for _ in 0..*freq {
            print!("*");
        }
        println!();
    }
}
