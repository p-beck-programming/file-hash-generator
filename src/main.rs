use std::fs::File;          //File handling 
use std::io::{self, Read};  //Reading File Contents
use sha2::{Digest, Sha256}; //SHA-256 hashing
use sha1::{Sha1};
use md5;
use hex;                    //hex encoding
use clap::{Parser, ValueEnum};

#[derive(Debug, Clone, ValueEnum)]
enum HashType {
    Sha256,
    Sha1,
    Md5,
}

#[derive(Parser)]
#[command(about = "Computes a cryptographic hash of file")]
struct Args {
    //File path to hash
    file_path: String,
    //Hash algorithm to use 
    #[arg(long, default_value = "sha256")]
    hash : HashType

}

fn main() -> io::Result<()> {
    let args = Args::parse(); //Parse CLI args
    
    let mut file = File::open(&args.file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    //Compute hash based on selected type
    let hash_hex = match args.hash {
        HashType::Sha256 => {
            let mut hasher = Sha256::new();
            hasher.update(&buffer);
            hex::encode(hasher.finalize())
        }
        HashType::Sha1 => {
            let mut hasher = Sha1::new();
            hasher.update(&buffer);
            hex::encode(hasher.finalize())
        }
        HashType::Md5 => {
            let digest = md5::compute(&buffer);
            hex::encode(digest.0)
        }
    };

    println!("{} hash of {}: {}", 
                format!("{:?}", args.hash), //Display hash type
                args.file_path, hash_hex); 

    Ok(())
}