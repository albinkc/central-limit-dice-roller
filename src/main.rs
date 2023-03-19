mod args_parser;
mod dice;

fn main() {
    let (rolls, number_of_dice, die_faces) = args_parser::parse_args();
    let outcome_frequencies = dice::generate_outcome_frequencies(rolls, number_of_dice, die_faces);
    dice::print_outcomes(outcome_frequencies, number_of_dice, die_faces);
}
