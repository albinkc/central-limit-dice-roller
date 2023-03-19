use std::env;

pub fn parse_args() -> (usize, usize, usize) {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 || args.len() > 4 {
        eprintln!("Error: Incorrect number of parameters provided.");
        eprintln!("Usage: {} <rolls> <number_of_dice> [die_faces]", args[0]);
        eprintln!("  <rolls>          : The number of rolls (required)");
        eprintln!("  <number_of_dice> : The number of dice to roll (required)");
        eprintln!("  [die_faces]      : The number of faces on each die (optional, default: 6)");
        std::process::exit(1);
    }

    let rolls: usize = args[1].parse().expect("Invalid number of rolls");
    let number_of_dice: usize = args[2].parse().expect("Invalid number of dice");
    let die_faces: usize = if args.len() == 4 {
        args[3].parse().expect("Invalid number of die faces")
    } else {
        6
    };

    (rolls, number_of_dice, die_faces)
}
