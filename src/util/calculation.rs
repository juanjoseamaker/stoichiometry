use super::equation::{Equation, Molecule};

#[derive(Clone)]
pub enum CalculationData {
    Gram(f64, Molecule),
    Mole(f64, Molecule),
}

pub struct Calculation {
    pub equation: Equation,
    pub input: Vec<CalculationData>,
    pub required_output: Molecule,
    pub output_gram: f64,
    pub output_mole: f64,
    pub steps: Vec<String>,
}

impl Calculation {
    pub fn new(
        equation: Equation,
        input: Vec<CalculationData>,
        required_output: Molecule,
    ) -> Result<Calculation, String> {
        // Return data in mass and mole
        if input.len() != 1 {
            unimplemented!();
        }

        let process_data = input[0].clone();
        let mut result = match process_data {
            CalculationData::Gram(g, _) => g,
            CalculationData::Mole(mol, _) => mol,
        };

        // If data is in mass, pass it to mole
        if let CalculationData::Gram(_, ref process_molecule) = process_data {
            result /= process_molecule.atomic_mass();
        }

        result *= molar_factor(
            &equation,
            match process_data {
                CalculationData::Gram(_, ref molecule) => molecule,
                CalculationData::Mole(_, ref molecule) => molecule,
            },
            &required_output,
        )?;

        Ok(Calculation {
            equation,
            input,
            output_gram: result * required_output.atomic_mass(),
            output_mole: result,
            required_output,
            steps: vec![],
        })
    }
}

fn molar_factor(
    equation: &Equation,
    molecule_from: &Molecule,
    molecule_to: &Molecule,
) -> Result<f64, String> {
    let mut mol_from: u32 = 0;
    let mut mol_to: u32 = 0;
    for molecule in equation.reactants.iter().chain(equation.products.iter()) {
        if molecule.0.raw == molecule_from.raw {
            mol_from = molecule.1;
        } else if molecule.0.raw == molecule_to.raw {
            mol_to = molecule.1;
        }
    }

    if mol_from == 0 || mol_to == 0 {
        return Err("Failed to get molar factor from equation".to_string());
    }

    Ok(mol_to as f64 / mol_from as f64)
}
