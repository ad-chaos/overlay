use crate::strip_ansi;

pub struct Buffer {
    styled: String,
    plain: String,
}

impl Buffer {
    pub fn line_indicies(&self, start: usize, len: usize) -> impl Iterator<Item = (usize, &str)> {
        self.styled.lines().skip(start).take(len).enumerate()
    }
}

impl From<String> for Buffer {
    fn from(value: String) -> Self {
        let plain = strip_ansi(&value);
        Buffer {
            styled: value,
            plain,
        }
    }
}
