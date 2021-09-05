mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    // The `console.log` is quite polymorphic, so we can bind it with multiple
    // signatures. Note that we need to use `js_name` to ensure we always call
    // `log` in JS.
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}
// Next let's define a macro that's like `println!`, only it works for
// `console.log`. Note that `println!` doesn't actually work on the wasm target
// because the standard library currently just eats all output. To get
// `println!`-like behavior in your app you'll likely want a macro like this.

macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert("Hello, wasm-game-of-life!");
    alert(&format!("Hello, {}!", name));
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}
impl Cell {
    fn toggle(&mut self) {
        *self = match *self {
            Cell::Dead => Cell::Alive,
            Cell::Alive => Cell::Dead,
        };
    }
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}


/// Public methods, exported to JavaScript.
extern crate js_sys;
// extern crate web_sys;
const SPACESHIP:  [(u32,u32);9] = [ (0,0), (1,0), (2,0), (3,0), (0,1), (4,1), (0,2), (1,3), (4,3)];
const RPENTOMINO: [(u32,u32); 5] = [ (0,0), (0,1), (1,1), (2,1), (1,2)];
const PIHEPTOMINO: [(u32,u32); 7] = [ (0,0), (1,0), (2,0), (0,1), (2,1), (0,2), (2,2)];
const GLIDER: [(u32,u32);5] =  [(1,2), (2,3), (3,1), (3,2), (3,3)];
#[wasm_bindgen]
impl Universe {
    pub fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }
                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (column + delta_col) % self.width;
                let idx = self.get_index(neighbor_row, neighbor_col);
                count += self.cells[idx] as u8;
            }
        }
        count
    }
    
}
#[wasm_bindgen]
impl Universe {
    pub fn tick(&mut self) {
        // let _timer = Timer::new("Universe::tick");
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbors = self.live_neighbor_count(row, col);

                let next_cell = match (cell, live_neighbors) {
                    // Rule 1: Any live cell with fewer than two live neighbours
                    // dies, as if caused by underpopulation.
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    // Rule 2: Any live cell with two or three live neighbours
                    // lives on to the next generation.
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    // Rule 3: Any live cell with more than three live
                    // neighbours dies, as if by overpopulation.
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    // Rule 4: Any dead cell with exactly three live neighbours
                    // becomes a live cell, as if by reproduction.
                    (Cell::Dead, 3) => Cell::Alive,
                    // All other cells remain in the same state.
                    (otherwise, _) => otherwise,
                };

                next[idx] = next_cell;
            }
        }

        self.cells = next;
    }

    // ...
}
use std::fmt;

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == Cell::Dead { '◻' } else { '◼' };
                write!(f, "{}", symbol)?;
            }
            writeln!(f, "\n")?;
        }

        Ok(())
    }
}

#[wasm_bindgen]
impl Universe {
    // ...

    pub fn new(w: u32, h: u32) -> Universe {
        let width = w;
        let height = h;

        let cells = (0..width * height)
            .map(|_i| {
                if js_sys::Math::random() < 0.2 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect();
        console_log!("Creating a {} x {} Life Universe", width, height);

        Universe {
            width,
            height,
            cells,
        }     
    }
    
    pub fn render(&self) -> String {
        self.to_string()
    }

    fn clear_grid( &mut self) {
        for i in 0..self.cells.len()  {
            self.cells[i] = Cell::Dead;
        }
    }

    pub fn make_spaceship(&mut self)  {
        // let targets = [ (0,0), (1,0), (2,0), (3,0), (0,1), (4,1), (0,2), (1,3), (4,3)];
        self.clear_grid();
        Universe::set_cells(self, &SPACESHIP);
    }
    pub fn make_rpentamino(&mut self)  {
        // let targets = [ (0,0), (1,0), (2,0), (3,0), (0,1), (4,1), (0,2), (1,3), (4,3)];
        // self.clear_grid();
        Universe::set_cells(self, &RPENTOMINO);
    }
    pub fn make_piheptomino(&mut self) {
        // let targets = [ (0,0), (1,0), (2,0), (3,0), (0,1), (4,1), (0,2), (1,3), (4,3)];
        self.clear_grid();
        Universe::set_cells(self, &PIHEPTOMINO);
    }

    pub fn make_glider(&mut self)  {
        self.clear_grid();
        Universe::set_cells(self, &GLIDER);
    }
    
}

#[wasm_bindgen]
impl Universe {
    // ...

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> *const Cell {
        // console_log!("Reference to {} x {} Life Universe ", self.width, self.height);
        self.cells.as_ptr()
    }
     // ...

    /// Set the width of the universe.
    ///
    /// Resets all cells to the dead state.
    pub fn set_width(&mut self, width: u32) {
        console_log!(" Set Width {}", width);
        self.width = width;
        self.cells = (0..width * self.height).map(|_i| Cell::Dead).collect();
    }

    /// Set the height of the universe.
    ///
    /// Resets all cells to the dead state.
    pub fn set_height(&mut self, height: u32) {
        console_log!(" Set Height {}", height);
        self.height = height;
        self.cells = (0..self.width * height).map(|_i| Cell::Dead).collect();
    }
    /// Set the dimensions of the universe.
    ///
    /// Resets all cells to the dead state.
    pub fn set_dimensions(&mut self, width: u32, height: u32) {
        console_log!(" Set Width {} and Height {}", width, height);
        self.width = width;
        self.height = height;
        self.cells = (0..self.width * height).map(|_i| Cell::Dead).collect();
    }

    pub fn toggle_cell(&mut self, row: u32, column: u32) {
        let idx = self.get_index(row, column);
        self.cells[idx].toggle();
    }
}
impl Universe {
    /// Get the dead and alive values of the entire universe.
    pub fn get_cells(&self) -> &[Cell] {
        &self.cells
    }

    /// Set cells to be alive in a universe by passing the row and column
    /// of each cell as an array.
    pub fn set_cells(&mut self, cells: &[(u32, u32)]) {
        self.clear_grid();
        let middle = (self.width * (self.height/2) + self.width/2) as usize;
        for (col, row) in cells.iter().cloned() {
            let idx = self.get_index(row, col);
            self.cells[idx+middle] = Cell::Alive;
        }
    }

}

extern crate web_sys;
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
