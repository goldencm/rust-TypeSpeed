#![macro_use]
extern crate linecount;
extern crate indexed_line_reader;
use std::collections::HashMap;
use indexed_line_reader::*;
use std::fs::*;
use std::io::{BufRead, BufReader, Seek,Read, SeekFrom, Write};

use rand::Rng;
use conrod_core::widget_ids;
use std::path::PathBuf;
use std::env;
use std::fs;
use linecount::count_lines;
widget_ids! {
    pub struct Ids {
        empty,
        canvas,
        guess_button,
        count_text,
        info_text,
        textbox,
        zero,
        one,
        two,
        three,
        four,
        five,
        six,
        seven,
        eight,
        nine,
        end,
    }
}

pub struct Data {
    attempt : String,

    
}
impl Data {
    pub fn new() -> Data {
        Data {
            attempt: String::new()
        }
    }
    pub fn new_attempt(&mut self, attempt: &str) {
        self.attempt = attempt.to_owned();
    }

    pub fn get_attempt(&self) -> String {
        self.attempt.clone()
    }
}
pub struct Game {
    num_words : usize,
    indexed_line_reader : IndexedLineReader<std::io::BufReader<std::fs::File>>,
    pub score : usize,
    pub failed : usize,
    //Word, Speed, X Position
    pub strings : HashMap<usize, (String, i32, f64)>,
    pub speedMultiplier : i32,
}

impl Game {

    pub fn new() -> Self {
        let lines: usize = count_lines(std::fs::File::open("words_alpha.txt").unwrap()).unwrap();
        let file_reader = OpenOptions::new().read(true).open("words_alpha.txt").expect("Unable to open file reader");
        let mut indexed_line_reader = IndexedLineReader::new(BufReader::new(file_reader), 100);

        
        Game {
            num_words : lines,
            indexed_line_reader : indexed_line_reader,
            score : 0,
            failed : 0,
            strings : HashMap::new(),
            speedMultiplier : 1,
        }
    }


    pub fn populate_hash_map(&mut self) {
        for row in 0..10 {
            let word = self.next_word();
            self.strings.insert(row, (word, rand::thread_rng().gen_range(0, 10) * self.speedMultiplier, 0 as f64));
        }
    }

    pub fn next_word(&mut self) -> String{
        self.indexed_line_reader.seek(SeekFrom::Start(rand::thread_rng().gen_range(0, self.num_words as u64)));
        let mut line = String::new();
        self.indexed_line_reader.read_line(&mut line).unwrap();
        line.truncate(line.len() - 2);
        println!("{}", line); // apanese
        return line;
    }

    pub fn end(&self) -> bool {
        self.failed >= 3
    }
}


pub fn load_font(font: &str) -> PathBuf {
    use super::find_folder::Search::KidsThenParents;

    let fonts_dir = KidsThenParents(3, 5)
        .for_folder("fonts")
        .expect("`fonts/` not found!");
    let font_path = fonts_dir.join(font);

    font_path
}

