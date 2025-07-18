use std::clone::Clone;
use std::cmp::PartialEq;
use std::cmp::PartialOrd;
use std::fmt::Display;

enum Rotation {
    LeftLeft,
    LeftRight,
    RightLeft,
    RightRight,
    NoRotaion,
}

pub struct Tree<
    K: Clone + PartialEq + Display + PartialOrd,
    V: Clone + PartialEq + Display + PartialOrd,
> {
    root: Option<Box<TreeNode<K, V>>>,
    pub size: usize,
}

struct TreeNode<
    K: Clone + Display + PartialEq + PartialOrd,
    V: Clone + Display + PartialEq + PartialOrd,
> {
    key: K,
    value: V,
    left: Option<Box<TreeNode<K, V>>>,
    right: Option<Box<TreeNode<K, V>>>,
    height: u8,
}

impl<K: Clone + Display + PartialEq + PartialOrd, V: Clone + Display + PartialEq + PartialOrd>
    Tree<K, V>
{
    pub fn new() -> Self {
        Self {
            root: None,
            size: 0,
        }
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<()> {
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
                self.size += 1;
                return Some(());
            }
        }
    }

    pub fn get(&self, key: K) -> Option<V> {
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
            i.show(self.root.as_ref().unwrap().key.clone());
        }
    }

    pub fn update(&mut self, key: K, new_value: V) -> Option<V> {
        if let Some(i) = self.root.as_mut() {
            return i.update_value(key, new_value);
        }
        None
    }

    pub fn delete(&mut self, key: K) -> Option<V> {
        if let Some(i) = self.root.take() {
            let (new_root, value) = TreeNode::delete(i, key);
            self.root = new_root;
            match value {
                Some(node) => {
                    self.size -= 1;
                    return Some(node.value);
                }
                None => return None,
            }
        }
        return None;
    }
}

