use std::ops::AddAssign;
use std::{env, str::FromStr};

use ramp::Int;

fn main() {
    let args: Vec<String> = env::args().collect();
    let end = u64::from_str(&args[1]).unwrap() + 1;

    let max_comb = end / 2 + 2;
    let mut comb = Vec::with_capacity(max_comb as usize);
    comb.push(Int::one());
    let mut fub = Vec::with_capacity(end as usize);
    fub.push(Int::one());
    // The intermediate fubini numbers
    let mut result: Int;
    // Midpoint of the current pascal's triangle row (the row is symmetrical after mid)
    let mut mid: u64;
    // temp variable to circumvent burrow checker
    let mut comb_cur: &mut Int;
    for n in 1..end {
        result = Int::one();
        mid = n / 2;
        // We need to fill the comb vector gradually
        if n < max_comb {
            comb.push(Int::one());
        }
        // Update half of the pascal's triangle's row
        for i in (1..=mid).rev() {
            unsafe {
                comb_cur = &mut *(comb.get_unchecked_mut(i as usize) as *mut Int);
            }
            comb_cur.add_assign(&comb[(i - 1) as usize]);
            result += &comb[i as usize] * &fub[(n - i) as usize];
        }
        // We add a dummy value here to make sure, that the recurrence for the binomial coefficient always works
        comb[(mid + 1) as usize] = comb[mid as usize].clone();
        // Use the symmetric values of the binomial coefficient in the other half of the range
        for i in (mid + 1)..n {
            result += &comb[(n - i) as usize] * &fub[(n - i) as usize];
        }
        fub.push(result);
    }
    println!("{:?}", fub);
}
