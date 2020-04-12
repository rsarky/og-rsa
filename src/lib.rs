mod util;

use num_bigint::{BigUint};
use std::cmp::Ordering;
use std::fs::{self, File};
use std::str;
use std::io::prelude::*;
use std::error::Error;

#[derive(Debug)]
pub struct Key {
    exponent: BigUint,
    base: BigUint
}

impl Key {
    pub fn to_string(&self) -> String {
        format!("{} {}", self.exponent.to_str_radix(10), self.base.to_str_radix(10))
    }
}

#[derive(Debug)]
pub struct KeySet {
    e: BigUint,
    d: BigUint,
    n: BigUint
}

impl KeySet {
    // TODO: make bitsize of key configurable
    pub fn new() -> KeySet {
        let num_bits = 1024;
        let p = util::gen_prime(num_bits/2);
        let q = util::gen_prime(num_bits/2);
        let n = &p*&q;
        println!("No. of bits in key: {}", n.bits());
        let maxpq = match p.cmp(&q) {
            Ordering::Less => &q,
            _ => &p
        };
        //totient of n.
        let phi = (&p-1u32)*(&q-1u32);

        // d need to be coprime to phi. A prime greater than max(p,q) should do
        let d = util::gen_prime_above(100, &maxpq);
        let e = util::mult_inverse(&phi, &d);

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

    pub fn get_private_key(&self) -> Key {
        Key {
            exponent : self.d.clone(),
            base : self.n.clone()
        }
    }

    pub fn get_public_key(&self) -> Key {
        Key {
            exponent : self.e.clone(),
            base : self.n.clone()
        }
    }

}

static ENCRYPTED_FILE_SUFFIX: &'static str = "enc";
static DECRYPTED_FILE_SUFFIX: &'static str = "dec";

pub fn encrypt_file(file: &str, key_set: &KeySet) {
    let contents = fs::read_to_string(file)
        .expect("Couldnt read file.");
    let encoded = BigUint::from_bytes_be(contents.as_bytes());
    let encrypted = key_set.encrypt(&encoded);
    create_file(format!("{}.{}", file, ENCRYPTED_FILE_SUFFIX), &encrypted.to_bytes_be());
}

pub fn decrypt_file(file: &str, key_set: &KeySet) {
    let contents = fs::read(file)
        .expect("Couldnt read file.");
    let decoded = BigUint::from_bytes_be(&contents);
    let decrypted = key_set.decrypt(&decoded);
    let decrypted = decrypted.to_bytes_be();
    let decrypted = str::from_utf8(&decrypted);
    let decrypted = decrypted.unwrap().to_string();
    create_file(format!("{}.{}", file, DECRYPTED_FILE_SUFFIX), decrypted.as_bytes());
}

pub fn create_file(filename: String, contents: &[u8]) {
    let mut file = match File::create(&filename) {
        Err(why) => panic!("Couldn't create {} file. {}", filename, why.description()),
        Ok(file) => file
    };

    match file.write_all(contents) {
        Err(why) => panic!("Couldn't write to {} file. {}", filename, why.description()),
        Ok(_) => println!("successfully created {} file.", filename)
    };
}

pub fn read_key_files(filename: String, public_key_suffix: String) -> KeySet {
    let priv_key = read_key_file(filename.to_string());
    let pub_key = read_key_file(format!("{}.{}", filename, public_key_suffix));
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