impl<K: Clone + PartialEq + Display + PartialOrd, V: Clone + Display + PartialEq + PartialOrd>
    TreeNode<K, V>
{
    fn show(&self, parent: K) {
        if let Some(i) = self.left.as_ref() {
            println!("Left");
            i.show(self.key.clone());
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
            i.show(self.key.clone());
            println!("Right");
        }
    }
    fn new(key: K, value: V) -> TreeNode<K, V> {
        Self {
            key,
            value,
            left: None,
            right: None,
            height: 0,
        }
    }

    fn insert(&mut self, key: K, value: V) -> Option<()> {
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

    fn fix_balance(mut node: Box<TreeNode<K, V>>) -> Box<TreeNode<K, V>> {
        let rotation = Self::new_rotation(&node);
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

    fn get(&self, key: K) -> Option<V> {
        if self.key == key {
            return Some(self.value.clone());
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

    fn right_rotation(mut base_node: Box<TreeNode<K, V>>) -> Box<TreeNode<K, V>> {
        let mut left_node = base_node.left.take().expect("Left must exist");
        let left_right_node = left_node.right.take();
        base_node.left = left_right_node;
        base_node.fix_height();
        left_node.right = Some(base_node);
        left_node.fix_height();
        return left_node;
    }

    fn left_rotation(mut base_node: Box<TreeNode<K, V>>) -> Box<TreeNode<K, V>> {
        let mut right_node = base_node.right.take().expect("Right must exist");
        let right_left_node = right_node.left.take();
        base_node.right = right_left_node;
        base_node.fix_height();
        right_node.left = Some(base_node);
        right_node.fix_height();
        return right_node;
    }

    fn update_value(&mut self, key: K, new_value: V) -> Option<V> {
        if self.key == key {
            let old_value = self.value.clone();
            self.value = new_value;
            return Some(old_value);
        }

        if self.key > key {
            if let Some(i) = self.left.as_mut() {
                return i.update_value(key, new_value);
            } else {
                return None;
            }
        }

        if self.key > key {
            if let Some(i) = self.right.as_mut() {
                return i.update_value(key, new_value);
            } else {
                return None;
            }
        }
        None
    }

    fn delete(
        mut actual_node: Box<TreeNode<K, V>>,
        key: K,
    ) -> (Option<Box<TreeNode<K, V>>>, Option<Box<TreeNode<K, V>>>) {
        if actual_node.key == key {
            let (replace, deleted_node) = Self::delete_node(actual_node);

            return (replace, Some(deleted_node));
        }

        if actual_node.key > key {
            if let Some(i) = actual_node.left.take() {
                let (replace, deleted_node) = Self::delete(i, key);
                actual_node.left = replace;
                actual_node.fix_height();
                return (Some(Self::fix_balance(actual_node)), deleted_node);
            } else {
                return (Some(actual_node), None);
            }
        } else if let Some(i) = actual_node.right.take() {
            let (replace, deleted_node) = Self::delete(i, key);
            actual_node.right = replace;
            actual_node.fix_height();
            return (Some(Self::fix_balance(actual_node)), deleted_node);
        } else {
            return (Some(actual_node), None);
        }
    }

    fn delete_node(
        actual_node: Box<TreeNode<K, V>>,
    ) -> (Option<Box<TreeNode<K, V>>>, Box<TreeNode<K, V>>) {
        if actual_node.left.is_some() {
            let (pred, new_actual) = Self::get_predecessor(actual_node);
            let mut pred = pred.unwrap();
            let mut new_actual = new_actual.unwrap();
            pred.left = new_actual.left.take();
            pred.right = new_actual.right.take();
            pred.fix_height();
            return (Some(Self::fix_balance(pred)), new_actual);
        }

        if actual_node.right.is_some() {
            let (pred, new_actual) = Self::get_successor(actual_node);
            let mut pred = pred.unwrap();
            let mut new_actual = new_actual.unwrap();
            pred.left = new_actual.left.take();
            pred.right = new_actual.right.take();

            pred.fix_height();
            return (Some(Self::fix_balance(pred)), new_actual);
        }

        return (None, actual_node);
    }

    fn get_successor(
        mut actual_node: Box<TreeNode<K, V>>,
    ) -> (Option<Box<TreeNode<K, V>>>, Option<Box<TreeNode<K, V>>>) {
        if let Some(i) = actual_node.right.take() {
            let (succ, new_right_node) = Self::get_lowest(i);
            actual_node.right = new_right_node;
            actual_node.fix_height();
            return (Some(succ), Some(Self::fix_balance(actual_node)));
        }
        (None, None)
    }

    fn get_predecessor(
        mut actual_node: Box<TreeNode<K, V>>,
    ) -> (Option<Box<TreeNode<K, V>>>, Option<Box<TreeNode<K, V>>>) {
        if let Some(i) = actual_node.left.take() {
            let (pred, new_left_node) = Self::get_greatest(i);
            actual_node.left = new_left_node;
            actual_node.fix_height();
            return (Some(pred), Some(Self::fix_balance(actual_node)));
        }
        (None, None)
    }

    fn get_lowest(
        mut actual_node: Box<TreeNode<K, V>>,
    ) -> (Box<TreeNode<K, V>>, Option<Box<TreeNode<K, V>>>) {
        if let Some(i) = actual_node.left.take() {
            let (lowest, fixed_node) = Self::get_lowest(i);
            actual_node.left = fixed_node;
            actual_node.fix_height();
            return (lowest, Some(Self::fix_balance(actual_node)));
        }
        let right_node = actual_node.right.take();
        return (actual_node, right_node);
    }

    fn get_greatest(
        mut actual_node: Box<TreeNode<K, V>>,
    ) -> (Box<TreeNode<K, V>>, Option<Box<TreeNode<K, V>>>) {
        if let Some(i) = actual_node.right.take() {
            let (greatest, fixed_node) = Self::get_greatest(i);
            actual_node.right = fixed_node;
            actual_node.fix_height();
            return (greatest, Some(Self::fix_balance(actual_node)));
        }
        let left_node = actual_node.left.take();
        return (actual_node, left_node);
    }

    fn new_rotation(node: &TreeNode<K, V>) -> Rotation {
        let factor = node.get_factor();
        if factor >= -1 && factor <= 1 {
            return Rotation::NoRotaion;
        }

        if factor < -1 {
            let child_factor = node.left.as_ref().unwrap().get_factor();
            if child_factor < 0 {
                return Rotation::LeftLeft;
            } else {
                return Rotation::LeftRight;
            }
        }

        if factor > 1 {
            let child_factor = node.right.as_ref().unwrap().get_factor();
            if child_factor > 0 {
                return Rotation::RightRight;
            } else {
                return Rotation::RightLeft;
            }
        }
        Rotation::NoRotaion
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut tree = Tree::<isize, isize>::new();
        tree.insert(4, 3);
        assert_eq!(tree.get(4), Some(3));
        assert_eq!(tree.get(3), None);
    }

    #[test]
    fn it_wors_string() {
        let mut tree = Tree::<String, String>::new();
        tree.insert("a".into(), "a".into());
        assert_eq!(tree.get("a".into()), Some(String::from("a")));
        assert_eq!(tree.get("b".into()), None);
    }

    #[test]
    fn key_string_value_isize() {
        let mut tree = Tree::<String, isize>::new();
        for i in 0..100 {
            let key = format!("{}", i);
            tree.insert(key, i);
        }
        for i in 0..100 {
            let key = format!("{}", i);
            let got = tree.get(key.clone());
            assert_eq!(got, Some(i));
        }

        for i in -100..0 {
            let key = format!("{}", i);
            let got = tree.get(key.clone());
            assert_eq!(got, None);
        }
        for i in 100..200 {
            let key = format!("{}", i);
            let got = tree.get(key.clone());
            assert_eq!(got, None);
        }
    }
}
