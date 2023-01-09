
const BASE64_CHARACTERS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

pub fn encode_to_str(data: &str) -> String {
    let mut input: String = data.to_owned();
    let mut result: String = "".to_owned();
    let mut padding: String = "".to_owned();

    let padding_number = input.len() % 3;
    if padding_number > 0 {
        for _ in 0..(3 - padding_number) {
            padding.push_str("=");
            input.push_str("\0")
        }
    }

    for c in (0..input.len()).step_by(3) {
        let mut n: u32 = (input.as_bytes()[c] as u32) << 16
            | (input.as_bytes()[c + 1] as u32) << 8
            | (input.as_bytes()[c + 2] as u32);

        let mut b: [u8; 4] = [0; 4];
        for i in (0..4).rev() {
            b[i] = BASE64_CHARACTERS.as_bytes()[(n % 64) as usize];
            n = n >> 6;
        }

        result.push_str(&String::from_utf8(b.to_vec()).unwrap());
    }

    result.push_str(&padding);
    return result;
}
