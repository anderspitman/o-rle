extern crate cfg_if;
extern crate wasm_bindgen;

mod utils;

use cfg_if::cfg_if;
use wasm_bindgen::prelude::*;

use std::iter::FromIterator;

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, o-rle!");
}

#[wasm_bindgen]
pub fn parse(rle_text: &str) -> PatternIter {
    let mut pattern = Pattern::new();
    let mut parser = Parser::new();
    parser.parse(rle_text);

    pattern.width = parser.rows[0].len();
    pattern.height = parser.rows.len();

    // TODO: modify the existing vec rather than making a new one
    pattern.grid = vec![0; pattern.width * pattern.height];

    let mut row_index = 0;
    let mut col_index = 0;

    for row in parser.rows {
        for cell in row {
            let i = (row_index * pattern.width) + col_index;
            pattern.grid[i] = cell;

            col_index += 1;
            if col_index == pattern.width {
                col_index = 0;
                row_index += 1;
            }
        }
    }

    PatternIter::new(pattern)
}

#[wasm_bindgen]
pub struct Pattern {
    grid: Vec<u8>,
    width: usize,
    height: usize,
}

#[wasm_bindgen]
impl Pattern {
    pub fn new() -> Pattern {
        Pattern {
            grid: Vec::new(),
            width: 0,
            height: 0,
        }
    }
}

impl Pattern {
    pub fn get_grid(&self) -> Vec<u8> {
        self.grid.clone()
    }
}

#[wasm_bindgen]
pub struct PatternIter {
    pattern: Pattern,
    row_index: usize,
}

impl PatternIter {
    fn new(pattern: Pattern) -> PatternIter {
        PatternIter {
            pattern: pattern,
            row_index: 0,
        }
    }
}

#[wasm_bindgen]
impl PatternIter {
    pub fn next(&mut self) -> Option<Vec<u8>> {
        if self.row_index < self.pattern.height {

            let mut v = vec![0; self.pattern.width];
            let i = self.row_index * self.pattern.width;
            v.copy_from_slice(&self.pattern.grid[i..i+self.pattern.width]);

            self.row_index += 1;

            Some(v)
        }
        else {
            None
        }
    }
}
pub struct Parser {
    pub rows: Vec<Vec<u8>>,
    digits: Vec<char>,
    current_row: Vec<u8>,
    row_length: usize,
}

impl Parser {
    pub fn new() -> Parser {
        Parser {
            digits: Vec::new(),
            rows: Vec::new(),
            current_row: Vec::new(),
            row_length: 0,
        }
    }

    pub fn parse(&mut self, text: &str) {
        for line in text.split("\n") {
            self.parse_line(line);
        }

        println!("{:?}", self.digits);
        println!("{:?}", self.rows);
    }

    fn parse_line(&mut self, line: &str) {
        if line.starts_with('#') {
            println!("comment: {}", line);
        }
        else if line.starts_with('x') {
            println!("rule: {}", line);
        }
        else {
            self.parse_pattern_line(line);
        }
    }

    fn parse_pattern_line(&mut self, line: &str) {
        println!("pattern: {}", line);

        //let mut row = Vec::new();

        for ch in line.chars() {
            match ch {
                'b' => {
                    let num = self.digit_val();

                    for _ in 0..num {
                        self.current_row.push(0);
                    }
                },
                'o' => {
                    let num = self.digit_val();
                    for _ in 0..num {
                        self.current_row.push(1);
                    }
                },
                '$' => {
                    let row = self.current_row.clone();
                    self.current_row = Vec::new();
                    self.row_length = row.len();
                    self.rows.push(row);
                },
                '!' => {
                    let mut row = self.current_row.clone();
                    self.current_row = Vec::new();

                    let len = row.len();
                    if len < self.row_length {
                        for _ in len..self.row_length {
                            row.push(0);
                        }
                    }
                    self.rows.push(row);
                },
                digit => {
                    self.digits.push(digit);
                },
            }
        }


    }

    fn digit_val(&mut self) -> i32 {
        if self.digits.len() == 0 {
            return 1;
        }
        else {
            let num_str = String::from_iter(&self.digits);
            let num = num_str.parse::<i32>().unwrap();
            self.digits.clear();
            return num;
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parser_constructor() {
        let _parser = Parser::new();
    }

    #[test]
    fn parse_glider() {
        let rle_text = "\
#N Glider
#O Richard K. Guy
#C The smallest, most common, and first discovered spaceship. Diagonal, has period 4 and speed c/4.
#C www.conwaylife.com/wiki/index.php?title=Glider
x = 3, y = 3, rule = B3/S23
bob$2bo$3o!";

        let mut parser = Parser::new();
        parser.parse(rle_text);
        assert_eq!(parser.rows, [[0, 1, 0], [0, 0, 1], [1, 1, 1]]);
    }

    #[test]
    fn parse_glider_gun() {
        let rle_text = "\
#N Gosper glider gun
#O Bill Gosper
#C A true period 30 glider gun.
#C The first known gun and the first known finite pattern with unbounded growth.
#C www.conwaylife.com/wiki/index.php?title=Gosper_glider_gun
x = 36, y = 9, rule = B3/S23
24bo11b$22bobo11b$12b2o6b2o12b2o$11bo3bo4b2o12b2o$2o8bo5bo3b2o14b$2o8b
o3bob2o4bobo11b$10bo5bo7bo11b$11bo3bo20b$12b2o!";

        let mut parser = Parser::new();
        parser.parse(rle_text);
        let expected = [
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1],
            [1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 1, 1, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        ];

        for (y, row) in expected.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                assert_eq!(parser.rows[y][x], *cell as u8);
            }
        }
    }

    #[test]
    fn parse_function() {
        let rle_text = "\
#N Glider
#O Richard K. Guy
#C The smallest, most common, and first discovered spaceship. Diagonal, has period 4 and speed c/4.
#C www.conwaylife.com/wiki/index.php?title=Glider
x = 3, y = 3, rule = B3/S23
bob$2bo$3o!";

        let mut pattern_iter = parse(rle_text);

        let mut row = pattern_iter.next();
        assert_eq!(row.unwrap(), [0,1,0]);
        row = pattern_iter.next();
        assert_eq!(row.unwrap(), [0,0,1]);
        row = pattern_iter.next();
        assert_eq!(row.unwrap(), [1,1,1]);
        row = pattern_iter.next();
        assert_eq!(row, None);
    }
}
