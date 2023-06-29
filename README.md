
# Central Limit Dice Roller
A command-line dice roller written in Rust that demonstrates the Central Limit Theorem by rolling multiple dice multiple times and printing the distribution of the sum of the dice.

## Features

    Roll multiple dice with a given number of faces
    Generate the distribution of the sum of the dice
    Visualize the distribution using a histogram

## Installation

### Prerequisites
    Rust 2021 or later

### Clone the Repository
```sh
git clone https://github.com/albinkc/central-limit-dice-roller.git
cd central_limit
```

### Build and Install
```sh
cargo build --release
cargo install --path .
```
This will install the central_limit binary in your ~/.cargo/bin folder. Make sure this folder is in your PATH.

## Usage
	central_limit <rolls> <number_of_dice> [die_faces]
		<rolls>: The number of rolls (required)
		<number_of_dice>: The number of dice to roll (required)
		[die_faces]: The number of faces on each die (optional, default: 6)

### Example:
```sh
central_limit 700 3 6
```
This will roll three 6-sided dice 700 times and print the distribution of the sum of the dice.

```
 3: *
 4: **************
 5: *******************
 6: *******************************
 7: ******************************************************
 8: ******************************************************************
 9: *************************************************************************************
10: ***********************************************************************************************
11: ***********************************************************************************
12: **********************************************************************************
13: ****************************************************************
14: ***************************************
15: ********************************
16: *************************
17: *******
18: ***
```

## Contributing

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

## License
MIT


