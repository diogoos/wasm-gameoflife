extern crate console_error_panic_hook;
use std::f64; 
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

const RESOLUTION: u32 = 10;
const COLS: u32 = 800 / RESOLUTION;
const ROWS: u32 = 600 / RESOLUTION;

use rand::Rng;
use web_sys::CanvasRenderingContext2d;


#[wasm_bindgen]
pub struct CellApp {
  state: [[bool; ROWS as usize]; COLS as usize],
  context: CanvasRenderingContext2d,
  
  pub background_inverted: bool,
  pub show_grid: bool,
}

#[wasm_bindgen]
impl CellApp {
  
  #[wasm_bindgen(constructor)]
  pub fn new() -> Self {
    // setup error logging
    console_error_panic_hook::set_once();
    
    // load canvas context
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
      .dyn_into::<web_sys::HtmlCanvasElement>()
      .map_err(|_| ())
      .unwrap();

    let context = canvas
      .get_context("2d")
      .unwrap()
      .unwrap()
      .dyn_into::<web_sys::CanvasRenderingContext2d>()
      .unwrap();

    // initialize the app
    let mut app = CellApp {
      background_inverted: false,
      show_grid: false,

      state: [[false; ROWS as usize]; COLS as usize],
      context,
    };

    app.randomize_board();

    app
  }

  pub fn randomize_board(&mut self) {
    let mut rng = rand::thread_rng();
    for i in 0..COLS {
      for j in 0..ROWS {
        // set state to random value
        self.state[i as usize][j as usize] = rng.gen_range(0..10) == 0;
      }
    }

    self.update();
  }

  pub fn clear_board(&mut self) {
    self.state = [[false; ROWS as usize]; COLS as usize];
  }

  fn count_neighbors(&self, x: u32, y: u32) -> u32 {
    let mut sum = 0;
    
    for x_shift in -1i32..2i32 {
      for y_shift in -1i32..2i32 {
        // wrap around logic
        let col = (x as i32 + x_shift + COLS as i32) % COLS as i32; 
        let row = (y as i32 + y_shift + ROWS as i32) % ROWS as i32;

        sum += self.state[col as usize][row as usize] as u32;
      }
    }
    sum -= self.state[x as usize][y as usize] as u32;

    sum
  }

  // run logic to make a new generation
  pub fn update(&mut self) {
    let mut new_state = self.state.clone();

    for i in 0..COLS {
      for j in 0..ROWS {
        let neighbors = self.count_neighbors(i, j); // count live neighbors
        let current_cell = self.state[i as usize][j as usize];

        if !current_cell && neighbors == 3 {
          new_state[i as usize][j as usize] = true;
        } else if current_cell && (neighbors < 2 || neighbors > 3) {
          new_state[i as usize][j as usize] = false;
        }
      }
    }

    self.state = new_state;
  }

  // render each frame
  pub fn draw(&mut self) {
    // erase frame first
    if self.background_inverted {
      self.context.set_fill_style(&JsValue::from_str("black"));
      self.context.fill_rect(0., 0., 800., 600.);
      self.context.set_fill_style(&JsValue::from_str("white"));
      self.context.set_stroke_style(&JsValue::from_str("white"));
    } else {
      self.context.clear_rect(0., 0., 800., 600.);
      self.context.set_fill_style(&JsValue::from_str("black"));
      self.context.set_stroke_style(&JsValue::from_str("black"));
    }
    

    for i in 0..COLS {
      for j in 0..ROWS {
        let x = i * RESOLUTION;
        let y = j * RESOLUTION;

        if self.state[i as usize][j as usize] {
          self.context.fill_rect(x as f64, y as f64, RESOLUTION as f64, RESOLUTION as f64);   
        } else if self.show_grid {
          self.context.stroke_rect(x as f64, y as f64, RESOLUTION as f64, RESOLUTION as f64);
        }
      }
    }
  }

  pub fn toggle_cell(&mut self, x_click: u32, y_click: u32) {
    let x = x_click / RESOLUTION;
    let y = y_click / RESOLUTION;

    self.state[x as usize][y as usize] ^= true; // toggle cell
  }
}