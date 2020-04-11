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
use std::fs;
use std::path::Path;
use std::str;

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
        let maxpq = match p.cmp(&q) {
            Ordering::Less => &q,
            _ => &p
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

    fn from_keys(priv_key: &Key, pub_key: &Key) -> KeySet {
        if priv_key.base != pub_key.base {
            panic!("Incorrect key pair.");
        }
        let e = pub_key.exponent.clone();
        let d = priv_key.exponent.clone();
        let n = priv_key.base.clone();
        KeySet {
            e,
            d,
            n
        }
    }

    fn encrypt(&self, msg: &BigUint) -> BigUint {
        msg.modpow(&self.e, &self.n)
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

static key_file_name: &'static str = "key";
static public_key_suffix: &'static str = "pub";
static encrypted_file_suffix: &'static str = "enc";
static decrypted_file_suffix: &'static str = "dec";
fn main() {
    let matches = App::new("RSA Encryption Algorithm")
        .setting(AppSettings::ArgRequiredElseHelp)
        .version("0.1")
        .author("Rohit Sarkar")
        .about("RSA implementation in Rust")
        .subcommand(SubCommand::with_name("gen")
                    .help("Generates keys"))
        .subcommand(SubCommand::with_name("encrypt")
                    .about("Encrypts plain text file using RSA algorithm")
                    .arg(Arg::with_name("key")
                         .short("k")
                         .long("key")
                         .required(true)
                         .value_name("KEY_FILE")
                         .help("Name of key files")
                         .takes_value(true))
                    .arg(Arg::with_name("file")
                         .short("f")
                         .long("file")
                         .required(true)
                         .value_name("FILE")
                         .help("Plain text file to encode")
                         .takes_value(true)))
        .subcommand(SubCommand::with_name("decrypt")
                    .help("Decrypts a file encrypted with RSA algorithm")
                    .arg(Arg::with_name("key")
                         .short("k")
                         .long("key")
                         .required(true)
                         .value_name("KEY_FILE")
                         .help("Name of key files")
                         .takes_value(true))
                    .arg(Arg::with_name("file")
                         .short("f")
                         .long("file")
                         .required(true)
                         .value_name("FILE")
                         .help("Plain text file to decode")
                         .takes_value(true)))
        .get_matches();

    match matches.subcommand_name() {
        Some("gen") => {
            let key_set = KeySet::new();
            create_file(key_file_name.to_string(), key_set.get_private_key().to_string().as_bytes());;
            create_file(format!("{}.{}",key_file_name, public_key_suffix), key_set.get_public_key().to_string().as_bytes());
        },
        Some("encrypt") => {
            let matches = matches.subcommand_matches("encrypt");
            let key_file = matches.unwrap().value_of("key").unwrap();
            let key_set =read_key_files(key_file.to_string());
            let file = matches.unwrap().value_of("file").unwrap();
            encrypt_file(&file, &key_set);
        },
        Some("decrypt") => {
            let matches = matches.subcommand_matches("decrypt");
            let key_file = matches.unwrap().value_of("key").unwrap();
            let key_set =read_key_files(key_file.to_string());
            let file = matches.unwrap().value_of("file").unwrap();
            decrypt_file(&file, &key_set);
        },
        _ => println!("dafuq")
    };
}

fn create_file(filename: String, contents: &[u8]) {
    let mut file = match File::create(&filename) {
        Err(why) => panic!("Couldn't create {} file. {}", filename, why.description()),
        Ok(file) => file
    };

    match file.write_all(contents) {
        Err(why) => panic!("Couldn't write to {} file. {}", filename, why.description()),
        Ok(_) => println!("successfully created {} file.", filename)
    };
}

fn read_key_files(filename: String) -> KeySet {
    let priv_key = read_key_file(filename.to_string());
    let pub_key = read_key_file(format!("{}.{}",key_file_name, public_key_suffix));
    KeySet::from_keys(&priv_key, &pub_key)
}
fn read_key_file(filename: String) -> Key {
    let contents = fs::read_to_string(filename)
        .expect("Couldnt read file.");
    let contents: Vec<&[u8]> = contents.split(' ').map(|x| x.as_bytes()).collect();
    if contents.len() != 2 {
        panic!("Incorrect key file format");
    }
    let exponent = BigUint::parse_bytes(contents[0], 10)
        .expect("Error parsing key file");
    let base = BigUint::parse_bytes(contents[1], 10)
        .expect("Error parsing key file");
    Key {
        exponent,
        base
    }
}

fn encrypt_file(file: &str, key_set: &KeySet) {
    let contents = fs::read_to_string(file)
        .expect("Couldnt read file.");
    let encoded = BigUint::from_bytes_be(contents.as_bytes());
    println!("{:#?}", encoded);
    let encrypted = key_set.encrypt(&encoded);
    create_file(format!("{}.{}", file, encrypted_file_suffix), &encrypted.to_bytes_be());
}

fn decrypt_file(file: &str, key_set: &KeySet) {
    let contents = fs::read(file)
        .expect("Couldnt read file.");
    let decoded = BigUint::from_bytes_be(&contents);
    let decrypted = key_set.decrypt(&decoded);
    println!("{:#?}", decrypted);
    let decrypted = decrypted.to_bytes_be();
    let decrypted = str::from_utf8(&decrypted);
    let decrypted = decrypted.unwrap().to_string();
    create_file(format!("{}.{}", file, decrypted_file_suffix), decrypted.as_bytes());
}
