// https://adventofcode.com/2025/day/7

#[path = "../../helpers.rs"]
mod helpers;

#[cfg(test)]
mod tests;

struct TachyonManifold<'a> {
    bytes: &'a mut [u8],
    num_rows: usize,
    num_cols: usize,
}

struct TachyonManifoldCoordinate {
    current: char,
    above: char,
    left: char,
    update: Box<dyn Fn(char)>,
    right: char,
    row: usize,
    col: usize,
}

impl<'a> TachyonManifold<'a> {
    fn new(string: &'a mut str) -> Self {
        let num_cols = string.find('\n').expect("");
        let num_rows = string.len() - (num_cols + 1);
        let bytes = unsafe { string.as_bytes_mut() };
        TachyonManifold {
            bytes,
            num_cols,
            num_rows,
        }
    }
    fn get(&self, row: usize, col: usize) -> char {
        self.bytes[row * self.num_cols + col] as char
    }
    fn set(&mut self, row: usize, col: usize, c: char) {
        // critical for upholding the unsafe guarantee above
        assert!(c.is_ascii());
        self.bytes[row * self.num_cols + col] = c as u8;
    }

    fn coordinates(&mut self) -> impl Iterator<Item = TachyonManifoldCoordinate> {
        self.bytes.iter_mut().enumerate().map(|(i, current)| {
            let x = self.get(0, 0);
            return TachyonManifoldCoordinate {
                current,
                right: 'a',
                left: 'b',
                above: 'c',
                row: 0,
                col: 0,
                //row: i / self.num_cols,
                //col: i % self.num_cols,
            };
        })
    }
}

fn main() {
    let mut input = helpers::get_input(2025, 7).unwrap();
    let mut manifold = TachyonManifold::new(&mut input);
    for TachyonManifoldCoordinate { byte, row, col } in manifold.coordinates() {
        let above = manifold.get(row - 1, col);
        *byte = 'h' as u8;
    }
}
