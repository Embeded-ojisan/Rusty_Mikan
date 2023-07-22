#![no_main]
#![no_std]

use crate::font::{Font};

pub struct ConsoleWriter {
    screen: [[char; ConsoleWriter::MAX_ROWS]; ConsoleWriter::MAX_COLUMNS],
    cursor_row: usize,
    cursor_column: usize,
    characters: [Font; Font::MAX],
    error_character: Font,
}

impl ConsoleWriter {
    const MAX_ROWS: usize = 25;
    const MAX_COLUMNS: usize = 80;

    pub fn new() -> ConsoleWriter {
        let screen = [['\0'; ConsoleWriter::MAX_ROWS]; ConsoleWriter::MAX_COLUMNS];
        let characters = Font::all();
        let error_character = Font::new('■');

        ConsoleWriter {
            screen: screen,
            cursor_row: 0,
            cursor_column: 0,
            characters: characters,
            error_character: error_character,
        }
    }

    pub fn write(&mut self, string: &str, writer: &impl PixelWriter) {
        for c in string.chars() {
            self.write_character(c, writer)
        }
    }

    fn write_character(&mut self, c: char, writer: &impl PixelWriter) {
        match c {
            '\n' => self.new_line(writer),
            _ => {
                if self.cursor_column >= ConsoleWriter::MAX_COLUMNS {
                    self.new_line(writer);
                }
                self.screen[self.cursor_column][self.cursor_row] = c;
                //へんなキャストだけど他にいい方法を知らない
                let code = c as u32 as usize;
                //範囲エラーが怖いのでget
                let font = self.characters.get(code).unwrap_or(&self.error_character);
                writer.write(self.cursor_column, self.cursor_row, font);
                self.cursor_column += 1;
            }
        }
    }

    fn new_line(&mut self, writer: &impl PixelWriter) {
        self.cursor_column = 0;
        if self.cursor_row < ConsoleWriter::MAX_ROWS - 1 {
            self.cursor_row += 1;
        } else {
            for x in 0..ConsoleWriter::MAX_COLUMNS {
                for y in 0..ConsoleWriter::MAX_ROWS {
                    writer.clear(x, y);
                }
            }
            for y in 0..(ConsoleWriter::MAX_ROWS - 1) {
                for x in 0..ConsoleWriter::MAX_COLUMNS {
                    self.screen[x][y] = self.screen[x][y + 1];
                    let character = self.screen[x][y];
                    //へんなキャストだけど他にいい方法を知らない
                    let code = character as u32 as usize;
                    //範囲エラーが怖いのでget
                    let font = self.characters.get(code).unwrap_or(&self.error_character);
                    writer.write(x, y, font);
                }
            }
            for x in 0..ConsoleWriter::MAX_COLUMNS {
                self.screen[x][ConsoleWriter::MAX_ROWS - 1] = '\0';
            }
        }
    }
}

