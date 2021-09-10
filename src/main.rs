mod conv;

fn set01_challenge01() {
    let bin = conv::from_hex("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d").unwrap();
    let b64 = conv::to_base64(bin.as_slice()).unwrap();
    assert_eq!(b64, "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t")
}

fn main() {
    set01_challenge01();
}