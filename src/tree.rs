use std::collections::HashMap;
use std::collections::VecDeque;

#[derive(Debug, PartialEq)]
pub enum FSNode {
    Node(String, HashMap<String, FSNode>),
    File(String, usize),
}

impl FSNode {
    pub fn new(data: String) -> FSNode {
        FSNode::Node(data, HashMap::new())
    }

    pub fn name(&self) -> String {
        match self {
            FSNode::Node(name, ..) => name.clone(),
            FSNode::File(name, ..) => name.clone(),
        }
    }

    pub fn add_child(&mut self, child: FSNode) {
        if let FSNode::Node(_, children) = self {
            // children.push(child);
            children.insert(child.name(), child);
        }
    }

    pub fn zipper(self) -> FSNodeZipper {
        FSNodeZipper {
            node: self,
            parent: None,
            index_in_parent: String::new(),
        }
    }

    pub fn size_below(&self) -> usize {
        match self {
            FSNode::File(_, size) => *size,
            FSNode::Node(_, children) => children.values().map(|c| c.size_below()).sum(),
        }
    }

    pub fn visit_nodes<'a, F>(&'a self, mut callback: F)
    where
        F: FnMut(&'a FSNode) -> (),
    {
        let mut queue = VecDeque::from([self]);
        while let Some(node) = queue.pop_front() {
            callback(node);
            if let FSNode::Node(_, children) = node {
                for child in children.values() {
                    queue.push_back(child)
                }
            }
        }
    }

    pub fn is_directory(&self) -> bool {
        match self {
            FSNode::Node(..) => true,
            FSNode::File(..) => false,
        }
    }
}

#[derive(Debug)]
pub struct FSNodeZipper {
    node: FSNode,
    parent: Option<Box<FSNodeZipper>>,
    index_in_parent: String,
}

impl FSNodeZipper {
    pub fn child(mut self, index: &String) -> Option<FSNodeZipper> {
        // Remove the specified child from the node's children.
        // A NodeZipper shouldn't let its users inspect its parent,
        // since we mutate the parents
        // to move the focused nodes out of their list of children.
        if let FSNode::Node(_, ref mut children) = self.node {
            let child = children.remove(index);

            if let Some(child) = child {
                // Return a new NodeZipper focused on the specified child.
                return Some(FSNodeZipper {
                    node: child,
                    parent: Some(Box::new(self)),
                    index_in_parent: index.clone(),
                });
            }
        }

        None
    }

    pub fn parent(self) -> FSNodeZipper {
        // Destructure this NodeZipper
        let FSNodeZipper {
            node,
            parent,
            index_in_parent,
        } = self;

        // Destructure the parent NodeZipper
        let FSNodeZipper {
            node: mut parent_node,
            parent: parent_parent,
            index_in_parent: parent_index_in_parent,
        } = *parent.unwrap();

        if let FSNode::Node(_, ref mut children) = parent_node {
            children.insert(index_in_parent, node);
        }

        // Return a new NodeZipper focused on the parent.
        FSNodeZipper {
            node: parent_node,
            parent: parent_parent,
            index_in_parent: parent_index_in_parent,
        }
    }

    pub fn finish(mut self) -> FSNode {
        while let Some(_) = self.parent {
            self = self.parent();
        }

        self.node
    }

    pub fn add_child(mut self, child: FSNode) -> FSNodeZipper {
        self.node.add_child(child);

        self
    }
}
