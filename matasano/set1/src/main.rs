
pub mod set1;
pub mod xor;
fn main() {

    let hex = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d".to_string();

    println!("Challenge 1 Result: {}", set1::challenge1::hex_to_base64(hex) == "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t".to_string());
    println!("Challenge 2 Result: {}", set1::challenge2::xor_compare(
        "1c0111001f010100061a024b53535009181c".to_string(),
        "686974207468652062756c6c277320657965".to_string(),
        "746865206b696420646f6e277420706c6179"
    ))
}
