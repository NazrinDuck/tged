use std::io::{self, Read};

struct ContentBuf {
    buffer: String,
    input: [u8; 1],
}

impl ContentBuf {
    pub fn interact(&mut self) -> io::Result<()> {
        loop {
            io::stdin().read_exact(&mut self.input)?;
            match self.input[0] {
                char if char.is_ascii_graphic() => self.buffer.push(char as char),
                b'\x1b' => break,
                _ => (),
            }
        }
        Ok(())
    }
}
