extern crate cfg_if;
extern crate fixedbitset;
extern crate js_sys;
extern crate wasm_bindgen;
extern crate web_sys;

mod utils;

use fixedbitset::FixedBitSet;
use std::fmt;
use wasm_bindgen::prelude::*;
use web_sys::console;

pub struct Timer<'a> {
    name: &'a str,
}

impl<'a> Timer<'a> {
    pub fn new(name: &'a str) -> Timer<'a> {
        console::time_with_label(name);
        Timer { name }
    }
}

impl<'a> Drop for Timer<'a> {
    fn drop(&mut self) {
        console::time_end_with_label(self.name);
    }
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: FixedBitSet,
}

impl Universe {
    fn init(&mut self) {
        let size = self.width * self.height;
        for i in 0..size {
            let random = js_sys::Math::random();
            let state = if random < 0.5 { true } else { false };
            self.cells.set(i as usize, state);
        }
    }

    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    // get the dead and alive values of the entire universe.
    pub fn get_cells(&self) -> &FixedBitSet {
        &self.cells
    }

    // set cells to be alive in a universe by passing the row and column
    // of each cell as an array.
    pub fn set_cells(&mut self, cells: &[(u32, u32)]) {
        for (row, col) in cells.iter().cloned() {
            let idx = self.get_index(row, col);
            self.cells.set(idx, true);
        }
    }

    // set the width of the universe, all cells set to false
    pub fn set_width(&mut self, width: u32) {
        self.width = width;
        let size = (width * self.height) as usize;
        self.cells = FixedBitSet::with_capacity(size);
    }

    // set the height of the universe, all cells set to false
    pub fn set_height(&mut self, height: u32) {
        self.height = height;
        let size = (height * self.width) as usize;
        self.cells = FixedBitSet::with_capacity(size);
    }

    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;

        let north = if row == 0 { self.height - 1 } else { row - 1 };

        let south = if row == self.height - 1 { 0 } else { row + 1 };

        let west = if column == 0 {
            self.width - 1
        } else {
            column - 1
        };

        let east = if column == self.width - 1 {
            0
        } else {
            column + 1
        };

        let nw = self.get_index(north, west);
        count += self.cells[nw] as u8;

        let n = self.get_index(north, column);
        count += self.cells[n] as u8;

        let ne = self.get_index(north, east);
        count += self.cells[ne] as u8;

        let w = self.get_index(row, west);
        count += self.cells[w] as u8;

        let e = self.get_index(row, east);
        count += self.cells[e] as u8;

        let sw = self.get_index(south, west);
        count += self.cells[sw] as u8;

        let s = self.get_index(south, column);
        count += self.cells[s] as u8;

        let se = self.get_index(south, east);
        count += self.cells[se] as u8;

        count
    }
}

// public methods, exported to JavaScript.
#[wasm_bindgen]
impl Universe {
    pub fn tick(&mut self) {
        // messure how long a trick takes
        // let _timer = Timer::new("Universe::tick");

        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbors = self.live_neighbor_count(row, col);

                next.set(
                    idx,
                    match (cell, live_neighbors) {
                        (true, x) if x < 2 => false,
                        (true, 2) | (true, 3) => true,
                        (true, x) if x > 3 => false,
                        (false, 3) => true,
                        (otherwise, _) => otherwise,
                    },
                );
            }
        }
        self.cells = next;
    }

    pub fn restart(&mut self) {
        self.init()
    }

    pub fn new(width: u32, height: u32) -> Universe {
        utils::set_panic_hook();

        let cells = FixedBitSet::with_capacity((width * height) as usize);

        let mut u = Universe {
            width,
            height,
            cells,
        };
        u.init();

        log!("Initial universe:\n{}", u.to_string());

        u
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> *const u32 {
        self.cells.as_slice().as_ptr()
    }

    pub fn toggle_cell(&mut self, row: u32, column: u32) {
        let idx = self.get_index(row, column);
        self.cells.toggle(idx);
    }
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for idx in 0..self.cells.len() {
            let symbol = if self.cells[idx] == false {
                '◻'
            } else {
                '◼'
            };
            write!(f, "{}", symbol)?;
            if (idx + 1) % (self.width as usize) == 0 {
                write!(f, "\n")?;
            }
        }
        Ok(())
    }
}
