use std::collections::HashMap;

/// Recovering the key consists of iteratively recovering partial keys,
///    and then stitching those together.

///    Each iteration we exclude the shortest ciphertext, then anaylyse
///    the remaining texts using the slice after the last removed text.
///    This way we get data on every part of the key, only that the
///    further along the key we recover, the less ciphertexts we have to analyse.
pub fn recover_key(mut ciphertexts: Vec<Vec<u8>>) -> Vec<Option<u8>> {
    let mut key = Vec::new();
    ciphertexts.sort_by_key(|ciphertext| ciphertext.len());
    // We need a minium of two ciphertexts to compare
    while ciphertexts.len() > 1 {
        let partial_key = recover_partial_key(&ciphertexts);
        key.extend(partial_key);

        // Remove first element from list as we want to compare
        // all strings longer than that one
        let string_length = ciphertexts.first().unwrap().len();
        ciphertexts.remove(0);
        for item in &mut ciphertexts {
            // Truncate all strings so we don't recalculate already known key bits
            *item = item[string_length..].to_owned();
        }
    }
    key
}

/// Using a set of ciphertexts, analyse for the space character in plaintext to recover part of the key
fn recover_partial_key(ciphertexts: &Vec<Vec<u8>>) -> Vec<Option<u8>> {
    let shortest_text = ciphertexts.first().unwrap();
    let mut key: Vec<Option<u8>> = vec![None; shortest_text.len()];

    for (main_index, main_ciphertext) in ciphertexts.iter().enumerate() {
        let mut main_counter: HashMap<usize, usize> = HashMap::new();

        for (secondary_index, secondary_ciphertext) in ciphertexts.iter().enumerate() {
            // Dont need to XOR itself
            if main_index == secondary_index {
                continue;
            }
            // Although we know it is a space we don't know which ciphertext it came from
            let counter = track_spaces(xor(main_ciphertext, secondary_ciphertext));
            for (index, value) in &counter {
                main_counter
                    .entry(*index)
                    .and_modify(|counter_value| *counter_value += value)
                    .or_insert(*value);
            }
        }

        // Now we have tracked all the possible spaces we have seen, anchored to the index of the main ciphertext where we saw it
        // Therefore, if we have seen a space len(ciphertexts) - 1 times in a certain position, we know that because it was present
        // when XORd with each ciphertext, that it must have come from the main_ciphertext. Meaning, that position is a space in the main_plaintext
        for (index, count) in main_counter.iter() {
            if *count == (ciphertexts.len() - 1) {
                // 0x20 is the ASCII value for a space ' '
                key[*index] = Some(0x20 ^ main_ciphertext[*index])
            }
        }
    }
    key
}

/// Keep track of the spaces in text
fn track_spaces(text: Vec<u8>) -> HashMap<usize, usize> {
    let mut counter = HashMap::new();
    for (index, char) in text.iter().enumerate() {
        if is_space(char) {
            counter
                .entry(index)
                .and_modify(|value| *value += 1)
                .or_insert(1);
        }
    }
    counter
}

/// XOR two bytearrays, truncates to shortest array
fn xor(a: &Vec<u8>, b: &Vec<u8>) -> Vec<u8> {
    a.iter().zip(b.iter()).map(|(&c, &d)| c ^ d).collect()
}

/// An XOR'ed character will be a space (or any punctuation character)
///     if the resulting XOR of a ^ b is either 0x00, or an ascii letter
fn is_space(c: &u8) -> bool {
    if (c == &0x00) || (c >= &b'a' && c <= &b'z') || (c >= &b'A' && c <= &b'Z') {
        return true;
    }
    false
}
