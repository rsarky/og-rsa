 use num_bigint::{RandBigInt, ToBigUint, BigUint, BigInt, Sign};
 use num_traits::{Zero, One};

pub fn gen_prime(num_bits: usize) -> BigUint {
    let mut rng = rand::thread_rng();
    let mut a = rng.gen_biguint(num_bits);

    // TODO: Better way to do this?
    while !is_prime(&a) {
        a = rng.gen_biguint(num_bits);
    }
    a
}

pub fn gen_prime_above(num_bits: usize, lbound: &BigUint) -> BigUint {
    let mut rng = rand::thread_rng();
    let mut a = rng.gen_biguint(num_bits);

    while !is_prime(&(&a+lbound)) {
        a = rng.gen_biguint(num_bits);
    }
    a+lbound
}

pub fn is_prime(num: &BigUint) -> bool {
    if num & 1_u8.to_biguint().unwrap() == 0_u8.to_biguint().unwrap() {
        return false;
    }
    // Fermat's test
    // TODO: Make the number of iterations configurable
    for _i in 0..4 {
        let mut rng = rand::thread_rng();
        let a = rng.gen_biguint_range(&0_u8.to_biguint().unwrap(), &(num-1u32)); //check for inclusivity
        let result = a.modpow(&(num-1u32), num);
        if result != 1_u8.to_biguint().unwrap() {
            return false;
        }
    }
    return true;

}

pub fn mult_inverse(a: &BigUint, b: &BigUint) -> BigUint {
    // mult inverse of b under a using Extended Euclidean Algorithm
    let mut s0: BigInt = Zero::zero();
    let mut s1: BigInt = One::one();
    let mut r0 = a.clone();
    let mut r1 = b.clone();

    while r1 != Zero::zero() {
        let r2 = &r0 - (&r0/&r1)*&r1;
        let s2 = &s0 - BigInt::from_biguint(Sign::Plus, &r0/&r1)*&s1;
        r0 = r1;
        r1 = r2;
        s0 = s1;
        s1 = s2;
    }

    while s0 < Zero::zero() {
        s0 = s0 + BigInt::from_biguint(Sign::Plus, a.clone());
    }
    s0.to_biguint().expect("Error converting to unsigned integer")
}

