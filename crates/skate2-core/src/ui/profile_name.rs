//! PC first-use name prompt. The mandatory startup timing is a PC addition;
//! DoRequestProfileName82759248 supplies the native title and ten-character limit.
use super::extras::TextInput;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Prompt {
    pub text: String,
    pub cursor: usize,
    pub error: Option<String>,
    accepted: bool,
    limit: usize,
    cancellable: bool,
    cancelled: bool,
}
impl Default for Prompt {
    fn default() -> Self {
        Self { text: String::new(), cursor: 0, error: None, accepted: false, limit: 10, cancellable: false, cancelled: false }
    }
}
impl Prompt {
    /// PC online nickname for `online-config.json`; Escape/B closes it unsaved.
    pub fn nickname() -> Self {
        Self { limit: super::online_config::NICKNAME_LIMIT, cancellable: true, ..Self::default() }
    }
    /// The nickname prompt holding `nickname` to edit, as much of it as the prompt can type.
    pub fn nickname_with(nickname: &str) -> Self {
        let mut prompt = Self::nickname();
        prompt.input(TextInput::Insert(nickname.trim().to_owned()));
        prompt
    }
    pub fn cancellable(&self) -> bool {
        self.cancellable
    }
    pub fn input(&mut self, input: TextInput) {
        match input {
            // Same printable host keyboard alphabet as the Extras textbox.
            TextInput::Insert(value) => {
                for c in value.chars().filter(|c| c.is_ascii_graphic() || *c == ' ') {
                    if self.text.len() == self.limit { break; }
                    self.text.insert(self.cursor, c);
                    self.cursor += 1;
                }
                self.error = None;
            }
            TextInput::Backspace if self.cursor > 0 => {
                self.cursor -= 1; self.text.remove(self.cursor);
            }
            TextInput::Delete if self.cursor < self.text.len() => { self.text.remove(self.cursor); }
            TextInput::Left => self.cursor = self.cursor.saturating_sub(1),
            TextInput::Right => self.cursor = (self.cursor + 1).min(self.text.len()),
            TextInput::Home => self.cursor = 0,
            TextInput::End => self.cursor = self.text.len(),
            TextInput::Accept => {
                if self.text.trim().is_empty() { self.error = Some("Enter a name to continue.".into()); }
                else { self.accepted = true; }
            }
            TextInput::Cancel if self.cancellable => self.cancelled = true,
            // PC addition: a mandatory prompt, which Escape/B cannot dismiss.
            _ => (),
        }
    }
    pub fn take_cancelled(&mut self) -> bool {
        std::mem::take(&mut self.cancelled)
    }
    pub fn take_accepted(&mut self) -> Option<String> {
        std::mem::take(&mut self.accepted).then(|| self.text.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::{Prompt, TextInput};
    use skate2_net::protocol::{NICKNAME_CHARACTERS, valid_nickname};

    #[test]
    fn the_nickname_prompt_makes_only_what_the_server_takes() {
        let mut prompt = Prompt::nickname();
        prompt.input(TextInput::Insert("a".repeat(40)));
        assert_eq!(prompt.text.len(), NICKNAME_CHARACTERS);
        assert!(valid_nickname(&prompt.text));
        let edit = Prompt::nickname_with("  Caf\u{e9} Racer \t");
        assert_eq!(edit.text, "Caf Racer");
        assert_eq!(edit.cursor, edit.text.len());
        assert!(valid_nickname(&edit.text));
    }
}
