pub struct TreeNode<T> {
    value: T,
    children: Vec<TreeNode<T>>,
}

impl<T> TreeNode<T> {
    pub fn new(value: T) -> Self {
        TreeNode {
            value,
            children: Vec::new(),
        }
    }

    pub fn add_child(&mut self, value: T) -> &mut TreeNode<T> {
        self.children.push(TreeNode::new(value));
        self.children.last_mut().unwrap()
    }

    pub fn iter(&self) -> InOrderIterator<T> {
        InOrderIterator::new(self)
    }

    pub fn has_child(&self, value: &T) -> bool
    where
        T: PartialEq,
    {
        self.children.iter().any(|child| child.value == *value)
    }

    pub fn get_child(&self, value: &T) -> Option<&TreeNode<T>>
    where
        T: PartialEq,
    {
        self.children.iter().find(|child| child.value == *value)
    }

    pub fn get_child_mut(&mut self, value: &T) -> Option<&mut TreeNode<T>>
    where
        T: PartialEq,
    {
        self.children.iter_mut().find(|child| child.value == *value)
    }

    pub fn get_or_create_child(&mut self, value: T) -> &mut TreeNode<T>
    where
        T: PartialEq,
    {
        if self.has_child(&value) {
            self.get_child_mut(&value).unwrap()
        } else {
            self.add_child(value)
        }
    }

    pub fn get_value(&self) -> &T {
        &self.value
    }

    pub fn get_children(&self) -> &Vec<TreeNode<T>> {
        &self.children
    }
}

pub struct InOrderIterator<'a, T> {
    stack: Vec<(&'a TreeNode<T>, usize)>,
}

impl<'a, T> InOrderIterator<'a, T> {
    fn new(root: &'a TreeNode<T>) -> Self {
        let mut stack = Vec::new();
        stack.push((root, 0));
        InOrderIterator { stack }
    }
}

impl<'a, T> Iterator for InOrderIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some((node, index)) = self.stack.pop() {
            if index < node.children.len() {
                self.stack.push((node, index + 1));

                let child = &node.children[index];
                self.stack.push((child, 0));

                return Some(&child.value);
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Simple test
    #[test]
    fn test() {
        let mut root = TreeNode::new(1);

        let mut child1 = TreeNode::new(2);
        child1.add_child(4);
        child1.add_child(5);

        let mut child2 = TreeNode::new(3);
        child2.add_child(6);
        child2.add_child(7);

        root.add_child(child1.value);
        root.add_child(child2.value);

        let mut it = InOrderIterator::new(&root);
        while let Some(value) = it.next() {
            print!("{} ", value);
        }
        // Output: 2 4 5 3 6 7
    }

}
