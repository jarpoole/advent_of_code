// https://adventofcode.com/2025/day/7

use ndarray::{Array2, ArrayBase};

#[path = "../../helpers.rs"]
mod helpers;

#[cfg(test)]
mod tests;

#[derive(PartialEq, Copy, Clone)]
enum Tachyon {
    Splitter,
    Beam(u64),
    Start,
    Empty,
}

struct TachyonManifold(Array2<Tachyon>);

struct TachyonSimulationResult {
    num_beam_splits: u64,
    num_timelines: u64,
}

impl TachyonManifold {
    fn new(string: &str) -> Self {
        let mut rows = 1;
        let array = ArrayBase::from_iter(string.trim().chars().filter_map(|char| match char {
            '.' => Some(Tachyon::Empty),
            'S' => Some(Tachyon::Start),
            '^' => Some(Tachyon::Splitter),
            '\n' => {
                rows += 1;
                None
            }
            c => panic!("should never encounter char '{}' in the input", c),
        }));
        let columns = (string.len() - rows) / rows;
        return TachyonManifold(
            array
                .into_shape_with_order((rows, columns))
                .expect("calculated dimensions should always match data"),
        );
    }

    fn simulate(&mut self) -> TachyonSimulationResult {
        let mut num_beam_splits = 0;
        let dims = self.0.dim();
        for row in 0..dims.0 {
            for col in 0..dims.1 {
                // Note that we cannot use simpler if/else or even .then_some() because
                // the subtraction expressions evaluate too early and underflow otherwise
                let above = (row > 0)
                    .then(|| self.0.get((row - 1, col)))
                    .flatten()
                    .copied();
                let above_left = (row > 0 && col > 0)
                    .then(|| self.0.get((row - 1, col - 1)))
                    .flatten()
                    .copied();
                let above_right = (row > 0 && col < dims.1)
                    .then(|| self.0.get((row - 1, col + 1)))
                    .flatten()
                    .copied();
                let left = (col > 0)
                    .then(|| self.0.get((row, col - 1)))
                    .flatten()
                    .copied();
                let right = (col < dims.1)
                    .then(|| self.0.get((row, col + 1)))
                    .flatten()
                    .copied();
                let cell = self
                    .0
                    .get_mut((row, col))
                    .expect("should always be in bounds");
                if matches! {above, Some(Tachyon::Beam(..))} && *cell == Tachyon::Splitter {
                    num_beam_splits += 1;
                }

                // start case
                if above == Some(Tachyon::Start) {
                    *cell = Tachyon::Beam(1)
                }

                // beam propagation
                if let Some(Tachyon::Beam(above_timelines)) = above
                    && *cell != Tachyon::Splitter
                {
                    let cell_timelines = match *cell {
                        Tachyon::Beam(t) => t,
                        _ => 0,
                    };
                    *cell = Tachyon::Beam(above_timelines + cell_timelines)
                }

                // splitter to the left
                if left == Some(Tachyon::Splitter)
                    && let Some(Tachyon::Beam(splitter_timelines)) = above_left
                {
                    let cell_timelines = match *cell {
                        Tachyon::Beam(t) => t,
                        _ => 0,
                    };
                    *cell = Tachyon::Beam(cell_timelines + splitter_timelines);
                }

                // splitter to the right
                if right == Some(Tachyon::Splitter)
                    && let Some(Tachyon::Beam(splitter_timelines)) = above_right
                {
                    let cell_timelines = match *cell {
                        Tachyon::Beam(t) => t,
                        _ => 0,
                    };
                    *cell = Tachyon::Beam(cell_timelines + splitter_timelines);
                }
            }
        }
        let manifold_output_row = self
            .0
            .rows()
            .into_iter()
            .rev()
            .next()
            .map(|row| row.into_iter())
            .expect("tachyon manifold should always have a non-zero number of rows");
        return TachyonSimulationResult {
            num_beam_splits,
            num_timelines: manifold_output_row
                .filter_map(|cell| match cell {
                    Tachyon::Beam(num_timelines) => Some(num_timelines),
                    _ => None,
                })
                .sum(),
        };
    }
}

fn main() {
    let input = helpers::get_input(2025, 7).unwrap();
    let mut manifold = TachyonManifold::new(&input);
    let result = manifold.simulate();
    println!("Number of beam splits: {}", result.num_beam_splits);
    println!("Number of active timelines: {}", result.num_timelines);
}
