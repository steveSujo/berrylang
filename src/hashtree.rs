#[derive(Default)]
pub struct HashTree<T> {
    nodes: Vec<Node<T>>,
}

#[derive(Default)]
struct Node<T> {
    parent: Option<NodeIndex>,

    prev_sibling: Option<NodeIndex>,
    next_sibling: Option<NodeIndex>,

    childern: Vec<NodeIndex>,
    data: T,
}

type NodeIndex = usize;

impl<T> Iterator for Node<T> {
    type Item = NodeIndex;
    fn next(&mut self) -> Option<Self::Item> {
        self.next_sibling
    }
}

// TODO currently not a hash tree
impl<T: std::default::Default> HashTree<T> {
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

        if parent.is_some() {
            if !self.nodes[parent.unwrap()].childern.is_empty() {
                prev_sibling = self.nodes[parent.unwrap()].childern.last().copied();
                self.nodes[prev_sibling.unwrap()].next_sibling = Some(index);
            };
            self.nodes[parent.unwrap()].childern.push(index);
        }

        self.nodes.push(Node {
            parent,
            prev_sibling,
            next_sibling: None,
            childern: Vec::new(),
            data,
        });
        index
    }

    pub fn find_node(&self, index: usize) -> &Node<T> {
        &self.nodes[index]
    }

    //TODO panick cases
    pub fn findmut_node(&mut self, index: usize) -> &mut Node<T> {
        &mut self.nodes[index]
    }

    pub fn remove_node(&mut self, index: usize) {
        self.nodes.remove(index);
    }

    pub fn child_iter(&self, index: usize) -> Option<&Vec<NodeIndex>> {
        Some(&self.nodes[index].childern)
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
            tree.find_node(tree.child_iter(0).unwrap()[0]).data,
            TESTSET[1]
        );

        // assert_eq!(tree.find_node(0).first_child, tree.find_node(0).last_child);

        tree.new_node(TESTSET[2], Some(0));

        assert_eq!(
            tree.find_node(*tree.child_iter(0).unwrap().last().unwrap())
                .data,
            TESTSET[2]
        );

        assert_eq!(
            tree.find_node(1).next_sibling,
            Some(tree.child_iter(0).unwrap().last().unwrap()).copied()
        );
    }

    //FIXME make the below test case easyer for the eyes
    #[test]
    fn iter_sblings() {
        let mut tree = HashTree::<isize>::from(TESTSET[0]);

        for i in 1..TESTSET.len() {
            tree.new_node(TESTSET[i], Some(0));
        }
        let mut sib = tree.child_iter(0).unwrap().iter();

        assert_eq!(tree.find_node(*sib.next().unwrap()).data, TESTSET[1]);
        assert_eq!(tree.find_node(*sib.next().unwrap()).data, TESTSET[2]);
        assert_eq!(tree.find_node(*sib.next().unwrap()).data, TESTSET[3]);
        assert_eq!(tree.find_node(*sib.next().unwrap()).data, TESTSET[4]);
        // TESTSET.iter().for_each(|i| tree.insert_child(0, *i));

        // let siblings = tree.find_first_child_of(0).iter();
    }
    #[ignore = "Not impl"]
    #[test]
    fn traverse_tree() {}
}
