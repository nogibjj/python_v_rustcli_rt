/*

To run:

cargo run --  --message "Off to the bunker. Every person for themselves" --encrypt --shift 10

To decrypt:

cargo run --  --message "Ypp dy dro lexuob. Ofobi zobcyx pyb drowcovfoc" --decrypt --shift 10

*/


use caeser_cipher_cli::{decrypt, encrypt};
use clap::Parser;

/// CLI tool to encrypt and decrypt messages using the caeser cipher
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Encrypt the message
    #[arg(short, long)]
    encrypt: bool,

    /// decrypt the message
    #[arg(short, long)]
    decrypt: bool,

    /// The message to encrypt or decrypt
    #[arg(short, long)]
    message: String,

    /// The shift to use for the cipher
    /// Must be between 1 and 25, the default is 3
    #[arg(short, long, default_value = "3")]
    shift: u8,

    /// The output format of the ciphertext
    #[arg(short, long, default_value = "plain")]
    output_format: String,
}

// run it
fn main() {
    let args = Args::parse();
    
    // Get the output format
    let output_format = args.output_format;

    // Encrypt or decrypt the message
    let output_message = if args.encrypt {
        encrypt(&args.message, args.shift)
    } else if args.decrypt {
        decrypt(&args.message, args.shift)
    } else {
        println!("Please specify either --encrypt or --decrypt");
        return;
    };

    // Output the ciphertext
    match output_format.as_str() {
        "plain" => println!("{}", output_message),
        "hex" => println!("{}", output_message.chars().map(|c| format!("{:02X}", c as u8)).collect()),
        _ => panic!("Unsupported output format"),
    }
}
