use groestl::{Digest, Groestl256};
use hex_literal::hex;

fn main() {
    // The original input message from the paper (before padding)
    let message = hex!("");

    // Initialize the Groestl-256 hasher
    let mut hasher = Groestl256::default();
    hasher.update(&message);

    // Compute the hash
    let hash = hasher.finalize();

    // The expected hash from the GROESTL paper
    let expected_groestl_hash = hex!("1A52D11D550039BE16107F9C58DB9EBCC417F16F736ADB2502567119F0083467");

    // The expected hash from the KUPYNA paper
    let expected_kupyna_hash = hex!("CD5101D1CCDF0D1D1F4ADA56E888CD724CA1A0838A3521E7131D4FB78D0F5EB6");

    // Print the computed hash
    println!("Computed Hash: {:X?}", hash);

    // Manually compare the computed hash with the expected groestl hash
    let mut mismatch_groestl = false;
    for (i, (&computed_byte, &expected_byte)) in hash.iter().zip(expected_groestl_hash.iter()).enumerate() {
        if computed_byte != expected_byte {
            println!(
                "Mismatch at byte {}: computed = {:02X}, expected = {:02X}",
                i, computed_byte, expected_byte
            );
            mismatch_groestl = true;
            break; // Stop after the first mismatch
        }
    }

    if !mismatch_groestl {
        println!("Groestl Hash matches the expected value!");
    } else {
        println!("Groestl Hash does not match the expected value.");
    }

    // Manually compare the computed hash with the expected kupyna hash
    let mut mismatch_kupyna = false;
    for (i, (&computed_byte, &expected_byte)) in hash.iter().zip(expected_kupyna_hash.iter()).enumerate() {
        if computed_byte != expected_byte {
            println!(
                "Mismatch at byte {}: computed = {:02X}, expected = {:02X}",
                i, computed_byte, expected_byte
            );
            mismatch_kupyna = true;
            break; // Stop after the first mismatch
        }
    }

    if !mismatch_kupyna {
        println!("Kupyna Hash matches the expected value!");
    } else {
        println!("Kupyna Hash does not match the expected value.");
    }
}