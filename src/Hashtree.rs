#[derive(Default)]
struct HashTree<T> {
    nodes: Vec<Node<T>>,
}

#[derive(Default)]
struct Node<T> {
    parent: Option<NodeIndex>,

    prev_sibling: Option<NodeIndex>,
    next_sibling: Option<NodeIndex>,

    first_child: Option<NodeIndex>,
    last_child: Option<NodeIndex>,

    data: T,
}

type NodeIndex = usize;

// TODO currently not a hash tree
impl<T: std::default::Default> HashTree<T> {
    fn new() -> Self {
        Default::default()
    }

    fn from(data: T) -> HashTree<T> {
        let mut root: Node<T> = Default::default();
        root.data = data;
        HashTree { nodes: vec![root] }
    }
    fn new_node(&mut self, data: T) -> NodeIndex {
        let index = self.nodes.len();

        self.nodes.push(Node {
            parent: None,
            prev_sibling: None,
            next_sibling: None,
            first_child: None,
            last_child: None,
            data,
        });

        index
    }

    fn find_node(&self, index: usize) -> &Node<T> {
        &self.nodes[index]
    }

    fn find_last_child_of(&self, index: usize) -> &Node<T> {
        let last_child_index = self.find_node(index).last_child.unwrap();

        &self.nodes[last_child_index]
    }
    fn find_first_child_of(&self, index: usize) -> &Node<T> {
        let first_child_index = self.find_node(index).first_child.unwrap();

        &self.nodes[first_child_index]
    }

    fn findmut_node(&mut self, index: usize) -> &mut Node<T> {
        &mut self.nodes[index]
    }

    fn findmut_last_child_of(&mut self, index: usize) -> &Node<T> {
        let last_child_index = self.find_node(index).last_child.unwrap();

        &self.nodes[last_child_index]
    }

    fn findmut_first_child_of(&mut self, index: usize) -> &mut Node<T> {
        let first_child_index = self.find_node(index).first_child.unwrap();

        &mut self.nodes[first_child_index]
    }

    fn remove_node(&mut self, index: usize) {
        self.nodes.remove(index);
    }

    fn insert_child(&mut self, parent_index: usize, data: T) {
        let child_index = self.new_node(data);

        self.nodes[child_index].parent = Some(parent_index);

        match self.nodes[parent_index].last_child {
            Some(index) => {
                self.nodes[parent_index].last_child = Some(child_index);
                self.nodes[index].next_sibling = Some(child_index);
                self.nodes[child_index].prev_sibling = Some(index);
            }
            None => {
                self.nodes[parent_index].last_child = Some(child_index);
                self.nodes[parent_index].first_child = Some(child_index);
            }
        };
    }

    fn child_iter(&self, index: usize) -> Option<impl Iterator<Item = &&Node<T>>> {
        let mut child_index = self.find_node(index).first_child;

        let mut iter: Vec<&Node<T>> = Vec::new();

        if child_index == None {
            return None;
        } else {
            //FIXME sibling iter
            iter.push(self.find_node(child_index.unwrap()));

            child_index = self.find_node(child_index.unwrap()).next_sibling;

            loop {
                if child_index != None {
                    iter.push(self.find_node(child_index.unwrap()));
                    child_index = self.find_node(child_index.unwrap()).next_sibling;
                } else {
                    break;
                }
            }
        };

        // Some(iter.iter()).to_owned()
    }
}

///////////////////////////////////////////////////////////////////////////////
//                                 Uit tests                                 //
///////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod hashtree {
    use std::env::consts::DLL_EXTENSION;

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

        tree.insert_child(0, TESTSET[1]);

        assert_eq!(
            // tree.nodes[tree.nodes[0].last_child.unwrap()].data,
            tree.find_last_child_of(0).data,
            TESTSET[1]
        );

        assert_eq!(tree.find_node(0).first_child, tree.find_node(0).last_child);

        tree.insert_child(0, TESTSET[2]);

        assert_eq!(tree.find_last_child_of(0).data, TESTSET[2]);

        assert_eq!(tree.find_node(1).next_sibling, tree.find_node(0).last_child);
    }

    #[ignore = "Not impl"]
    #[test]
    fn iter_sblings() {
        let mut tree = HashTree::<isize>::from(TESTSET[0]);

        TESTSET.iter().for_each(|i| tree.insert_child(0, *i));

        let siblings = tree.find_first_child_of(0).iter();
    }
    #[ignore = "Not impl"]
    #[test]
    fn traverse_tree() {}
}
