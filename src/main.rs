use std::{env, str::FromStr};

use rayon::{
    iter::{once, ParallelIterator},
    prelude::*,
    slice::ParallelSlice,
};
use rug::Integer;

fn main() {
    // Read the argument, panics if argument is missing
    let args: Vec<String> = env::args().collect();
    let end = usize::from_str(&args[1]).unwrap() + 1;

    // Max number of binomial coefficients to "cache"
    let max_comb = end / 2 + 2;
    // The binom coeff. cache
    let mut comb = Vec::with_capacity(max_comb);
    // n over 0 is always 1
    comb.push(Integer::from(1));
    // Fubini numbers result list
    let mut fub = Vec::with_capacity(end);
    fub.push(Integer::from(1));
    // Intermediate fubini number result
    let mut result: Integer;
    // Midpoint of the current pascal's triangle row (the row is symmetrical after mid)
    let mut mid: usize;
    // Temp variable to circumvent borrow checker
    let mut is_n_even = false;
    for n in 1..end {
        mid = (n + 1) / 2;
        // Update the current row of the pascal's triangle
        comb = once(Integer::from(1))
            .chain(comb.par_windows(2).map(|w| Integer::from(&w[0] + &w[1])))
            .collect();

        // Elongate the pascal's triangles row
        if !is_n_even {
            comb.push(comb.last().cloned().unwrap());
        }

        result = (1..mid)
            .into_par_iter()
            .map(|k| &comb[k] * Integer::from(&fub[n - k] + &fub[k]))
            .reduce(|| Integer::new(), |a, b| a + b);
        // Value for k = n of the sum formula implemented above
        result += Integer::from(1);
        // Add the middle element of the sum
        if is_n_even {
            result += &fub[mid] * &comb[mid]
        }
        fub.push(result);
        is_n_even ^= true;
    }
    println!("{:?}", fub);
}
