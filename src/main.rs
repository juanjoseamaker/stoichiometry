mod util;
use util::calculation::{Calculation, CalculationData};
use util::equation::{Equation, Molecule};

fn main() {
    let calculation = Calculation::new(
        Equation::new("4Fe+3O_2=2Fe_2O_3".to_string()).unwrap(),
        vec![CalculationData::Mass(
            240000.0,
            Molecule::new("Fe_2O_3".to_string()).unwrap(),
        )],
    );

    println!("{:?}{:?}", calculation.equation.products, calculation.equation.reactants);
    println!("Result Calulation = {:?}", calculation.calculate(Molecule::new("O_2".to_string()).unwrap()).unwrap());
}
