use std::collections::HashMap;

use rand::prelude::{SliceRandom, StdRng};
use rand::Rng;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use crate::utils::random_generators::generator;

#[derive(Debug, PartialEq, Serialize)]
pub enum VarType {
    SPIN,
    BINARY,
    INTEGER,
    REAL,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde_as]
pub struct QmSchema {
    #[serde_as(as = "Vec<(_, _)>")]
    pub quadratic: HashMap<(String, String), f64>,
    pub linear: HashMap<String, f64>,

    pub offset: Option<f64>,
}

impl QmSchema {
    fn generate_vtype_map(
        variables: &[String],
        num_variables: usize,
    ) -> HashMap<&String, VarType> {
        // Create a mapping from references to variables to a fixed VarType::BINARY
        let vtype_map: HashMap<&String, VarType> = variables.iter()
            .take(num_variables)  // Limit iteration to the number of variables
            .map(|var| (var, VarType::BINARY))  // Directly create the tuple
            .collect();

        vtype_map
    }

    fn create_quadratic(
        coo_x: &[String],
        coo_y: &[String],
        coo_v: &[f64],
        vtype_map: &HashMap<&String, VarType>,
    ) -> HashMap<(String, String), f64> {
        let mut quadratic: HashMap<(String, String), f64> = HashMap::new();

        // Iterate over zipped iterators of coo_x, coo_y, and coo_v
        for ((v1, v2), &x) in coo_x.iter().zip(coo_y.iter()).zip(coo_v.iter()) {
            // Check if the variables should be included in the quadratic map

            if vtype_map.get(v1).unwrap_or(&VarType::BINARY) != &VarType::BINARY || v1 != v2 {
                // Insert into the hashmap if the condition is met
                quadratic.insert((v1.clone(), v2.clone()), x);
            }
        }

        quadratic
    }

    fn random(
        variable_names: &mut Vec<String>,
        interactions: usize,
        quadratic_terms: bool,
        rng: &mut StdRng,
    ) -> QmSchema {
        let quadratic: HashMap<(String, String), f64>;

        if quadratic_terms {
            let coo_x: Vec<String> = variable_names
                .choose_multiple(rng, interactions)
                .cloned()
                .collect();
            let coo_y: Vec<String> = variable_names
                .choose_multiple(rng, interactions)
                .cloned()
                .collect();
            let coo_v: Vec<f64> = (0..interactions).map(|_| rng.gen_range(-20.0..20.0)).collect();
            let var_map = QmSchema::generate_vtype_map(&variable_names, interactions);

            quadratic = QmSchema::create_quadratic(&coo_x, &coo_y, &coo_v, &var_map);
        } else {
            quadratic = HashMap::new();
        }

        let linear_values = generator(rng, variable_names.len());

        let linear: HashMap<String, f64> = variable_names.clone().into_iter().zip(linear_values).collect();


        let offset: Option<f64> = Some(2.0);

        return Self {
            quadratic,
            linear,
            offset,
        };
    }
}