extern crate rand;
extern crate num_bigint as bigint;
extern crate num_traits;

use bigint::{RandBigInt, ToBigUint, BigUint, BigInt, Sign};
use num_traits::{Zero, One};
use std::cmp::Ordering;
use std::io;

struct Key {
    exponent: BigUint,
    base: BigUint
}

#[derive(Debug)]
struct KeySet {
    e: BigUint,
    d: BigUint,
    n: BigUint
}

impl KeySet {
    fn gen_key() -> KeySet {
        let mut rng1 = rand::thread_rng();
        let mut rng2 = rand::thread_rng();
        let p = gen_prime(100);
        let q = gen_prime(100);
        let n = &p*&q;
        let maxpq = match p.cmp(&q){ // TODO Find better way to do this
            Ordering::Less => &q,
            Ordering::Greater => &p,
            Ordering::Equal => &p
        };
        let phi = (&p-1u32)*(&q-1u32);

        // d need to be coprime to phi. A prime greater than max(p,q) should do
        let d = gen_prime_above(100, &maxpq);
        println!("{} {}", d, phi);
        let e = mult_inverse(&phi, &d);

        KeySet {
            e,
            d,
            n
        }
    }

    fn encrypt(&self, msg: u32) -> BigUint {
        msg.to_biguint().unwrap().modpow(&self.e, &self.n)
    }

    fn decrypt(&self, cipher: &BigUint) -> BigUint {
        cipher.modpow(&self.d, &self.n)
    }

}
// reference implementation of modular exponentiation for 32 bit numbers.
fn mod_pow(base: u32, exponent: u32, modulus: u32) -> u32 {
    if modulus == 1 {
        return 0;
    }
    let mut result = 1;
    let mut base = base % modulus;
    let mut exponent = exponent;
    while exponent != 0 {
        if (exponent & 1) != 0 {
            result = result*base % modulus;
        }
        exponent = exponent >> 1;
        base = base*base % modulus;
    }
    result
}



fn gen_prime(num_bits: usize) -> BigUint {
    let mut rng = rand::thread_rng();
    let mut a = rng.gen_biguint(num_bits);

    // TODO: Better way required.
    while !is_prime(&a) {
        a = rng.gen_biguint(num_bits);
    }
    a
}

fn gen_prime_above(num_bits: usize, lbound: &BigUint) -> BigUint {
    let mut rng = rand::thread_rng();
    let mut a = rng.gen_biguint(num_bits);

    while !is_prime(&(&a+lbound)) {
        a = rng.gen_biguint(num_bits);
    }
    a+lbound
}

fn is_prime(num: &BigUint) -> bool {
    if num & 1_u8.to_biguint().unwrap() == 0_u8.to_biguint().unwrap() {
        return false;
    }
    // Fermat's test
    // make the number of iterations configurable
    for i in 0..4 {
        let mut rng = rand::thread_rng();
        let a = rng.gen_biguint_range(&0_u8.to_biguint().unwrap(), &(num-1u32)); //check for inclusivity
        let result = a.modpow(&(num-1u32), num);
        if result != 1_u8.to_biguint().unwrap() {
          return false;
        }
    }
    return true;
    
}

fn mult_inverse(a: &BigUint, b: &BigUint) -> BigUint {
    // mult inverse of b under a
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
    s0.to_biguint().expect("Something is fishy!")
}

fn main() {
    let key_set = KeySet::gen_key();
    let mut input = String::new();
    println!("Enter a positive integer");
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");
    let input: u32 = input.trim().parse()
        .expect("Please enter a valid integer.");
    println!("You entered: {}", input);
    let enc = key_set.encrypt(input);
    let dec = key_set.decrypt(&enc);
    println!("{} {}",enc,dec);

}
