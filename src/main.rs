use std::fs::File;
use std::io::Read;

#[derive(Debug)]
pub struct Node {
    pub children: Vec<Node>,
    pub node_type: NodeType,
}

#[derive(Debug)]
pub enum NodeType {
    Element(ElementData),
    Text(String),
}

#[derive(Debug)]
pub struct ElementData {
    pub tag_name: String,
}

pub struct Parser {
    pos: usize,
    input: String,
}

impl Parser {
    pub fn build_tree(&mut self) -> Vec<Node> {
        let mut children = vec![];

        loop {
            self.consume_whitespaces();
            if self.file_not_finished() || self.starts_with("</") { // detecting closing tag
                break;
            }

            children.push(self.parse_node());
        }

        children
    }

    fn file_not_finished(&self) -> bool {
        self.pos >= self.input.len()
    }

    fn starts_with(&self, s: &str) -> bool {
        self.input[self.pos ..].starts_with(s)
    }

    fn parse_node(&mut self) -> Node {
        match self.get_next_char().unwrap() {
            '<' => self.parse_tag(),
            _ => self.parse_value()
        }
    }

    fn parse_tag(&mut self) -> Node {
        // opening tag
        assert_eq!(self.consume_next_char().unwrap(), '<');
        let tag_name = self.read_until('>');
        let full_tag = '<'.to_string() + &tag_name + &'>'.to_string();
        assert_eq!(self.consume_next_char().unwrap(), '>');

        // parsing children
        let children = self.build_tree();
        println!("{}'s children: {:?}", full_tag, children);

        // closing tag
        assert_eq!(self.consume_next_char().unwrap(), '<');
        assert_eq!(self.consume_next_char().unwrap(), '/');
        assert_eq!(self.read_until('>'), tag_name);
        assert_eq!(self.consume_next_char().unwrap(), '>');

        Node {
            children: children,
            node_type: NodeType::Element(ElementData { tag_name: full_tag }),
        }
    }

    fn parse_value(&mut self) -> Node {
        let value = self.read_until('<');

        Node {
            children: vec![],
            node_type: NodeType::Text(value.trim().to_string()),
        }
    }

    fn get_next_char(&mut self) -> Option<char> {
        self.input[self.pos..].chars().next()
    }

    // same as get_next_char but moves pos pointer
    fn consume_next_char(&mut self) -> Option<char> {
        match self.input[self.pos..].chars().next() {
            Some(c) => {
                self.pos = self.pos + 1;
                Some(c)
            }
            None => None,
        }
    }

    fn consume_whitespaces(&mut self) -> () {
        loop {
            match self.get_next_char() {
                Some(c) => {
                    if !c.is_whitespace() {
                        break;
                    }
                    self.consume_next_char();
                },
                None => break
            }
        }
    }

    fn read_until(&mut self, character: char) -> String {
        let mut out = String::from("");
        loop {
            match self.get_next_char() {
                Some(c) => {
                    if c == character {
                        return out;
                    }
                    self.consume_next_char();
                    out.push(c);
                }
                None => {
                    return out;
                }
            }
        }
    }
}

fn read_source(filename: String) -> String {
    let mut str = String::new();
    File::open(filename)
        .unwrap()
        .read_to_string(&mut str)
        .unwrap();
    str
}

fn main() {
    let html = read_source("examples/test2.html".to_string());
    let mut parser = Parser {
        pos: 0,
        input: html,
    };

    let tree = parser.build_tree();
    println!("{:?}", tree)
}
