File Hash Generator by Parker Beckman
-This is a Rust tool that computes cryptographic hashes (SHA-256, SHA-1, MD5) of a file.
-Useful for quickly verifying file integrity, allowing for a more streamline security scheme.

Usage
```bash
cargo run -- [--hash <sha256 | sha1 | md5>] <file_path>

Examples:

cargo run -- test.txt               #Generates a SHA-256 hash by default
    or 
cargo run -- --hash sha1 test.txt   #Uses --hash flag to set hash algorithm to SHA-1
```

Issue resolution:
When making this program, I ran into an issue where the MD5 package was not properly importing. So, I had to fix it by by computing the MD5 directly and then extracting the digest bytes.
