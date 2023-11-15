use std::{
    collections::{
        hash_map::{self, DefaultHasher},
        HashMap,
    },
    fmt,
    hash::Hash,
    hash::Hasher,
    ops::Index,
};

#[derive(Default)]
pub struct HashTree<T> {
    pub nodes: HashMap<NodeIndex, Node<T>>,
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

pub type NodeIndex = u64;

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
    T: std::hash::Hash,
    T: std::fmt::Display,
{
    pub fn hasher_boi(data: &impl Hash) -> u64 {
        let mut hasher = DefaultHasher::new();
        data.hash(&mut hasher);
        hasher.finish()
    }
    pub fn new() -> Self {
        Default::default()
    }

    pub fn from(data: T) -> (HashTree<T>, NodeIndex) {
        let mut root: Node<T> = Default::default();
        let index = Self::hasher_boi(&data);
        let mut nodes = HashMap::new();
        root.index = index;
        root.data = data;
        nodes.insert(index, root);
        (HashTree { nodes }, index)
    }
    pub fn new_node(&mut self, data: T, parent: Option<NodeIndex>) -> NodeIndex {
        let index = Self::hasher_boi(&data);
        let mut prev_sibling: Option<NodeIndex> = None;

        if let Some(parent) = parent {
            if !self.get(parent).unwrap().childern.is_empty() {
                prev_sibling = self.get(parent).unwrap().childern.last().copied();
                self.get_mut(prev_sibling.unwrap()).unwrap().next_sibling = Some(index);
            };
            self.get_mut(parent).unwrap().childern.push(index);
        }

        self.nodes.insert(
            index,
            Node {
                parent,
                prev_sibling,
                next_sibling: None,
                childern: Vec::new(),
                data,
                index,
            },
        );
        index
    }

    pub fn get(&self, index: NodeIndex) -> Option<&Node<T>> {
        // self.nodes.iter().find(|n| n.index == index)
        self.nodes.get(&index)
    }

    //TODO panick cases
    pub fn get_mut(&mut self, index: NodeIndex) -> Option<&mut Node<T>> {
        // self.nodes.iter_mut().find(|n| n.index == index)
        self.nodes.get_mut(&index)
    }

    pub fn remove_node(&mut self, index: NodeIndex) -> Option<Node<T>> {
        // update parent
        let parent = self.get(index).unwrap().parent.unwrap();
        self.get_mut(parent).unwrap().childern = self
            .get(parent)
            .unwrap()
            .childern
            .iter()
            .filter_map(|&i| match i != index {
                true => Some(i),
                false => None,
            })
            .collect();
        // update sib
        match (
            self.get(index).unwrap().next_sibling,
            self.get(index).unwrap().prev_sibling,
        ) {
            (None, None) => {}
            (None, Some(prev)) => {
                self.get_mut(prev).unwrap().next_sibling = None;
            }
            (Some(next), None) => {
                self.get_mut(next).unwrap().prev_sibling = None;
            }
            (Some(next), Some(prev)) => {
                self.get_mut(prev).unwrap().next_sibling = Some(next);
                self.get_mut(next).unwrap().prev_sibling = Some(prev);
            }
        }

        self.nodes.remove(&index)
    }

    pub fn child_iter(&self, index: NodeIndex) -> Option<impl Iterator<Item = &Node<T>>> {
        let children = self
            .get(index)
            .unwrap()
            .childern
            .iter()
            .map(|&i| self.get(i).unwrap());

        Some(children)
    }
    pub fn childmut_iter(&mut self, index: NodeIndex) -> Option<impl Iterator<Item = &Node<T>>> {
        let children = self
            .get(index)
            .unwrap()
            .childern
            .iter()
            .map(|&i| self.get(i).unwrap());

        Some(children)
    }

    pub fn siblings_iter(&self, index: NodeIndex) -> Option<impl Iterator<Item = &Node<T>>> {
        let children = self
            .get(self.get(index).unwrap().parent.unwrap())
            .unwrap()
            .childern
            .iter()
            .filter(move |&i| i != &index)
            .map(|&i| self.get(i).unwrap());

        Some(children)
    }

    pub fn siblingsmut_iter(&mut self, index: NodeIndex) -> Option<impl Iterator<Item = &Node<T>>> {
        let children = self
            .get(self.get(index).unwrap().parent.unwrap())
            .unwrap()
            .childern
            .iter()
            .filter(move |&i| i != &index)
            .map(|&i| self.get(i).unwrap());

        Some(children)
    }

    pub fn make_child(&mut self, index: NodeIndex, parent: NodeIndex) {
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
            self.get_mut(parent).unwrap().childern.push(node.index);

            node.prev_sibling = None;
            node.next_sibling = None;
            if let Some(&last_index) = self.get(parent).unwrap().childern.last() {
                self.get_mut(last_index).unwrap().next_sibling = Some(node.index);
                node.prev_sibling = Some(last_index);
            };
        }
        self.nodes.insert(node.index, node);
    }
    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    // pub fn traverse(&self, path: &str) -> Option<(dyn Iterator<Item = Node<T>> + 'static)> {}
    // TODO depth var for this function to optimize scope
    // pub fn dfs_iter(&self) -> Option<Vec<&Node<T>>> {
    //     let mut depth: Vec<NodeIndex> = Vec::<NodeIndex>::new();
    //     depth.push(self.nodes[0].index);
    //     let children = &self.nodes[0].childern;
    //     let mut breath = children.to_vec();

    //     while !breath.is_empty() {
    //         depth.extend(&breath);
    //         let temp: Vec<NodeIndex> = breath.to_vec();
    //         breath.clear();
    //         for &i in temp.iter() {
    //             breath.extend(&self.get(i).unwrap().childern)
    //         }
    //     }

