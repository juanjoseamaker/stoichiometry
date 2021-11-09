use std::io;

mod util;
use util::calculation::{Calculation, CalculationData};
use util::equation::{Equation, Molecule};

fn main() {
    println!("Enter equation:");
    let equation = Equation::new(read_line()).unwrap();

    let mut data: Vec<CalculationData> = vec![];

    println!("Enter Input:");
    loop {
        println!("Enter Molecule Data Type:");
        let molecule = Molecule::new(read_line()).unwrap();

        println!("Mole or Gram? [mole/gram/m/g]:");
        match read_line().to_lowercase().as_ref() {
            "mole" | "m" => {
                println!("Enter Data (number):");
                data.push(CalculationData::Mole(
                    read_line().trim().parse().expect("Please type a number!"),
                    molecule,
                ));
            }
            "gram" | "g" => {
                println!("Enter Data:");
                data.push(CalculationData::Gram(
                    read_line().trim().parse().expect("Please type a number!"),
                    molecule,
                ));
            }
            _ => {
                eprintln!("Expected mole/gram/m/g");
                continue;
            }
        }

        println!("Continue Adding More Data? [yes/no/y/n]");
        match read_line().to_lowercase().as_ref() {
            "yes" | "y" => continue,
            "no" | "n" | _ => break,
        }
    }

    println!("Enter Molecule To Output Using Input And Equation:");
    let output = Molecule::new(read_line()).unwrap();

    let calculation = Calculation::new(equation, data, output).unwrap();

    println!(
        "Result In Mole={} Gram={}",
        calculation.output_mole, calculation.output_gram
    );
}

fn read_line() -> String {
    let mut string = String::new();

    io::stdin()
        .read_line(&mut string)
        .expect("Failed to read line");

    string[..string.len() - 1].to_string()
}
