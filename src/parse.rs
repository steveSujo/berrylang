use std::{
    error::Error,
    fmt::{self, write},
    fs::File,
    io::Read,
};

use crate::hashtree::{HashTree, Node, NodeIndex};

#[derive(Clone, Debug)]
enum Tokens {
    Root,
    Int(i64),
    StateDlimit(char),
    BlockDlimit(Block),
    Identifier(String),
    Operator(char),
    ExprLeft(Box<Tokens>),
    ExprRight(Box<Tokens>),
}
impl Default for Tokens {
    fn default() -> Self {
        Tokens::StateDlimit(Default::default())
    }
}
impl std::fmt::Display for Tokens {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Tokens::Root => write!(f, "Root node"),
            Tokens::Int(x) => write!(f, "int toke {x}"),
            Tokens::StateDlimit(x) => write!(f, "Dlimt toke {x}"),
            Tokens::BlockDlimit(x) => write!(f, "blockdlimt toke {x}"),
            Tokens::Identifier(x) => write!(f, "ident toke {x}"),
            Tokens::Operator(x) => write!(f, "oprator toke {x}"),
            Tokens::ExprLeft(x) => write!(f, "left exp toke: {x}"),
            Tokens::ExprRight(x) => write!(f, "right exp toke: {x}"),
        }
    }
}
#[derive(Clone, Debug)]
enum Block {
    Open(char),
    Close(char),
}

impl std::fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Block::Open(x) => write!(f, "Open toke {x}"),
            Block::Close(x) => write!(f, "Close toke {x}"),
        }
    }
}
// pub fn parse(mut file: File, mut writer: impl std::io::Write) {
pub fn parse(mut contents: String) {
    let mut tree = HashTree::<Tokens>::from(Tokens::Root);

    // let mut contents = String::new();

    // file.read_to_string(&mut contents).unwrap();

    // write!(writer, "{:#?}", contents).unwrap();

    tree = tokenize(tree, contents);
    // println!("{:#?}", tree);
    let ast = ast_eval(tree);
    let val = interpret(ast).unwrap();
    println!("\nVAl:: {}\n", val);
    // write!(writer, "{:#?}\n", new_string).unwrap();
}

fn interpret(ast: HashTree<Tokens>) -> Result<i64, String> {
    let mut val: i64 = 0;
    let binding = ast.dfs_iter().unwrap();
    let mut iter = binding.iter().peekable();
    while let Some(node) = iter.peek() {
        if let Tokens::Operator(x) = node.data {
            match x {
                '+' => {
                    let children = ast.child_iter(node.index).unwrap();
                    for c in children {
                        if let Tokens::ExprLeft(left) = &c.data {
                            if let Tokens::Int(x) = **left {
                                val += x
                            }
                        }
                        if let Tokens::ExprRight(right) = &c.data {
                            if let Tokens::Int(x) = **right {
                                val += x
                            }
                        }
                    }
                }

                '*' => {
                    let children = ast.child_iter(node.index).unwrap();
                    let mut temp_val: i64 = 0;
                    for c in children {
                        if let Tokens::ExprLeft(left) = &c.data {
                            if let Tokens::Int(l) = **left {
                                temp_val = l;
                            }
                        } else if let Tokens::ExprRight(right) = &c.data {
                            if let Tokens::Int(r) = **right {
                                temp_val *= r;
                            }
                        }
                        // if let Tokens::ExprRight(right) = &c.data {
                        //     if let Tokens::Int(x) = **right {}
                        // }
                    }
                    val = temp_val;
                }
                _ => {
                    iter.next();
                }
            };
        }
        iter.next();
    }
    Ok(val)
}

