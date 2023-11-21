pub struct Label<'a>(&'a str);

impl Label<'_> {
    pub fn from(s: &str) -> Label {
        Label(s)
    }

    pub fn get_label(&self) -> &str {
        &self.0[1..self.0.len() - 1]
    }
}
