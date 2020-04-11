# og_rsa
Implementation of the original RSA algorithm in Rust.

MADE FOR LEARNING AND FUN. SHOULD **NOT** BE USED IN PRODUCTION.


**og_rsa** is pretty much a direct implementation of the [original RSA paper](http://people.csail.mit.edu/rivest/Rsapaper.pdf), hence the name.

## Usage
I am using _Cargo_ as my build system. Clone this repository and run a `cargo build`.

NOTE: **og_rsa** can currently handle only small(< ~128 byte) files.

### Key generation
The first step is to generate  a public/private key pair using `./rsa generate`.
This will generate a pair of files: `key` (private key) and `key.pub` (public key).

### Encryption
```
./rsa encrypt -k key.pub -f a-test-file.txt
```

This will generate a file with the same name as the one being encrypted but with
`.enc` appended. In the above example the file generated will be `a-test-file.txt.enc`.

### Decryption
```
./rsa encrypt -k key.pub -f a-test-file.txt.enc
```

This will generate a file with the same name as the one being decrypted but with
`.dec` appended. In the above example the file generated will be `a-test-file.txt.enc.dec`.

### `./rsa`

```
RSA Encryption Algorithm 0.1
Rohit Sarkar
RSA implementation in Rust

USAGE:
    rsa [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    decrypt     Decrypts a file encrypted with the RSA algorithm
    encrypt     Encrypts a plain text file using the RSA algorithm
    generate    Generates a pair of public and private keys (key and key.pub)
    help        Prints this message or the help of the given subcommand(s)
```

## TODO
- [ ] Add support for arbitrary sized files.
- [ ] Tests
- [ ] Padding
- [ ] Make things configurable (number of bits in the key, key file name, public key suffix etc.)
- [ ] Better error handling
