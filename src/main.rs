use std::{env, str::FromStr};

use rayon::{
    iter::{once, IntoParallelRefIterator, ParallelIterator},
    prelude::*,
    slice::ParallelSlice,
};
use rug::Integer;

fn main() {
    // Read the argument, panics if argument is missing
    let args: Vec<String> = env::args().collect();
    let end = usize::from_str(&args[1]).unwrap() + 1;
    if end == 1 {
        println!("[1]");
        return;
    }
    let mut fub = Vec::with_capacity(end);
    // Fubini numbers result list
    let mut results: Vec<Integer> = Vec::with_capacity(end);
    results.push(Integer::from(1));
    results.push(Integer::from(1));
    let mut fac = Integer::from(1);
    for n in 2..end {
        fub = once(Integer::from(1))
            .chain(
                fub.par_windows(2)
                    .enumerate()
                    .map(|(k, w)| (k + 2) as u64 * Integer::from(&w[0] + &w[1])),
            )
            .collect();
        fac *= n as u64;
        fub.push(fac.clone());
        results.push(fub.par_iter().sum());
    }
    println!("{:?}", results);
}
