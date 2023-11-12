use std::{fmt, ops::Index};

#[derive(Default)]
pub struct HashTree<T> {
    pub nodes: Vec<Node<T>>,
}

#[derive(Default, PartialEq, Debug)]
pub struct Node<T> {
    pub index: NodeIndex,
    pub parent: Option<NodeIndex>,

    pub prev_sibling: Option<NodeIndex>,
    pub next_sibling: Option<NodeIndex>,

    pub childern: Vec<NodeIndex>,
    pub data: T,
}

pub type NodeIndex = usize;

// impl<T> Iterator for Node<T> {
//     type Item = NodeIndex;
//     fn next(&mut self) -> Option<Self::Item> {
//         self.next_sibling
//     }
// }

// impl<T> Node<T> {
// }

// TODO currently not a hash tree
impl<T> HashTree<T>
where
    T: std::default::Default,
{
    pub fn new() -> Self {
        Default::default()
    }

    pub fn from(data: T) -> HashTree<T> {
        let mut root: Node<T> = Default::default();
        root.data = data;
        HashTree { nodes: vec![root] }
    }
    pub fn new_node(&mut self, data: T, parent: Option<NodeIndex>) -> NodeIndex {
        let index = self.nodes.len();
        let mut prev_sibling: Option<NodeIndex> = None;

        if let Some(parent) = parent {
            if !self.find_node(parent).unwrap().childern.is_empty() {
                prev_sibling = self.find_node(parent).unwrap().childern.last().copied();
                self.findmut_node(prev_sibling.unwrap())
                    .unwrap()
                    .next_sibling = Some(index);
            };
            self.findmut_node(parent).unwrap().childern.push(index);
        }

        self.nodes.push(Node {
            parent,
            prev_sibling,
            next_sibling: None,
            childern: Vec::new(),
            data,
            index,
        });
        index
    }

    pub fn find_node(&self, index: usize) -> Option<&Node<T>> {
        self.nodes.iter().find(|n| n.index == index)
    }

    //TODO panick cases
    pub fn findmut_node(&mut self, index: usize) -> Option<&mut Node<T>> {
        self.nodes.iter_mut().find(|n| n.index == index)
    }

    pub fn remove_node(&mut self, index: usize) -> Option<Node<T>> {
        // update parent
        let parent = self.nodes[index].parent.unwrap();
        self.nodes[parent].childern = self.nodes[parent]
            .childern
            .iter()
            .filter_map(|&i| match i != index {
                true => Some(i),
                false => None,
            })
            .collect();
        // update sib
        match (
            self.nodes[index].next_sibling,
            self.nodes[index].prev_sibling,
        ) {
            (None, None) => {}
            (None, Some(prev)) => {
                self.nodes[prev].next_sibling = None;
            }
            (Some(next), None) => {
                self.nodes[next].prev_sibling = None;
            }
            (Some(next), Some(prev)) => {
                self.nodes[prev].next_sibling = Some(next);
                self.nodes[next].prev_sibling = Some(prev);
            }
        }

        Some(self.nodes.remove(index))
    }

    pub fn child_iter(&self, index: usize) -> Option<impl Iterator<Item = &Node<T>>> {
        let children = self.nodes[index]
            .childern
            .iter()
            .map(|&i| self.find_node(i).unwrap());

        Some(children)
    }
    pub fn childmut_iter(&mut self, index: usize) -> Option<impl Iterator<Item = &Node<T>>> {
        let children = self.nodes[index]
            .childern
            .iter()
            .map(|&i| self.find_node(i).unwrap());

        Some(children)
    }

    pub fn siblings_iter(&self, index: usize) -> Option<impl Iterator<Item = &Node<T>>> {
        let children = self.nodes[self.nodes[index].parent.unwrap()]
            .childern
            .iter()
            .filter(move |&i| i != &index)
            .map(|&i| self.find_node(i).unwrap());

        Some(children)
    }

    pub fn siblingsmut_iter(&mut self, index: usize) -> Option<impl Iterator<Item = &Node<T>>> {
        let children = self.nodes[self.nodes[index].parent.unwrap()]
            .childern
            .iter()
            .filter(move |&i| i != &index)
            .map(|&i| self.find_node(i).unwrap());

        Some(children)
    }

    pub(crate) fn make_child(&mut self, index: usize, parent: usize) {
        // let mut tree = self;

        let node = self.remove_node(index).unwrap();
        self.new_node(node.data, Some(parent));
        // self.nodes[index].parent = Some(parent);
        // self.findmut_node(index).unwrap().parent = Some(parent);

        // self.nodes[index].prev_sibling = None;
        // self.nodes[index].next_sibling = None;

        // self.nodes[parent].childern.push(index);

        // let last = self.nodes[parent].childern.last();

        // if let Some(&last_index) = last {
        //     self.nodes[last_index].next_sibling = Some(index);
        //     self.nodes[index].prev_sibling = Some(last_index);
        // };
    }
    pub fn insert_node(&mut self, mut node: Node<T>, parent: Option<NodeIndex>) {
        if let Some(old_parent) = node.parent {
            self.remove_node(node.index);
        }
        if let Some(parent) = parent {
            node.parent = Some(parent);
            self.nodes[parent].childern.push(node.index);

            self.nodes[node.index].prev_sibling = None;
            self.nodes[node.index].next_sibling = None;
            if let Some(&last_index) = self.nodes[parent].childern.last() {
                self.nodes[last_index].next_sibling = Some(node.index);
                self.nodes[node.index].prev_sibling = Some(last_index);
            };
        }
        self.nodes.push(node);
    }
    pub(crate) fn len(&self) -> usize {
        self.nodes.len()
    }

    // pub fn traverse(&self, path: &str) -> Option<(dyn Iterator<Item = Node<T>> + 'static)> {}
    // TODO depth var for this function to optimize scope
    pub fn dfs_iter(&self) -> Option<Vec<&Node<T>>> {
        let mut depth: Vec<NodeIndex> = Vec::<NodeIndex>::new();
        depth.push(0);
        let children = &self.nodes[0].childern;
        let mut breath = children.to_vec();

        while !breath.is_empty() {
            depth.extend(&breath);
            let temp: Vec<NodeIndex> = breath.to_vec();
            breath.clear();
            for &i in temp.iter() {
                breath.extend(&self.nodes[i].childern)
            }
        }

        let list = depth
            .iter()
            .map(|&f| self.find_node(f).unwrap())
            .collect::<Vec<&Node<T>>>();

        Some(list)
    }
    pub fn nodes(&self) -> Option<&Vec<Node<T>>> {
        Some(&self.nodes)
    }
    pub fn clear(&mut self) {
        self.nodes.clear();
    }
    pub fn apend(&self, subtree: HashTree<T>, parent: usize) {}
}

