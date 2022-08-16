use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

pub struct WordFreq {
    pub word: String,
    pub frequency: u128,
}

struct Node {
    frequency: Option<u128>,
    is_last: bool,
    children: HashMap<char, Node>,
}

pub struct Trie {
    root: Node,
}

impl Trie {
    pub fn new() -> Self {
        Self {
            root: Node {
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
            children: HashMap::new(),
            frequency: None,
            is_last: false,
        };
    }
}

pub trait Dictionary {
    fn build(&mut self, words: Vec<WordFreq>);

    fn insert(self: &mut Self, word_freq: WordFreq) -> Result<(), ()>;

    fn delete(&self, word_freq: WordFreq) -> bool;

    fn search(&mut self, word: String) -> Option<u128>;

    fn autocomplete(&mut self, prefix: &String) -> Vec<WordFreq>;
}

impl Trie {
    fn dfs(&self, node: &Node, word: &String, node_letter: char, res: &mut Vec<WordFreq>) {
        if node.is_last {
            let mut word = word.to_owned();
            word.push(node_letter);
            res.push(WordFreq {
                word,
                frequency: node.frequency.unwrap(),
            });
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

    fn autocomplete(&mut self, prefix: &String) -> Vec<WordFreq> {
        let res: Vec<WordFreq> = Vec::new();
        let mut cur = &self.root;
        for ch in prefix.chars() {
            if !cur.children.contains_key(&ch) {
                return res;
            }
            cur = cur.children.get(&ch).unwrap();
        }
        let mut res = Vec::new();
        self.dfs(
            cur,
            &prefix[..prefix.len() - 1].to_string(),
            prefix.chars().last().unwrap(),
            &mut res,
        );
        res.sort_unstable_by(|a, b| b.frequency.partial_cmp(&a.frequency).unwrap());
        return res;
    }
}

pub fn load_words() -> Vec<WordFreq> {
    let mut words = Vec::new();
    let file = File::open("/home/ryan/rust/trie/sampleData.txt").unwrap();
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
