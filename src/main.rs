extern crate clap;
extern crate rand;
extern crate num_bigint as bigint;
extern crate num_traits;

use og_rsa::{KeySet, create_file, read_key_files, encrypt_file, decrypt_file};
use clap::{Arg, App, SubCommand, AppSettings};

fn main() {
    let matches = App::new("RSA Encryption Algorithm")
        .setting(AppSettings::ArgRequiredElseHelp)
        .version("0.1")
        .author("Rohit Sarkar")
        .about("RSA implementation in Rust")
        .subcommand(SubCommand::with_name("generate")
                    .about("Generates a pair of public and private keys (key and key.pub)")
                    .help("Generates keys"))
        .subcommand(SubCommand::with_name("encrypt")
                    .about("Encrypts a plain text file using the RSA algorithm")
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
                    .about("Decrypts a file encrypted with the RSA algorithm")
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

    static KEY_FILE_NAME: &'static str = "key";
    static PUBLIC_KEY_SUFFIX: &'static str = "pub";

    match matches.subcommand_name() {
        Some("generate") => {
            let key_set = KeySet::new();
            create_file(KEY_FILE_NAME.to_string(), key_set.get_private_key().to_string().as_bytes());;
            create_file(format!("{}.{}",KEY_FILE_NAME, PUBLIC_KEY_SUFFIX), key_set.get_public_key().to_string().as_bytes());
        },
        Some("encrypt") => {
            let matches = matches.subcommand_matches("encrypt");
            let key_file = matches.unwrap().value_of("key").unwrap();
            let key_set =read_key_files(key_file.to_string(), PUBLIC_KEY_SUFFIX.to_string());
            let file = matches.unwrap().value_of("file").unwrap();
            encrypt_file(&file, &key_set);
        },
        Some("decrypt") => {
            let matches = matches.subcommand_matches("decrypt");
            let key_file = matches.unwrap().value_of("key").unwrap();
            let key_set =read_key_files(key_file.to_string(), PUBLIC_KEY_SUFFIX.to_string());
            let file = matches.unwrap().value_of("file").unwrap();
            decrypt_file(&file, &key_set);
        },
        _ => println!("dafuq")
    };
}
