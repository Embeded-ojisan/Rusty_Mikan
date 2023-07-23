#![no_main]
#![no_std]
#![feature(const_generics, generic_const_exprs)]

use crate::font::Font;
use crate::graphics::{
    PixelColor,
    PixelWriter
};

use core::fmt;

const MAX_ROWS: usize = 500;
const MAX_COLUMNS: usize = 120;

pub struct ConsoleWriter<'a, T: PixelWriter> {
    screen: [[char; MAX_ROWS]; MAX_COLUMNS],
    cursor_row: usize,
    cursor_column: usize,
    characters: [Font; Font::MAX],
    error_character: Font,
    writer: &'a T,
}

impl<'a, T: PixelWriter> ConsoleWriter<'a, T> {
    pub fn new(writer: &'a T) -> ConsoleWriter<'a, T> {
        let screen = [['\0'; MAX_ROWS]; MAX_COLUMNS];
        let characters = Font::all();
        let error_character = Font::new('■');

        ConsoleWriter {
            screen,
            cursor_row: 0,
            cursor_column: 0,
            characters,
            error_character,
            writer,
        }
    }

    pub fn write(&mut self, string: &str) {
        for c in string.chars() {
            self.write_character(c)
        }
    }

    fn write_character(&mut self, c: char) {
        match c {
            '\n' => self.new_line(),
            _ => {
                if self.cursor_column >= MAX_COLUMNS {
                    self.new_line();
                }
                self.screen[self.cursor_column][self.cursor_row] = c;
                //へんなキャストだけど他にいい方法を知らない
                let code = c as u32 as usize;
                //範囲エラーが怖いのでget
                let font = self.characters.get(code).unwrap_or(&self.error_character);
                font.write(self.cursor_column, self.cursor_row, self.writer);
                self.cursor_column += 8;
            }
        }
    }

    fn new_line(&mut self) {
        self.cursor_column = 0;
        if self.cursor_row < MAX_ROWS - 1 {
            self.cursor_row += 20;
        } else {
            for x in 0..MAX_COLUMNS {
                for y in 0..MAX_ROWS {
                    let character = self.screen[x][y];
                    let code = character as u32 as usize;
                    let font = self.characters.get(code).unwrap_or(&self.error_character);
                    font.clear(x, y, self.writer);
                }
            }
            for y in 0..(MAX_ROWS - 1) {
                for x in 0..MAX_COLUMNS {
                    self.screen[x][y] = self.screen[x][y + 1];
                    let character = self.screen[x][y];
                    //へんなキャストだけど他にいい方法を知らない
                    let code = character as u32 as usize;
                    //範囲エラーが怖いのでget
                    let font = self.characters.get(code).unwrap_or(&self.error_character);
                    font.write(x, y, self.writer);
                }
            }
            for x in 0..MAX_COLUMNS {
                self.screen[x][MAX_ROWS - 1] = '\0';
            }
        }
    }
}

impl<'a, T: PixelWriter> fmt::Write for ConsoleWriter<'a, T> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write(s);
        Ok(())
    }
}
