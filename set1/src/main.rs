
mod base64;

fn challenge1(input: &str) -> String {
    let input_processed: String = (0..input.len())
        .step_by(2)
        .map(|c| u8::from_str_radix(&input[c..c+2], 16).unwrap() as char)
        .collect::<String>();
    println!("Input: {}", input_processed);
    base64::encode_to_str(&input_processed)
}

fn main() {
    let input1: &str = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let result1: &str = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
    let computed_result1: String = challenge1(input1);
    println!("Challenge 1: {}", computed_result1);
    assert!(result1 == computed_result1, "[-] Challenge 1 is wrong!");
    println!("[+] Challenge 1 solved!");

}
