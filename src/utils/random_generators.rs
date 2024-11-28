use std::collections::HashSet;
use rand::prelude::{SliceRandom, StdRng};
use std::fmt::Write;

pub fn generate_unique_rng_strings(variables: usize, var_length: usize, rng: &mut StdRng, ordered_strings: &mut Vec<String>) {
    let alpha_num: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

    let mut unique_strings = HashSet::with_capacity(variables);

    let mut buffer = vec![0u8; var_length]; // Reusable buffer to store random characters

    while unique_strings.len() < variables {
        // Generate a random string by filling the buffer
        for byte in buffer.iter_mut() {
            *byte = *alpha_num.choose(rng).expect("alpha_num should not be empty");
        }

        // Convert buffer to String
        let random_string = unsafe { String::from_utf8_unchecked(buffer.clone()) };

        // Insert into the set to ensure uniqueness
        if unique_strings.insert(random_string.clone()) {
            ordered_strings.push(random_string);
        }
    }
}

pub fn generate_unique_string(variables: usize, ordered_strings: &mut Vec<String>) {
    for i in 0..variables {
        let mut result = String::with_capacity(1 + 20);

        result.push('v'); // Add the literal 'v'
        write!(&mut result, "{}", i).unwrap(); // Append the integer efficiently
        ordered_strings.push(result)
    }
}

pub fn generator(rng: &mut impl rand::Rng, size: usize) -> Vec<f64> {
    (0..size).map(|_| rng.gen_range(-20.0..20.0)).collect()
}