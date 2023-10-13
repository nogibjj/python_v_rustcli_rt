## Caesar cipher tool
by Titus Robin Arun

![Caesar cipher tool screenshot](https://github.com/titusrobin/rust-data-engineering_rt/assets/143838819/1a4df0eb-08a9-4bdf-bc7e-389a1422199d)

## Introduction

This tool is used to encrypt and decrypt messages using the Caesar cipher. The Caesar cipher is a simple substitution cipher in which each letter in the plaintext is replaced by a letter some fixed number of positions down the alphabet.

## Usage

To use the tool, simply run the following command:

cargo run --message "Your message" --encrypt/decrypt --shift <shift value> --output-format <output format>

The encrypt/decrypt argument specifies whether to encrypt or decrypt the message. The shift value is the number of positions to shift each letter in the alphabet. The output format can be either plain or hex. The default value is plain.

For example, to encrypt the message "Hello, world!" with a shift of 10 and output the ciphertext in hexadecimal format, you would run the following command:

cargo run --message "Hello, world!" --encrypt --shift 10 --output-format hex
This would output the following ciphertext:

48656C6C6F2C20776F726C6421

## Conclusion
This tool is a simple and easy-to-use way to encrypt and decrypt messages using the Caesar cipher. It can be used for a variety of purposes, such as sending secret messages to friends or securing data from unauthorized access.

###Sources
github.com/Burland313/DecoderEncoder
