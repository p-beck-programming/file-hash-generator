use std::env;               //cmd arguments
use std::fs::File;          //File handling 
use std::io::{self, Read};  //Reading File Contents
use sha2::{Digest, Sha256}; //SHA-256 hashing
use hex;                    //hex encoding

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();  //Collect args into a vector
    if args.len() != 2 {                            //Expects 1 arg
        eprintln!("Usage: {} <file_path>", args[0]);
        std::process::exit(1);                      //Error handling
    } 
    let file_path = &args[1];                       //Get the file path
    let mut file = File::open(file_path)?;          //Open file
    let mut buffer = Vec::new();                    //Buffer to hold contents
    file.read_to_end(&mut buffer)?;                 //Read entire file into buffer

    let mut hasher = Sha256::new();                 //Create new hasher instance
    hasher.update(&buffer);                         //Feed file contents to hasher
    let result = hasher.finalize();                 //Compute final result

    let hash_hex = hex::encode(result);
    println!("SHA-256 hash of {}: {}", file_path, hash_hex);

    Ok(())
}