fn ast_eval(tree: HashTree<Tokens>) -> HashTree<Tokens> {
    let mut tokens = tree.child_iter(0).unwrap().peekable();
    let mut statement_tree = HashTree::<Tokens>::from(Tokens::Root);
    // let mut child: Vec<&Node<Tokens>> = Vec::new();
    let mut child: Vec<NodeIndex> = Vec::new();

    //TODO probably Match is better
    // optimize this

    while let Some(&token) = tokens.peek() {
        if let Tokens::StateDlimit(x) = token.data {
            let parent = statement_tree.new_node(Tokens::StateDlimit(x), Some(0));
            for &c in child.iter() {
                statement_tree.make_child(c, parent);
            }
            child.clear();
        } else if let Tokens::Operator(x) = token.data {
            let left = statement_tree.remove_node(*child.last().unwrap()).unwrap();
            // println!("__token {:?}", left);
            let parent = statement_tree.new_node(token.data.to_owned(), Some(0));
            if left.childern.is_empty() {
                if x == '+' {
                    statement_tree.new_node(Tokens::ExprLeft(Box::new(left.data)), Some(parent));
                    child.clear();
                    tokens.next();
                    statement_tree.new_node(
                        Tokens::ExprRight(Box::new(tokens.peek().unwrap().data.to_owned())),
                        Some(parent),
                    );
                } else if x == '*' {
                    statement_tree.new_node(Tokens::ExprLeft(Box::new(left.data)), Some(parent));
                    child.clear();
                    tokens.next();
                    statement_tree.new_node(
                        Tokens::ExprRight(Box::new(tokens.peek().unwrap().data.to_owned())),
                        Some(parent),
                    );
                }
            } else {
                if x == '*' {
                    statement_tree.insert_node(left, Some(parent));
                    child.clear();
                    tokens.next();
                    statement_tree.new_node(
                        Tokens::ExprRight(Box::new(tokens.peek().unwrap().data.to_owned())),
                        Some(parent),
                    );
                }
            }
            child.push(parent)
        } else {
            child.push(statement_tree.new_node(token.data.to_owned(), Some(0)));
        }

        tokens.next();
        // println!("{:#?}", token);
    }

    // debug
    println!(
        "state tree: {}\nlen: {}",
        statement_tree,
        statement_tree.len()
    );
    println!("state tree DEBUG: {:?}\n", statement_tree);
    // println!("token tree: {}\nlen: {}", &tree, tree.len());
    statement_tree
}
//FIXME fun prog this tokenizer
fn tokenize(mut tree: HashTree<Tokens>, contents: String) -> HashTree<Tokens> {
    let mut text = contents.chars().peekable();

    while let Some(&x) = text.peek() {
        match x {
            //tokenize number
            '0'..='9' => {
                let mut num = String::from(x);
                text.next();
                while let Some(&n) = text.peek() {
                    if n.is_digit(10) {
                        num.push(n);
                        text.next();
                    } else {
                        break;
                    }
                }
                if let Ok(int) = num.parse::<i64>() {
                    tree.new_node(Tokens::Int(int), Some(0));
                } else {
                    panic!("int parser error");
                }
            }
            //tokenize aritmetirc operator
            '*' | '/' | '+' | '-' => {
                tree.new_node(Tokens::Operator(x), Some(0));
                text.next();
            }
            // Blocks
            '(' | '<' | '{' => {
                tree.new_node(Tokens::BlockDlimit(Block::Open(x)), Some(0));
                text.next();
            }
            ')' | '>' | '}' => {
                tree.new_node(Tokens::BlockDlimit(Block::Close(x)), Some(0));
                text.next();
            }

            // Dlmit
            ';' => {
                tree.new_node(Tokens::StateDlimit(x), Some(0));
                text.next();
            }
            _ => {
                text.next();
            }
        }
    }

    tree
}

///////////////////////////////////////////////////////////////////////////////
//                                 Unit Tests                                //
///////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod parser {
    use super::*;

    // #[ignore = "not now"]
    #[test]
    fn parse_fn() {
        let mut string = String::from("111+20;");
        parse(string);
        string = String::from("20*2;");
        parse(string);
        string = String::from("1+2*2;");
        parse(string);
        // assert_eq!(ve, b"whatever");
    }
}
