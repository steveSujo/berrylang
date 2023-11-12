use std::{
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
        }
    }
}
#[derive(Clone, Debug)]
enum Block {
    Open(char),
    Close(char),
}
#[derive(Clone, Debug)]
enum Expr {
    Left(char),
    Right(char),
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
    let new = tree_eval(tree);
    // write!(writer, "{:#?}\n", new_string).unwrap();
}

fn tree_eval(tree: HashTree<Tokens>) -> HashTree<Tokens> {
    let mut tokens = tree.child_iter(0).unwrap().peekable();
    let mut statement_tree = HashTree::<Tokens>::from(Tokens::Root);
    let mut child: Vec<&Node<Tokens>> = Vec::new();
    // let mut child: Vec<NodeIndex> = Vec::new();

    //TODO probably Match is better
    // optimize this

    while let Some(&token) = tokens.peek() {
        if let Tokens::StateDlimit(x) = token.data {
            let parent = statement_tree.new_node(Tokens::StateDlimit(x), Some(0));
            for &&c in child.iter() {
                // statement_tree.new_node(tree.find_node(c).data.to_owned(), Some(parent));
                statement_tree.insert_node(c, Some(parent));
            }
            child.clear();
        } else if let Tokens::Operator(x) = token.data {
            // let parent = statement_tree.new_node(token.data.to_owned(), Some(0));
            let parent: Node<Tokens> = Node {
                index: 0,
                parent: None,
                childern: vec![],
                data: Tokens::Operator(x),
                prev_sibling: None,
                next_sibling: None,
            };
            if x == '+' {
                statement_tree.new_node(child.last().unwrap().data.to_owned(), Some(parent));
                child.clear();
                tokens.next();
                statement_tree.new_node(tokens.peek().unwrap().data.to_owned(), Some(parent));
            }
            child.push(&parent)
        } else {
            child.push(token);
        }

        tokens.next();
        // println!("{:#?}", token);
    }

    // let mut expr_tree = &mut statement_tree;

    // while let Some(token) = expr_tree.dfs_iter().unwrap().iter().peekable().peek() {
    //     for &token in token.childern.iter() {
    //         if let Tokens::Operator(x) = expr_tree.findmut_node(token).data {
    //             let parent = expr_tree.new_node(Tokens::Operator(x), None);
    //             if x == '+' {
    //                 expr_tree.new_node(child.last().unwrap().data.to_owned(), Some(parent));
    //                 child.clear();
    //                 tokens.next();
    //                 expr_tree.new_node(tokens.peek().unwrap().data.to_owned(), Some(parent));
    //             }
    //             child.push(statement_tree.find_node(parent));
    //         }
    //         // child.push(token);
    //     }
    // }

    println!("{}", statement_tree);
    println!("{:?}", statement_tree);
    println!("{}", &tree);
    statement_tree
    //     // match tree.find_node(token).data {
    //     //     Tokens::StateDlimit(x) => {
    //     //         if let Tokens::Operator(op) = tree.find_node(**tokens.peek().unwrap()).data {}
    //     //     }
    //     //     _ => todo!(),
    //     // }
    // }
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
        let string = String::from("1+2;\n20*30;");
        parse(string);
        // assert_eq!(ve, b"whatever");
    }
}
