/*
This code defines two functions: encrypt and decrypt.
The encrypt function takes a plaintext string and a shift value, and returns the ciphertext string. The decrypt function takes a ciphertext string and a shift value,
and returns the plaintext string.

*/

pub fn encrypt(text: &str, shift: u8) -> String {
    let mut result = String::new();
    for c in text.chars() {
        if c.is_ascii_alphabetic() {
            let base = if c.is_ascii_lowercase() { b'a' } else { b'A' };
            let offset = (c as u8 - base + shift) % 26;
            result.push((base + offset) as char);
        } else {
            result.push(c);
        }
    }
    result
}

pub fn decrypt(text: &str, shift: u8) -> String {
    encrypt(text, 26 - shift)
}
pub fn analyze_frequency(ciphertext: &str) -> String {
    let english_frequencies = [
        ('a', 8.167), ('b', 1.492), ('c', 2.782), ('d', 4.253), ('e', 12.702),
        ('f', 2.228), ('g', 2.015), ('h', 6.094), ('i', 6.966), ('j', 0.153),
        ('k', 0.772), ('l', 4.025), ('m', 2.406), ('n', 6.749), ('o', 7.507),
        ('p', 1.929), ('q', 0.095), ('r', 5.987), ('s', 6.327), ('t', 9.056),
        ('u', 2.758), ('v', 0.978), ('w', 2.360), ('x', 0.150), ('y', 1.974), ('z', 0.074)
    ];

    let mut best_shift = 0;
    let mut best_score = f64::MAX;

    for shift in 0..26 {
        let plaintext = decrypt(ciphertext, shift);
        let mut score = 0.0;

        for (letter, freq) in &english_frequencies {
            let count = plaintext.chars().filter(|&c| c == *letter).count() as f64;
            let text_length = plaintext.len() as f64;
            let observed_freq = (count / text_length) * 100.0;
            score += (observed_freq - freq).abs();
        }

        if score < best_score {
            best_score = score;
            best_shift = shift;
        }
    }

    decrypt(ciphertext, best_shift)
}
