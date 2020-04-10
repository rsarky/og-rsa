extern crate clap;
extern crate rand;
extern crate num_bigint as bigint;
extern crate num_traits;

use clap::{Arg, App, SubCommand, AppSettings};
use bigint::{RandBigInt, ToBigUint, BigUint, BigInt, Sign};
use num_traits::{Zero, One};
use std::cmp::Ordering;
use std::io::prelude::*;
use std::error::Error;
use std::fs::File;
use std::path::Path;

#[derive(Debug)]
struct Key {
    exponent: BigUint,
    base: BigUint
}

impl Key {
    fn to_string(&self) -> String {
        format!("{} {}", self.exponent.to_str_radix(10), self.base.to_str_radix(10))
    }
}

#[derive(Debug)]
struct KeySet {
    e: BigUint,
    d: BigUint,
    n: BigUint
}

impl KeySet {
    // TODO: should take numbits as argument
    fn new() -> KeySet {
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

    fn get_private_key(&self) -> Key {
        Key {
            exponent : self.d.clone(),
            base : self.n.clone()
        }
    }

    fn get_public_key(&self) -> Key {
        Key {
            exponent : self.e.clone(),
            base : self.n.clone()
        }
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
    s0.to_biguint().expect("Error converting to unsigned integer")
}

fn main() {
    let matches = App::new("RSA Encryption Algorithm")
        .setting(AppSettings::ArgRequiredElseHelp)
        .version("0.1")
        .author("Rohit Sarkar")
        .about("RSA implementation in Rust")
        .subcommand(SubCommand::with_name("gen")
                    .help("Generates keys"))
        .subcommand(SubCommand::with_name("encrypt")
                    .help("Encrypts plain text file using RSA algorithm"))
        .subcommand(SubCommand::with_name("decrypt")
                    .help("Decrypts a file encrypted with RSA algorithm"))
        .get_matches();

    match matches.subcommand_name() {
        Some("gen") => {
            let key_set = KeySet::new();
            let path_priv_key = "key";
            let path_pub_key = "key.pub";
            create_file(path_priv_key, key_set.get_private_key().to_string().as_bytes());
            create_file(path_pub_key, key_set.get_public_key().to_string().as_bytes());
        },
        Some("encrypt") => println!("encrypt"),
        Some("decrypt") => println!("decrypt"),
        _ => println!("dafuq")
    };

    fn create_file(filename: &str, contents: &[u8]) {
            let mut file = match File::create(filename) {
                Err(why) => panic!("Couldn't create {} file. {}", filename, why.description()),
                Ok(file) => file
            };

            match file.write_all(contents) {
                Err(why) => panic!("Couldn't write to {} file. {}", filename, why.description()),
                Ok(_) => println!("successfully created {} file.", filename)
            };
    }

}
