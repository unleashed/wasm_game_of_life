extern crate cfg_if;
extern crate wasm_bindgen;
extern crate web_sys;
extern crate gol;

mod utils;

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

// FIXME: ensure with constant expressions that this Cell enum
// exactly matches the Cell in the gol crate.
#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

impl From<gol::Cell> for Cell {
    fn from(c: gol::Cell) -> Cell {
        match c {
            gol::Cell::Alive => Cell::Alive,
            _ => Cell::Dead,
        }
    }
}

#[wasm_bindgen]
#[repr(transparent)]
pub struct Universe(gol::Universe);

impl Universe {
    /// Get the dead and alive values of the entire universe.
    pub fn get_cells(&self) -> &[Cell] {
        let cells = self.0.get_cells();
        unsafe { std::mem::transmute(cells) }
    }

    /// Set cells to be alive in a universe by passing the row and column
    /// of each cell as an array.
    pub fn set_cells(&mut self, cells: &[(u32, u32)]) {
        self.0.set_cells(cells);
    }
}

/// Public methods, exported to JavaScript.
#[wasm_bindgen]
impl Universe {
    pub fn tick(&mut self) {
        self.0.tick()
    }

    pub fn new() -> Self {
        utils::set_panic_hook();

        Self(gol::Universe::new())
    }

    pub fn width(&self) -> u32 {
        self.0.width()
    }

    /// Set the width of the universe.
    ///
    /// Resets all cells to the dead state.
    pub fn set_width(&mut self, width: u32) {
        self.0.set_width(width)
    }

    pub fn height(&self) -> u32 {
        self.0.height()
    }

    /// Set the height of the universe.
    ///
    /// Resets all cells to the dead state.
    pub fn set_height(&mut self, height: u32) {
        self.0.set_height(height)
    }

    pub fn cells(&self) -> *const Cell {
        let cells = self.0.cells();
        unsafe { std::mem::transmute(cells) }
    }

    pub fn toggle_cell(&mut self, row: u32, column: u32) {
        self.0.toggle_cell(row, column);
    }
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}
