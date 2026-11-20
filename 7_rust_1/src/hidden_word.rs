const MAX_LENGTH_WORD: usize = 50;
const ALPHABET_LENGTH: usize = 26;

pub enum PressState {
    Success,
    WasPressed,
    Failure
}

pub struct HiddenWord {
    masked_word: [char; MAX_LENGTH_WORD],
    length: usize,
    visible_indices: [bool; MAX_LENGTH_WORD],
    pressed_letters: [bool; ALPHABET_LENGTH]
}

impl HiddenWord {
    pub fn new(word: &str) -> Self {
        if word.len() > MAX_LENGTH_WORD {
            panic!("The word \"{word}\" is too big")
        }
        let mut chars = word.chars();
        Self {
            masked_word: std::array::from_fn(|_| chars.next().unwrap_or('-')),
            length: word.len(),
            visible_indices: [false; MAX_LENGTH_WORD],
            pressed_letters: [false; ALPHABET_LENGTH]
        }
    }
    pub fn press_letter(&mut self, letter: char) -> PressState {
        let letter = letter.to_ascii_lowercase();
        let offset = (letter as u8 - b'a') as usize;

        if self.pressed_letters[offset] {
            return PressState::WasPressed;
        }
        self.pressed_letters[offset] = true;

        let mut state = PressState::Failure;

        for (index, &value) in self.masked_word[..self.length].iter().enumerate() {
            if value == letter {
                self.visible_indices[index] = true;
                state = PressState::Success;
            }
        }

        state
    }
    pub fn render_word(&self) -> String {
        self.masked_word[..self.length]
            .iter()
            .zip(&self.visible_indices)
            .map(|(&ch, &visible)| if visible { ch } else { '-' })
            .collect()
    }
    pub fn is_solved(&self) -> bool {
        self.visible_indices[..self.length].iter().all(|&visible| visible)
    }
}
