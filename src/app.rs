type Key = Vec<Option<u8>>;

/// App holds the state of the application
pub struct App {
    /// Current value of the input box
    pub input: Option<char>,
    /// Position of cursor in the editor area.
    pub cursor_position_x: usize,
    pub cursor_position_y: usize,

    pub min_x: usize,
    pub min_y: usize,
    pub max_x: usize,
    pub max_y: usize,

    /// History of recorded messages
    pub ciphertexts: Vec<Vec<u8>>,
    pub partial_key: Key,
    pub output: String,
}

impl App {
    pub fn new(ciphertexts: Vec<Vec<u8>>, partial_key: Key, output: String) -> App {
        App {
            ciphertexts,
            partial_key,
            output,
            input: None,
            cursor_position_x: 0,
            cursor_position_y: 0,
            min_x: 0,
            min_y: 0,
            max_x: 0,
            max_y: 0,
        }
    }
}

impl App {
    pub fn move_cursor_left(&mut self) {
        self.cursor_position_x = if self.cursor_position_x <= 0 {
            self.max_x
        } else {
            self.cursor_position_x - 1
        };
    }

    pub fn move_cursor_right(&mut self) {
        self.cursor_position_x = if self.cursor_position_x >= self.max_x {
            0
        } else {
            self.cursor_position_x + 1
        };
    }

    pub fn move_cursor_up(&mut self) {
        self.cursor_position_y = if self.cursor_position_y <= 0 {
            self.max_y
        } else {
            self.cursor_position_y - 1
        };
    }

    pub fn move_cursor_down(&mut self) {
        self.cursor_position_y = if self.cursor_position_y >= self.max_y {
            0
        } else {
            self.cursor_position_y + 1
        };
    }

    pub fn move_cursor_home(&mut self) {
        self.cursor_position_x = 0;
    }

    pub fn move_cursor_end(&mut self) {
        self.cursor_position_x = self.max_x;
    }

    pub fn move_cursor_to_position(&mut self, x: usize, y: usize) {
        // Validates input to ensure cursor is within bounds
        if x < self.min_x || x > self.max_x || y < self.min_y || y > self.max_y {
            return;
        }
        self.cursor_position_x = x - self.min_x;
        self.cursor_position_y = y - self.min_y;
    }

    pub fn enter_char(&mut self, new_char: char) {
        self.input = Some(new_char);
        self.update_ciphertexts(new_char);
        self.move_cursor_right();
    }

    pub fn delete_char(&mut self, shift_left: bool) {
        let is_not_cursor_leftmost = self.cursor_position_x != 0;
        if is_not_cursor_leftmost {
            // Method "remove" is not used on the saved text for deleting the selected char.
            // Reason: Using remove on String works on bytes instead of the chars.
            // Using remove would require special care because of char boundaries.

            self.partial_key[self.cursor_position_x] = None;
            if shift_left {
                self.move_cursor_left()
            };
        }
    }


    fn update_ciphertexts(&mut self, c: char) {
        // Discover cipher text position
        let (text_index, char_index) = self.get_ciphertext_position();
        // TODO: Change to know the current index position (See if part of the same Line)
        // XOR key with char
        let key_byte = self.partial_key.get_mut(char_index);
        if key_byte.is_none() {
            return;
        }
        let key_byte = key_byte.unwrap();
        // WILL PANIC IF NOT CORRECT
        let ciphertext = self.ciphertexts.get(text_index);
        if ciphertext.is_none() {
            return;
        }
        let ciphertext = ciphertext.unwrap();
        let char = ciphertext.get(char_index);
        if char.is_none() {
            return;
        }
        *key_byte = Some(self.ciphertexts[text_index][char_index] ^ c as u8);
    }

    fn get_ciphertext_position(&self) -> (usize, usize) {
        let (mut text_index, mut char_index) = (self.cursor_position_y, self.cursor_position_x);
        let mut current_start = 0;
        let mut next_cipher_start = 0;
        for (i, ciphertext) in self.ciphertexts.iter().enumerate() {
            let rows_per_ciphertext = ciphertext.len() / self.max_x;
            next_cipher_start += rows_per_ciphertext + 1;
            if self.cursor_position_y < next_cipher_start {
                text_index = i;
                let multiplier = self.cursor_position_y - current_start;
                char_index = multiplier * self.max_x + self.cursor_position_x;
                break;
            }
            current_start = next_cipher_start;
        }
        (text_index, char_index)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_ciphertext_position_0_0() {
        let app = get_app_for_test(0, 0);
        assert_eq!(app.get_ciphertext_position(), (0, 0));
    }
    #[test]
    fn test_get_ciphertext_position_0_1() {
        let app = get_app_for_test(0, 1);
        assert_eq!(app.get_ciphertext_position(), (0, 4));
    }
    #[test]
    fn test_get_ciphertext_position_0_3() {
        let app = get_app_for_test(0, 3);
        assert_eq!(app.get_ciphertext_position(), (2, 0));
    }
    #[test]
    fn test_get_ciphertext_position_1_4() {
        let app = get_app_for_test(1, 4);
        assert_eq!(app.get_ciphertext_position(), (3, 1));
    }
    #[test]
    fn test_get_ciphertext_position_3_6() {
        let app = get_app_for_test(3, 6);
        assert_eq!(app.get_ciphertext_position(), (3, 11));
    }
    #[test]
    fn test_get_ciphertext_position_0_8() {
        let app = get_app_for_test(0, 8);
        assert_eq!(app.get_ciphertext_position(), (3, 16));
    }
    #[test]
    fn test_get_ciphertext_position_1_9() {
        let app = get_app_for_test(1, 9);
        assert_eq!(app.get_ciphertext_position(), (4, 1));
    }

    fn get_app_for_test(x: usize, y: usize) -> App {
        let ciphertexts = get_ciphertexts_for_text();
        let partial_key = get_partial_key_for_test();
        let mut app = App::new(ciphertexts, partial_key, "test.json".to_string());
        (app.max_x, app.max_y) = (4, 9);
        (app.cursor_position_x, app.cursor_position_y) = (x, y);
        app
    }

    fn get_ciphertexts_for_text() -> Vec<Vec<u8>> {
        vec![
            vec![96, 96, 96, 96, 96, 96],
            vec![96, 96, 96],
            vec![96],
            vec![
                96, 96, 96, 96, 96, 96, 96, 96, 96, 96, 96, 96, 96, 96, 96, 96, 96,
            ],
            vec![96, 96],
        ]
    }

    fn get_partial_key_for_test() -> Vec<Option<u8>> {
        vec![
            Some(1),
            Some(2),
            None,
            Some(4),
            Some(5),
            Some(6),
            None,
            Some(8),
            Some(9),
            Some(10),
            Some(11),
            None,
        ]
    }
}