impl<T: std::fmt::Display> HashTree<T> {
    //TODO scop of this fn
    fn fmt_tree(&self, mut depth: i32, n: &Node<T>) -> Vec<String> {
        depth += 1;
        if n.childern.len() == 0 {
            let len: String = (1..depth).map(|_| " ").collect();
            return vec![format!("\n{}|--{}", len, n.data.to_string())];
            // return n
            //     .childern
            //     .iter()
            //     .map(|&x| format!("\n{}|---{}", len, self.nodes[x].data))
            //     .collect::<Vec<_>>();
        } else {
            let len: String = (1..depth).map(|_| " ").collect();
            let mut string_vec: Vec<String> = vec![format!("\n{}∟ {}", len, n.data.to_string())];
            for &child in n.childern.iter() {
                string_vec.extend(self.fmt_tree(depth, &self.nodes[child]));
            }
            return string_vec;
        }
    }
}

impl<T: std::fmt::Display> fmt::Display for HashTree<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();
        let depth = 0;
        s.extend(self.fmt_tree(depth, &self.nodes[0]));
        write!(f, "{}", s).unwrap();
        Ok(())
    }
}

// root
//  |- child 1
//  |- child 2
//      |- subchild 1
impl<T: std::fmt::Debug> fmt::Debug for HashTree<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.nodes).unwrap();
        Ok(())
    }
}
///////////////////////////////////////////////////////////////////////////////
//                                 Uit tests                                 //
///////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod hashtree {

    use super::*;

    const TESTSET: [isize; 6] = [123, 23, 5, 34, 26, 89];

    #[test]
    fn rootnode() {
        let tree = HashTree::<isize>::from(TESTSET[0]);

        assert_eq!(tree.nodes[0].data, TESTSET[0]);
    }

    //FIXME make the below test case easyer for the eyes
    #[test]
    fn insert_child() {
        let mut tree = HashTree::<isize>::from(TESTSET[0]);

        tree.new_node(TESTSET[1], Some(0));

        assert_eq!(
            // tree.nodes[tree.nodes[0].last_child.unwrap()].data,
            // tree.find_node(tree.child_iter(0).unwrap()[0]).data,
            tree.child_iter(0).unwrap().next().unwrap().data,
            TESTSET[1]
        );

        // assert_eq!(tree.find_node(0).first_child, tree.find_node(0).last_child);

        tree.new_node(TESTSET[2], Some(0));

        assert_eq!(tree.child_iter(0).unwrap().last().unwrap().data, TESTSET[2]);

        assert_eq!(
            tree.find_node(tree.find_node(1).unwrap().next_sibling.unwrap())
                .unwrap()
                .data,
            tree.child_iter(0).unwrap().last().unwrap().data
        );
    }

    //FIXME make the below test case easyer for the eyes
    #[test]
    fn children_iter() {
        let mut tree = HashTree::<isize>::from(TESTSET[0]);

        for i in 1..TESTSET.len() {
            tree.new_node(TESTSET[i], Some(0));
        }
        let mut children = tree.child_iter(0).unwrap();

        assert_eq!(children.next().unwrap().data, TESTSET[1]);
        assert_eq!(children.next().unwrap().data, TESTSET[2]);
        assert_eq!(children.next().unwrap().data, TESTSET[3]);
        assert_eq!(children.next().unwrap().data, TESTSET[4]);
        // TESTSET.iter().for_each(|i| tree.insert_child(0, *i));

        // let siblings = tree.find_first_child_of(0).iter();
    }

    #[test]
    fn siblings_iter() {
        let mut tree = HashTree::<isize>::from(TESTSET[0]);

        for i in 1..TESTSET.len() {
            tree.new_node(TESTSET[i], Some(0));
        }
        let mut sib = tree.siblings_iter(1).unwrap();

        // assert_eq!(sib.next().unwrap().data, TESTSET[1]);
        assert_eq!(sib.next().unwrap().data, TESTSET[2]);
        assert_eq!(sib.next().unwrap().data, TESTSET[3]);
        assert_eq!(sib.next().unwrap().data, TESTSET[4]);
    }
    #[test]
    fn dsf_iter() {
        let mut tree = HashTree::<isize>::from(TESTSET[0]);
        let mut flag = 0;
        for i in 1..TESTSET.len() {
            if i <= 3 && i != 1 {
                tree.new_node(TESTSET[i], Some(1));
                flag = i;
            } else {
                tree.new_node(TESTSET[i], Some(flag));
            }
        }

        //BFS
        let list = tree.dfs_iter().unwrap();
        let mut iter = list.iter();

        assert_eq!(iter.next().unwrap().data, TESTSET[0]);
        assert_eq!(iter.next().unwrap().data, TESTSET[1]);
        assert_eq!(iter.next().unwrap().data, TESTSET[2]);
        assert_eq!(iter.next().unwrap().data, TESTSET[3]);
        assert_eq!(iter.next().unwrap().data, TESTSET[4]);
        assert_eq!(iter.next().unwrap().data, TESTSET[5]);
    }
    #[test]
    fn fmt_test() {
        let mut tree = HashTree::<isize>::from(TESTSET[0]);
        for i in 1..TESTSET.len() {
            tree.new_node(TESTSET[i], Some(i - 1));
        }

        // println!("{}", tree);
        // println!("{:?}\n", tree);
    }
}
