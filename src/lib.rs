pub struct Tree {
    root: Option<Box<TreeNode>>,
    size: usize,
}

struct TreeNode {
    key: isize,
    value: isize,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
    height: u8,
}

impl Tree {
    pub fn new() -> Self {
        return Self {
            root: None,
            size: 0,
        };
    }

    pub fn insert(&mut self, key: isize, value: isize) -> Option<()> {
        match self.root.as_mut() {
            Some(i) => {
                let result = i.insert(key, value);
                if result == Some(()) {
                    self.size += 1;
                }
                if let Some(mut i) = self.root.take() {
                    i.fix_height();
                    self.root = Some(TreeNode::fix_balance(i));
                }

                return result;
            }
            None => {
                self.root = Some(Box::new(TreeNode::new(key, value)));
                return Some(());
            }
        }
    }

    pub fn get(&self, key: isize) -> Option<isize> {
        match self.root.as_ref() {
            Some(i) => {
                return i.get(key);
            }
            None => {
                return None;
            }
        }
    }

    pub fn show(&self) {
        if let Some(i) = self.root.as_ref() {
            i.show(-1);
        }
    }
}

enum Rotation {
    LeftLeft,
    LeftRight,
    RightLeft,
    RightRight,
    NoRotaion,
}

impl Rotation {
    fn new(node: &TreeNode) -> Self {
        let factor = node.get_factor();
        if factor >= -1 && factor <= 1 {
            return Self::NoRotaion;
        }

        if factor < -1 {
            let child_factor = node.left.as_ref().unwrap().get_factor();
            if child_factor < 0 {
                return Self::LeftLeft;
            } else {
                return Self::LeftRight;
            }
        }

        if factor > 1 {
            let child_factor = node.right.as_ref().unwrap().get_factor();
            if child_factor > 0 {
                return Self::RightRight;
            } else {
                return Self::RightLeft;
            }
        }
        Self::NoRotaion
    }
}

impl TreeNode {
    fn show(&self, parent: isize) {
        if let Some(i) = self.left.as_ref() {
            i.show(self.key);
        }
        println!(
            "Factor: {}, key: {}, value:{}, height: {}, parent: {}",
            self.get_factor(),
            self.key,
            self.value,
            self.height,
            parent
        );
        if let Some(i) = self.right.as_ref() {
            i.show(self.key);
        }
    }
    fn new(key: isize, value: isize) -> TreeNode {
        Self {
            key,
            value,
            left: None,
            right: None,
            height: 0,
        }
    }

    fn insert(&mut self, key: isize, value: isize) -> Option<()> {
        if key == self.key {
            return None;
        }
        let result;
        if self.key > key {
            match self.left.as_mut() {
                Some(i) => {
                    result = i.insert(key, value);
                    self.fix_height();
                }
                None => {
                    self.left = Some(Box::new(Self::new(key, value)));
                    self.fix_height();
                    result = Some(());
                }
            }
        } else {
            match self.right.as_mut() {
                Some(i) => {
                    result = i.insert(key, value);
                    self.fix_height();
                }
                None => {
                    self.right = Some(Box::new(Self::new(key, value)));
                    self.fix_height();
                    result = Some(());
                }
            }
        }
        if let Some(i) = self.right.take() {
            let mut actual = Self::fix_balance(i);
            actual.fix_height();
            self.right = Some(actual);
        }
        if let Some(i) = self.left.take() {
            let mut actual = Self::fix_balance(i);
            actual.fix_height();
            self.left = Some(actual);
        }
        self.fix_height();
        return result;
    }

    fn fix_balance(mut node: Box<TreeNode>) -> Box<TreeNode> {
        let rotation = Rotation::new(&node);
        match rotation {
            Rotation::LeftLeft => {
                return Self::right_rotation(node);
            }
            Rotation::LeftRight => {
                let holder = Self::left_rotation(node.left.take().unwrap());
                node.left = Some(holder);
                return Self::right_rotation(node);
            }
            Rotation::RightLeft => {
                let holder = Self::right_rotation(node.right.take().unwrap());
                node.right = Some(holder);
                return Self::left_rotation(node);
            }
            Rotation::RightRight => {
                return Self::left_rotation(node);
            }
            Rotation::NoRotaion => {
                return node;
            }
        }
    }

    fn get(&self, key: isize) -> Option<isize> {
        if self.key == key {
            return Some(self.value);
        }
        if self.key > key {
            match self.left.as_ref() {
                Some(i) => return i.get(key),
                None => return None,
            };
        }
        if self.key < key {
            match self.right.as_ref() {
                Some(i) => return i.get(key),
                None => return None,
            };
        }
        None
    }

    fn get_heights(&self) -> (u8, u8) {
        let left = match self.left.as_ref() {
            None => 0,
            Some(i) => i.height + 1,
        };
        let right = match self.right.as_ref() {
            None => 0,
            Some(i) => i.height + 1,
        };
        return (left, right);
    }

    fn get_factor(&self) -> isize {
        let (left, right) = self.get_heights();
        return (right as isize) - (left as isize);
    }

    fn fix_height(&mut self) {
        let (left, right) = self.get_heights();
        self.height = left.max(right);
    }

    fn right_rotation(mut base_node: Box<TreeNode>) -> Box<TreeNode> {
        let mut left_node = base_node.left.take().expect("Left must exist");
        let left_right_node = left_node.right.take();
        base_node.left = left_right_node;
        base_node.fix_height();
        left_node.right = Some(base_node);
        left_node.fix_height();
        return left_node;
    }

    fn left_rotation(mut base_node: Box<TreeNode>) -> Box<TreeNode> {
        let mut right_node = base_node.right.take().expect("Right must exist");
        let right_left_node = right_node.left.take();
        base_node.right = right_left_node;
        base_node.fix_height();
        right_node.left = Some(base_node);
        right_node.fix_height();
        return right_node;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut tree = Tree::new();
        tree.insert(4, 3);
        assert_eq!(tree.get(4), Some(3));
        assert_eq!(tree.get(3), None);
    }
}
