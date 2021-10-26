use super::equation::{Equation, Molecule};

#[derive(Clone)]
pub enum CalculationData {
    Mass(f64, Molecule),
    Mole(f64, Molecule),
}

pub struct Calculation {
    pub equation: Equation,
    pub input: Vec<CalculationData>,
}

impl Calculation {
    pub fn new(equation: Equation, input: Vec<CalculationData>) -> Calculation {
        Calculation { equation, input }
    }

    pub fn calculate(&self, output: Molecule) -> Result<(f64, f64), String> {
        // Return data in mass and mole
        if self.input.len() != 1 {
            unimplemented!();
        }

        let process_data = self.input[0].clone();
        let mut result = match process_data {
            CalculationData::Mass(g, _) => g,
            CalculationData::Mole(mol, _) => mol,
        };

        // If data is in mass, pass it to mole
        if let CalculationData::Mass(_, ref process_molecule) = process_data {
            println!("{} {} {:?}", process_molecule.raw, process_molecule.atomic_mass(), process_molecule.elements);
            result /= process_molecule.atomic_mass();
        }

        result *= self.molar_factor(
            match process_data {
                CalculationData::Mass(_, ref molecule) => molecule,
                CalculationData::Mole(_, ref molecule) => molecule,
            },
            &output,
        )?;

        Ok((result, result * output.atomic_mass()))
    }

    fn molar_factor(
        &self,
        molecule_from: &Molecule,
        molecule_to: &Molecule,
    ) -> Result<f64, String> {
        let mut mol_from: u32 = 0;
        let mut mol_to: u32 = 0;
        for molecule in self
            .equation
            .reactants
            .iter()
            .chain(self.equation.products.iter())
        {
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
}
