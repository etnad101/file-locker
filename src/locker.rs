fn wrapping_shift_left(value: &u8, shifts: u8) -> u8 {
    let mut new_value = *value;
    for _ in 0..shifts {
        let leading_digit = new_value >> 7;
        new_value = new_value << 1;
        new_value = new_value | leading_digit;
    }
    new_value
}

fn wrapping_shift_right(value: &u8, shifts: u8) -> u8 {
    let mut new_value = *value;
    for _ in 0..shifts {
        let leading_digit = new_value & 0b0000_0001;
        new_value = new_value >> 1;
        new_value = new_value | (leading_digit << 7);
    }
    new_value
}

pub fn encrypt_file(file: Vec<u8>, key: &[u8]) -> Vec<u8> {
    let mut key_index = 0;
    let mut dir = 0;

    file.iter()
        .map(|item| {
            let new_item: u8;
            let shifts = key[key_index] - 32;
            if dir == 0 {
                new_item = wrapping_shift_left(item, shifts);
                dir = 1
            } else {
                new_item = wrapping_shift_right(item, shifts);
                dir = 0
            }
            key_index += 1;
            if key_index == key.len() {
                key_index = 0;
            }
            new_item
        })
        .collect()
}

pub fn decrypt_file(file: Vec<u8>, key: &[u8]) -> Vec<u8> {
    let mut key_index = 0;
    let mut dir = 1;

    file.iter()
        .map(|item| {
            let new_item;
            let shifts = key[key_index] - 32;
            if dir == 0 {
                new_item = wrapping_shift_left(item, shifts);
                dir = 1
            } else {
                new_item = wrapping_shift_right(item, shifts);
                dir = 0
            }
            key_index += 1;
            if key_index == key.len() {
                key_index = 0;
            }
            new_item
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    mod shift {
        use super::*;

        #[test]
        fn test_shift_left() {
            let res = wrapping_shift_left(&0b0000_0001, 8);
            let expected = 0b0000_0001;

            assert_eq!(res, expected)
        }

        #[test]
        fn test_shift_right() {
            let res = wrapping_shift_right(&0b0000_0001, 8);
            let expected = 0b0000_0001;

            assert_eq!(res, expected)
        }
    }

    mod enc_dec {
        use super::*;

        #[test]
        fn test() {
            let key = b"abcdef";
            let message: Vec<u8> = vec![90, 91, 92, 93, 94, 95, 96, 97, 98, 99, 100];
            let encrypted = encrypt_file(message.clone(), key);
            let dectypted = decrypt_file(encrypted, key);
            assert_eq!(message, dectypted);
        }
    }
}
