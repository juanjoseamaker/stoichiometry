use std::fmt;
use std::fs::File;
use std::io::BufReader;

#[derive(Debug, Clone)]
pub struct Element {
    symbol: String,
}

impl Element {
    pub fn new(symbol: String) -> Element {
        // TODO: check integrity
        Element { symbol }
    }

    pub fn atomic_mass(&self) -> Result<f64, Box<dyn std::error::Error>> {
        let file = File::open("PeriodicTable.csv")?;
        let buf_reader = BufReader::new(file);
        let mut rdr = csv::Reader::from_reader(buf_reader);

        for result in rdr.records() {
            let record = result?;
            
            if record[2].to_lowercase() == self.symbol.to_lowercase() {
                return Ok(record[3].parse()?);
            }
        }

        Err("Element symbol is invalid".into())
    }
}

// ----------------------------------------------------------------------

#[derive(Clone)]
pub struct Molecule {
    pub raw: String,
    pub elements: Vec<(Element, u32)>,
}

impl Molecule {
    pub fn new(raw: String) -> Result<Molecule, String> {
        let mut elements: Vec<(Element, u32)> = vec![];

        let mut element_index = 1;
        let mut symbol = "".to_string();

        let mut waiting_for_index = false;
        let mut waiting_for_digit = false;
        let mut number = 0;

        for c in raw.chars() {
            if c.is_digit(10) {
                waiting_for_digit = true;
                number = number * 10
                    + match c.to_digit(10) {
                        Some(d) => d,
                        None => {
                            panic!("Internal Error: failed to parse digit after checking integrity")
                        }
                    };
            } else {
                if waiting_for_digit {
                    waiting_for_digit = false;

                    if waiting_for_index {
                        element_index = number;
                    } else {
                        return Err("Unexpected digit".to_string());
                    }

                    number = 0;
                }

                if c == '_' {
                    waiting_for_index = true;
                } else if is_uppercase(c) {
                    // Add element
                    if symbol.len() != 0 {
                        elements.push((Element::new(symbol), element_index));
                    }

                    // Reset
                    symbol = "".to_string();
                    element_index = 1;
                    symbol.push(c)
                } else {
                    symbol.push(c)
                }
            }
        }

        if waiting_for_digit {
            if waiting_for_index {
                element_index = number;
            } else {
                return Err("Unexpected digit".to_string());
            }
        }

        if symbol.len() != 0 {
            elements.push((Element::new(symbol), element_index));
        }

        Ok(Molecule { raw, elements })
    }

    pub fn atomic_mass(&self) -> f64 {
        self.elements.iter().map(|e| e.1 as f64 * e.0.atomic_mass().unwrap()).sum()
    }
}

impl fmt::Debug for Molecule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Molecule")
            .field("raw", &self.raw)
            .field("elements", &self.elements)
            .finish()
    }
}

// ----------------------------------------------------------------------

#[derive(Debug)]
pub struct Equation {
    pub raw: String,
    pub reactants: Vec<(Molecule, u32)>,
    pub products: Vec<(Molecule, u32)>,
}

impl Equation {
    pub fn new(raw: String) -> Result<Equation, String> {
        // NOTE: If the coefficient is explicit zero, it will be changed to 1

        let mut reactants: Vec<(Molecule, u32)> = vec![];
        let mut products: Vec<(Molecule, u32)> = vec![];

        let mut waiting_for_digit = true;
        let mut waiting_for_molecule = false;
        let mut parsing_reactants = true;

        let mut molecule_raw = "".to_string();
        let mut coefficient = 0;

        for c in raw.chars() {
            if c.is_digit(10) && !waiting_for_molecule {
                waiting_for_digit = true;
                coefficient = coefficient * 10
                    + match c.to_digit(10) {
                        Some(d) => d,
                        None => {
                            panic!("Internal Error: failed to parse digit after checking integrity")
                        }
                    };
            } else {
                if waiting_for_digit {
                    waiting_for_digit = false;
                    waiting_for_molecule = true;
                }

                if c == '+' {
                    if parsing_reactants {
                        reactants.push((
                            match Molecule::new(molecule_raw) {
                                Ok(m) => m,
                                Err(msg) => return Err(msg),
                            },
                            match coefficient {
                                0 => 1,
                                _ => coefficient,
                            },
                        ));
                        molecule_raw = "".to_string();
                        coefficient = 0;
                        waiting_for_digit = true;
                        waiting_for_molecule = false;
                    } else {
                        products.push((
                            match Molecule::new(molecule_raw) {
                                Ok(m) => m,
                                Err(msg) => return Err(msg),
                            },
                            match coefficient {
                                0 => 1,
                                _ => coefficient,
                            },
                        ));
                        molecule_raw = "".to_string();
                        coefficient = 0;
                        waiting_for_digit = true;
                        waiting_for_molecule = false;
                    }
                } else if c == '=' {
                    if parsing_reactants {
                        reactants.push((
                            match Molecule::new(molecule_raw) {
                                Ok(m) => m,
                                Err(msg) => return Err(msg),
                            },
                            match coefficient {
                                0 => 1,
                                _ => coefficient,
                            },
                        ));
                        molecule_raw = "".to_string();
                        coefficient = 0;
                        waiting_for_digit = true;
                        waiting_for_molecule = false;
                    } else {
                        return Err("Unexpected '='".to_string());
                    }

                    parsing_reactants = false;
                } else if waiting_for_molecule {
                    molecule_raw.push(c);
                } else {
                    return Err("Failed to parse".to_string());
                }
            }
        }

        if parsing_reactants == false {
            products.push((
                match Molecule::new(molecule_raw) {
                    Ok(m) => m,
                    Err(msg) => return Err(msg),
                },
                match coefficient {
                    0 => 1,
                    _ => coefficient,
                },
            ));
        } else {
            return Err("Expected '='".to_string());
        }

        Ok(Equation {
            raw,
            reactants,
            products,
        })
    }
}

// ----------------------------------------------------------------------

fn is_uppercase(c: char) -> bool {
    let uppercase = c.to_uppercase().to_string();
    c.to_string() == uppercase
}
