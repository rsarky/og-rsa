extern crate rand;
extern crate num_bigint as bigint;
extern crate num_traits;

use bigint::{RandBigInt, ToBigUint, BigUint, BigInt, Sign};
use num_traits::{Zero, One};
use std::cmp::Ordering;
use std::io;

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

fn encrypt(msg: u32, e: &BigUint, n: &BigUint) -> BigUint {
    // These will replaced by 2 large randomly generated primes
    // TODO Should this be bigint?
    msg.to_biguint().unwrap().modpow(&e, &n)
}

fn decrypt(cipher: &BigUint, d: &BigUint, n: &BigUint) -> BigUint {
    cipher.modpow(&d, &n)
}

fn gen_key() -> (BigUint, BigUint, BigUint) {
    let mut rng1 = rand::thread_rng();
    let mut rng2 = rand::thread_rng();
    let p = gen_prime();
    let q = gen_prime();
    let n = &p*&q;
    let maxpq = match p.cmp(&q){ // TODO Find better way to do this
        Ordering::Less => &q,
        Ordering::Greater => &p,
        Ordering::Equal => &p
    };
    let phi = (&p-1u32)*(&q-1u32);
    let d = rng1.gen_biguint_range(&maxpq, &(maxpq + rng2.gen_biguint(100)));
    let e = mult_inverse(&phi, &d);
    (e,d,n) // e,n => public key. d,n => private key.
}

fn gen_prime() -> BigUint {
    let num_bits = 100;
    let mut rng = rand::thread_rng();
    let mut a = rng.gen_biguint(num_bits);
    while !is_prime(&a) {
        a = rng.gen_biguint(num_bits);
    }
    a
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
    // mult inverse of a under b
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
        s0 = s0 + BigInt::from_biguint(Sign::Plus, b.clone());
        println!("falal");
    }
    s0.to_biguint().expect("Something is fishy!")
}

fn main() {
    let (e,d,n) = gen_key();
    let mut input = String::new();
    println!("Enter a positive integer");
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");
    let input: u32 = input.trim().parse()
        .expect("Please enter a valid integer.");
    println!("You entered: {}", input);
    let enc = encrypt(input, &e, &n);
    let dec = decrypt(&enc, &d, &n);
    println!("{} {} {}", e, d, n);
    println!("{} {}",enc,dec);
}
