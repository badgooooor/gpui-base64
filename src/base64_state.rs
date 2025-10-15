use base64::{engine::general_purpose, Engine as _};

pub struct Base64State {
    pub plain_text: String,
    pub cipher_text: String,

    pub invalid_base64: bool
}

impl Base64State {
    pub fn new() -> Self {
        Self {
            plain_text: String::new(),
            cipher_text: String::new(),
            invalid_base64: false,
        }
    }

    pub fn encode(&mut self, plain_text: &str) {
        self.plain_text = plain_text.to_string();
        self.cipher_text = general_purpose::STANDARD.encode(plain_text);
    }

    pub fn decode(&mut self, cipher_text: &str) {
        self.cipher_text = cipher_text.to_string();

        match general_purpose::STANDARD.decode(cipher_text) {
            Ok(decoded_bytes) => {
                if let Ok(decoded_string) = String::from_utf8(decoded_bytes) {
                    self.plain_text = decoded_string;
                    self.invalid_base64 = false;
                } else {
                    self.invalid_base64 = true;
                }
            },
            Err(_e) => {
                self.invalid_base64 = true;
            },
        }
    }

    pub fn reset(&mut self) {
        self.plain_text = String::new();
        self.cipher_text = String::new();
        self.invalid_base64 = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode() {
        let mut state = Base64State::new();
        state.encode("hello");

        assert_eq!(state.plain_text, "hello");
        assert_eq!(state.cipher_text, "aGVsbG8=");
        assert_eq!(state.invalid_base64, false);
    }

    #[test]
    fn test_decode() {
        let mut state = Base64State::new();
        state.decode("aGVsbG8=");

        assert_eq!(state.plain_text, "hello");
        assert_eq!(state.cipher_text, "aGVsbG8=");
        assert_eq!(state.invalid_base64, false);
    }

    #[test]
    fn test_decode_invalid() {
        let mut state = Base64State::new();
        state.decode("aGVs%%bG8=");

        assert_eq!(state.plain_text, "");
        assert_eq!(state.cipher_text, "aGVs%%bG8=");
        assert_eq!(state.invalid_base64, true);
    }

    #[test]
    fn test_reset() {
        let mut state = Base64State::new();
        state.reset();

        assert_eq!(state.plain_text, "");
        assert_eq!(state.cipher_text, "");
        assert_eq!(state.invalid_base64, false);
    }
}