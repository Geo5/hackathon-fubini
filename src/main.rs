use std::ops::AddAssign;
use std::{env, str::FromStr};

use ramp::Int;

fn main() {
    // Read the argument, panics if argument is missing
    let args: Vec<String> = env::args().collect();
    let end = u64::from_str(&args[1]).unwrap() + 1;

    // Max number of binomial coefficients to "cache"
    let max_comb = end / 2 + 2;
    // The binom coeff. cache
    let mut comb = Vec::with_capacity(max_comb as usize);
    // n over 0 is always 1
    comb.push(Int::one());

    // Fubini numbers result list
    let mut fub = Vec::with_capacity(end as usize);
    fub.push(Int::one());
    // Intermediate fubini number result
    let mut result: Int;
    // Midpoint of the current pascal's triangle row (the row is symmetrical after mid)
    let mut mid: u64;
    // Temp variable to circumvent borrow checker
    let mut comb_cur: &mut Int;
    for n in 1..end {
        result = Int::one();
        mid = n / 2;
        // We need to gradually expand the comb vector, until it is full
        // It could have been done on creation, but benchmarking revealed that to be slower
        if n < max_comb {
            comb.push(Int::one());
        }
        // Update the cached first half of the pascal's triangle's row in dynamic programming fashion
        for i in (1..=mid).rev() {
            // Unsafe to make the borrow checker happy. We are sure, wo did not introduce any undefined behavior here.
            unsafe {
                comb_cur = &mut *(comb.get_unchecked_mut(i as usize) as *mut Int);
            }
            // Update of the binom coeff. value
            comb_cur.add_assign(&comb[(i - 1) as usize]);
            // Summation of the current fubini number
            result += &comb[i as usize] * &fub[(n - i) as usize];
        }
        // We add a dummy value to the binom coeff. cache here to make sure, that the recurrence for the binomial coefficient always works.
        // The recurrence on even n needs one element past the midpoint of the binom coeff. cache, which is always equal to the element before it.
        comb[(mid + 1) as usize] = comb[mid as usize].clone();
        // Now we just need to sum up the rest of the recurrence, without updating the binom coeffs, because they are symmetrical.
        for i in (mid + 1)..n {
            result += &comb[(n - i) as usize] * &fub[(n - i) as usize];
        }
        fub.push(result);
    }
    println!("{:?}", fub);
}