    //     let list = depth
    //         .iter()
    //         .map(|&f| self.get(f).unwrap())
    //         .collect::<Vec<&Node<T>>>();

    //     Some(list)
    // }
    // pub fn nodes(&self) -> Option<&Vec<Node<T>>> {
    //     Some(&self.nodes)
    // }
    pub fn clear(&mut self) {
        self.nodes.clear();
    }

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
                string_vec.extend(self.fmt_tree(depth, &self.get(child).unwrap()));
            }
            return string_vec;
        }
    }
}

// impl<T: std::fmt::Display> HashTree<T> {
//     //TODO scop of this fn
// }

impl<T> fmt::Display for HashTree<T>
where
    T: std::default::Default,
    T: std::hash::Hash,
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();
        let depth = 0;
        s.extend(self.fmt_tree(depth, &self.nodes.iter().next().unwrap().1));
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
        let (mut tree, hash) = HashTree::<isize>::from(TESTSET[0]);

        assert_eq!(tree.nodes[&hash].data, TESTSET[0]);
    }

    //FIXME make the below test case easyer for the eyes
    #[test]
    fn insert_child() {
        let (mut tree, root_hash) = HashTree::<isize>::from(TESTSET[0]);

        tree.new_node(TESTSET[1], Some(root_hash));

        assert_eq!(
            // tree.nodes[tree.nodes[0].last_child.unwrap()].data,
            // tree.find_node(tree.child_iter(0).unwrap()[0]).data,
            tree.child_iter(root_hash).unwrap().next().unwrap().data,
            TESTSET[1]
        );

        // assert_eq!(tree.find_node(0).first_child, tree.find_node(0).last_child);

        tree.new_node(TESTSET[2], Some(root_hash));

        assert_eq!(
            tree.child_iter(root_hash).unwrap().last().unwrap().data,
            TESTSET[2]
        );

        assert_eq!(
            tree.get(
                tree.get(HashTree::<isize>::hasher_boi(&TESTSET[1]))
                    .unwrap()
                    .next_sibling
                    .unwrap()
            )
            .unwrap()
            .data,
            tree.child_iter(root_hash).unwrap().last().unwrap().data
        );
    }

    //FIXME make the below test case easyer for the eyes
    #[test]
    fn children_iter() {
        let (mut tree, hash) = HashTree::<isize>::from(TESTSET[0]);

        for i in 1..TESTSET.len() {
            tree.new_node(TESTSET[i], Some(HashTree::<isize>::hasher_boi(&TESTSET[0])));
        }
        let mut children = tree
            .child_iter(HashTree::<isize>::hasher_boi(&TESTSET[0]))
            .unwrap();

        assert_eq!(children.next().unwrap().data, TESTSET[1]);
        assert_eq!(children.next().unwrap().data, TESTSET[2]);
        assert_eq!(children.next().unwrap().data, TESTSET[3]);
        assert_eq!(children.next().unwrap().data, TESTSET[4]);
        // TESTSET.iter().for_each(|i| tree.insert_child(0, *i));

        // let siblings = tree.find_first_child_of(0).iter();
    }

    #[test]
    fn siblings_iter() {
        let (mut tree, hash) = HashTree::<isize>::from(TESTSET[0]);

        for i in 1..TESTSET.len() {
            tree.new_node(TESTSET[i], Some(HashTree::<isize>::hasher_boi(&TESTSET[0])));
        }
        let mut sib = tree
            .siblings_iter(HashTree::<isize>::hasher_boi(&TESTSET[1]))
            .unwrap();

        // assert_eq!(sib.next().unwrap().data, TESTSET[1]);
        assert_eq!(sib.next().unwrap().data, TESTSET[2]);
        assert_eq!(sib.next().unwrap().data, TESTSET[3]);
        assert_eq!(sib.next().unwrap().data, TESTSET[4]);
    }
    #[ignore = "hash update"]
    #[test]
    fn dsf_iter() {
        let (mut tree, hash) = HashTree::<isize>::from(TESTSET[0]);
        let mut flag = 0;
        for i in 1..TESTSET.len() {
            if i <= 3 && i != 1 {
                tree.new_node(TESTSET[i], Some(HashTree::<isize>::hasher_boi(&TESTSET[1])));
                flag = i;
            } else {
                tree.new_node(
                    TESTSET[i],
                    Some(HashTree::<isize>::hasher_boi(&TESTSET[flag])),
                );
            }
        }

        //BFS
        // let list = tree.dfs_iter().unwrap();
        // let mut iter = list.iter();

        // assert_eq!(iter.next().unwrap().data, TESTSET[0]);
        // assert_eq!(iter.next().unwrap().data, TESTSET[1]);
        // assert_eq!(iter.next().unwrap().data, TESTSET[2]);
        // assert_eq!(iter.next().unwrap().data, TESTSET[3]);
        // assert_eq!(iter.next().unwrap().data, TESTSET[4]);
        // assert_eq!(iter.next().unwrap().data, TESTSET[5]);
    }
    #[test]
    fn fmt_test() {
        let (mut tree, hash) = HashTree::<isize>::from(TESTSET[0]);
        for i in 1..TESTSET.len() {
            tree.new_node(
                TESTSET[i],
                Some(HashTree::<isize>::hasher_boi(&TESTSET[i - 1])),
            );
        }

        // println!("{}", tree);
        // println!("{:?}\n", tree);
    }
    #[test]
    fn hash_test() {
        let hash1 = HashTree::<isize>::hasher_boi(&TESTSET[0]);
        let hash2 = HashTree::<isize>::hasher_boi(&TESTSET[0]);
        println!("HASH1: {}", hash1);
        println!("HASH2: {}", hash2);
    }
}
