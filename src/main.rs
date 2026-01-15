use std::collections::HashMap;
#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct Node {
    children: HashMap<char, Node>,
    is_word: bool,
}

impl Node {
    fn new() -> Self {
        Node {
            children: HashMap::new(),
            is_word: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct WordDictionary {
    root: Node,
}

impl WordDictionary {
    fn new() -> Self {
        Self::default()
    }

    fn insert(&mut self, word: &str) {
        let mut current = &mut self.root;
        for ch in word.chars() {
            current = current.children.entry(ch).or_insert_with(Node::new);
        }
        if !current.is_word {
            current.is_word = true;
        }
    }

    fn search(&self, word: &str) -> bool {
        let mut current = &self.root;
        for w in word.chars() {
            if current.children.get(&w).is_some() {
                current = current.children.get(&w).unwrap();
            } else {
                return false;
            }
        }
        current.is_word
    }
}
fn main() {
    let words = vec![
        "hello",
        "world",
        "from",
        "rust",
        "language",
        "example",
        "code",
        "test",
        "functionality",
        "development",
        "programming",
        "system",
        "performance",
        "safety",
        "concurrency",
        "memory",
        "management",
        "ownership",
        "borrowing",
        "lifetime",
        "traits",
        "generics",
        "macros",
        "crates",
        "modules",
        "packages",
        "ecosystem",
        "community",
        "documentation",
        "tooling",
        "compiler",
        "interoperability",
        "efficiency",
        "scalability",
        "robustness",
        "flexibility",
        "extensibility",
        "maintainability",
        "readability",
        "simplicity",
        "complexity",
        "abstraction",
        "encapsulation",
        "polymorphism",
        "inheritance",
        "composition",
        "design patterns"
    ]
        .into_iter()
        .map(|s| String::from(s))
        .collect::<Vec<String>>();

    let mut d = WordDictionary::new();
    for i in 0..words.len() {
        d.insert(&words[i]);
    }

    println!("Searching 'concurrency' results:{}", d.search("concurrency"));
    println!("Searching 'this' results:{}", d.search("this"));
    println!("Searching 'language' results:{}", d.search("language"));
}
