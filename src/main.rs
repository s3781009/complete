use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};
struct WordFreq {
    word: String,
    frequency: u128,
}

struct Node {
    letter: Option<char>,
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
                letter: None,
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
            letter: None,
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

    fn autocomplete(&self, prefix: String) -> Vec<WordFreq>;
}
impl Dictionary for Trie {
    fn build(&mut self, word_freqs: Vec<WordFreq>) {
        println!("building");
        word_freqs
            .into_iter()
            .for_each(|w| if let Ok(()) = self.insert(w) {});
        println!("building done");
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
        println!("{}", cur.frequency.unwrap());
        return Ok(());
    }

    fn delete(&self, word_freq: WordFreq) -> bool {
        todo!()
    }

    fn search(&mut self, word: String) -> Option<u128> {
        // cur = self.root
        // for ch in word:
        //     if ch not in cur.children:
        //         return 0
        //     cur = cur.children[ch]

        // return cur.frequency if cur.frequency else 0
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

    fn autocomplete(&self, prefix: String) -> Vec<WordFreq> {
        todo!()
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

fn main() {
    let words = load_words();
    // for word in words {
    //     println!("{}", word.frequency);
    // }
    let mut trie = Trie::new();
    trie.build(words);
    trie.search(String::from("wookie"));
    if let Some(freq) = trie.search(String::from("wookie")) {
        print!("{}", freq);
    }
}
