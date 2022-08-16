use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame, Terminal,
};
use unicode_width::UnicodeWidthStr;
mod ui;
struct WordFreq {
    word: String,
    frequency: u128,
}

struct Node {
    // letter: Option<char>,
    frequency: Option<u128>,
    is_last: bool,
    children: HashMap<char, Node>,
}

struct Trie {
    root: Node,
}

impl Trie {
    fn new() -> Self {
        Self {
            root: Node {
                // letter: None,
                frequency: None,
                is_last: false,
                children: HashMap::new(),
            },
        }
    }
}

impl Node {
    fn new_branch() -> Self {
        return Self {
            // letter: None,
            children: HashMap::new(),
            frequency: None,
            is_last: false,
        };
    }
}

trait Dictionary {
    fn build(&mut self, words: Vec<WordFreq>);

    fn insert(self: &mut Self, word_freq: WordFreq) -> Result<(), ()>;

    fn delete(&self, word_freq: WordFreq) -> bool;

    fn search(&mut self, word: String) -> Option<u128>;

    fn autocomplete(&mut self, prefix: String) -> Vec<String>;
}

impl Trie {
    fn dfs(&self, node: &Node, word: &String, node_letter: char, res: &mut Vec<String>) {
        if node.is_last {
            res.push(word.clone());
        }
        for child in &node.children {
            let mut w = word.clone();
            w.push(node_letter);
            self.dfs(&child.1, &w, child.0.clone(), res);
        }
    }
}

impl Dictionary for Trie {
    fn build(&mut self, word_freqs: Vec<WordFreq>) {
        word_freqs
            .into_iter()
            .for_each(|w| if let Ok(()) = self.insert(w) {});
    }

    fn insert(&mut self, word_freq: WordFreq) -> Result<(), ()> {
        let mut cur = &mut self.root;
        for ch in word_freq.word.chars() {
            if !cur.children.contains_key(&ch) {
                cur.children.insert(ch, Node::new_branch());
            }
            cur = cur.children.get_mut(&ch).unwrap();
        }
        cur.is_last = true;
        cur.frequency = Some(word_freq.frequency);
        return Ok(());
    }

    fn delete(&self, word_freq: WordFreq) -> bool {
        todo!()
    }

    fn search(&mut self, word: String) -> Option<u128> {
        let mut cur = &mut self.root;
        for ch in word.chars() {
            if !cur.children.contains_key(&ch) {
                println!(" not found");
                return None;
            }
            cur = cur.children.get_mut(&ch).unwrap();
        }
        return cur.frequency;
    }

    fn autocomplete(&mut self, prefix: String) -> Vec<String> {
        let res = Vec::new();
        let mut cur = &self.root;
        for ch in prefix.chars() {
            if !cur.children.contains_key(&ch) {
                return res;
            }
            cur = cur.children.get(&ch).unwrap();
        }
        let res = &mut Vec::new();
        self.dfs(cur, &prefix, prefix.chars().last().unwrap(), res);
        return res[..3].to_owned();
    }
}

fn load_words() -> Vec<WordFreq> {
    let mut words = Vec::new();
    let file = File::open("/home/ryan/rust/trie/sampleData200k.txt").unwrap();
    let lines = BufReader::new(file).lines();
    for line in lines {
        let line = line.unwrap();
        let mut tokens = line.split("  ");
        let word = tokens.next().unwrap().to_string();
        let frequency: u128 = tokens.next().unwrap().parse().unwrap();
        let element = WordFreq { word, frequency };
        words.push(element);
    }
    return words;
}

fn main() -> Result<(), Box<dyn Error>> {
    ui::setup_and_run()
}
