## Caesar cipher tool
by Robin Arun

## Introduction

This tool is used to encrypt and decrypt messages using the Caesar cipher. The Caesar cipher is a simple substitution cipher in which each letter in the plaintext is replaced by a letter some fixed number of positions down the alphabet.

## Usage

To use the tool, run the following:

1. Run the following command:

cargo run --message "Your message" --encrypt/decrypt --shift <shift value> --output-format <output format>

- The `encrypt/decrypt` argument specifies whether to encrypt or decrypt the message.
- The `shift` value represents the number of positions to shift each letter in the alphabet.
- The `output-format` can be either plain or hex. The default value is plain.


2. For example, to encrypt the message "Hello, world!" with a shift of 10 and output the ciphertext in hexadecimal format, you would run:

cargo run --message "Hello, world!" --encrypt --shift 10 --output-format hex

This would output the following ciphertext:

48656C6C6F2C20776F726C6421

<img width="777" alt="DE" src="https://github.com/nogibjj/python_v_rustcli_rt/assets/143838819/f968022f-0aef-4312-b92f-de1ad782ff1f">

## Conclusion

This tool provides a simple and user-friendly way to encrypt and decrypt messages using the Caesar cipher. It can serve various purposes, such as sending secret messages to friends or securing data from unauthorized access.

## Performance Comparison (Python vs. Rust)

In this specific use case, we compared the performance of Python and Rust implementations of the Caesar cipher tool:

### Python Implementation

- CPU Usage: 36.9%
- Memory Usage: 45.7%
- Elapsed Time: 192.3ms

### Rust Implementation

- CPU Usage: 0.80%
- Memory Usage: 45.394943%
- Elapsed Time: 34.070125ms

The observed performance differences between Python and Rust can be attributed to their fundamental design and implementation characteristics. Python, while easy to use, tends to be slower due to its dynamic typing and interpreted nature. Rust, being a compiled systems programming language, excels in performance and efficiency, making it a better choice for high-performance tasks.

## Sources

- [GitHub Repository](https://github.com/Burland313/DecoderEncoder)
