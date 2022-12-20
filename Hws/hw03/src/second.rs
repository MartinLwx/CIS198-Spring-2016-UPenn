use std::cmp::Ordering;

#[derive(Debug)]
struct Node<T> {
    val: T,
    left: Link<T>,
    right: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
pub struct BST<T> {
    root: Link<T>,
}

trait InsertSearch<T> {
    // avoid conflicts with the `insert` function
    fn my_insert(&mut self, e: T) -> bool;
    fn my_search(&self, e: T) -> bool;
}

impl<T: Ord> InsertSearch<T> for Link<T> {
    fn my_insert(&mut self, e: T) -> bool {
         match self {
             None => {
                 *self = Some(Box::new(Node { val: e, left: None, right: None }));
                 return true;
             }
             Some(node) => match node.val.cmp(&e) {
                 Ordering::Equal => false,
                 Ordering::Less => node.right.my_insert(e),
                 Ordering::Greater => node.left.my_insert(e),
             },
         }
        
    }
    fn my_search(&self, e: T) -> bool {
         match self {
             None => false,
             Some(ref node) => match node.val.cmp(&e) {
                 Ordering::Equal => true,
                 Ordering::Less => node.right.my_search(e),
                 Ordering::Greater => node.left.my_search(e),
             },
         }
    }
}

impl<T: Ord> BST<T> {
    pub fn new() -> Self {
        BST { root: None }
    }

    pub fn insert(&mut self, val: T) -> bool {
        self.root.my_insert(val)
    }

    pub fn search(&self, target: T) -> bool {
        self.root.my_search(target)
    }
}

/// for ... in bst
pub struct BSTIntoIter<T> {
    current_node: Link<T>
}

impl<T> Iterator for BSTIntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.current_node.is_none() {
            return None
        }
        // the order matters, we should first extract right edge
        let next_node = self.current_node.as_mut().unwrap().right.take();
        let current = self.current_node.take().unwrap().val;
        self.current_node = next_node;
        Some(current)
    }
}

impl<T> IntoIterator for BST<T> {
    type Item = T;
    type IntoIter = BSTIntoIter<T>;
    fn into_iter(self) -> Self::IntoIter {
        let iter = BSTIntoIter { current_node: self.root };
        iter
    }
}


/// for ... in &bst
pub struct BSTIter<'a, T> {
    current_node: Option<&'a Node<T>>
}


impl<'a, T> IntoIterator for &'a BST<T> {
    type Item = &'a T;
    type IntoIter = BSTIter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        let iter = BSTIter { current_node: self.root.as_deref()};
        iter
    }
}

impl<'a, T> Iterator for BSTIter<'a, T> {
    type Item =  &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.current_node.map(|node| {
            self.current_node = node.right.as_deref();
            &node.val
        })
    }
}

/// for ... in &mut bst
pub struct BSTIterMut<'a, T> {
    current_node: Option<&'a mut Node<T>>
}


impl<'a, T> IntoIterator for &'a mut BST<T> {
    type Item = &'a mut T;
    type IntoIter = BSTIterMut<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        let iter = BSTIterMut { current_node: self.root.as_deref_mut() };
        iter
    }
}

impl<'a, T> Iterator for BSTIterMut<'a, T> {
    type Item =  &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        self.current_node.take().map(|node| {
            self.current_node = node.right.as_deref_mut();
            &mut node.val
        })
    }
}



#[cfg(test)]
mod test {
    use super::BST;

    #[test]
    fn test_insert_search() {
        let mut some_bst: BST<i32> = BST::new();
        println!("-------------------------------");
        println!("Empty BST");
        println!("{:#?}", some_bst);
        println!("-------------------------------");

        some_bst.insert(3);
        assert_eq!(some_bst.search(3), true);
        println!("-------------------------------");
        println!("Add single node 3 to the BST. Current nodes: 3");
        println!("{:#?}", some_bst);
        println!("-------------------------------");

        some_bst.insert(1);
        println!("-------------------------------");
        assert_eq!(some_bst.search(1), true);
        println!("Add single node 1 to the BST. Current nodes: 1, 3");
        println!("{:#?}", some_bst);
        println!("-------------------------------");

        some_bst.insert(0);
        some_bst.insert(2);
        assert_eq!(some_bst.search(0), true);
        assert_eq!(some_bst.search(2), true);
        println!("-------------------------------");
        println!("Add single node 0, 2 to the BST. Current nodes: 0, 1, 2, 3");
        println!("{:#?}", some_bst);
        println!("-------------------------------");

        some_bst.insert(6);
        assert_eq!(some_bst.search(6), true);
        println!("-------------------------------");
        println!("Add single node 6 to the BST. Current nodes: 0, 1, 2, 3, 6");
        println!("{:#?}", some_bst);
        println!("-------------------------------");

        assert_eq!(some_bst.search(5), false);
        println!("-------------------------------");
        println!("Search a node not in the BST: 5");
        println!("{:#?}", some_bst);
        println!("-------------------------------");

        some_bst.insert(5);
        assert_eq!(some_bst.search(5), true);
        println!("-------------------------------");
        println!("Add single node 5 to the BST. Current nodes: 0, 1, 2, 3, 5, 6");
        println!("{:#?}", some_bst);
        println!("-------------------------------");

        println!("-------------------------------");
        println!("Insert a node previous inserted(5) should return false");
        assert_eq!(some_bst.insert(5), false);
        println!("-------------------------------");
    }

    #[test]
    fn test_intoiter() {
        let mut bst = BST::new();
        bst.insert(1);
        bst.insert(2);
        bst.insert(3);

        let mut iterator = bst.into_iter();
        assert_eq!(iterator.next(), Some(1));
        assert_eq!(iterator.next(), Some(2));
        assert_eq!(iterator.next(), Some(3));
        assert_eq!(iterator.next(), None); 
    }

    #[test]
    fn test_iter() {
        let mut bst = BST::new();
        bst.insert(2);
        bst.insert(1);
        bst.insert(3);

        let mut iterator = (&bst).into_iter();
        assert_eq!(iterator.next(), Some(&2));
        assert_eq!(iterator.next(), Some(&3));
        assert_eq!(iterator.next(), None); 
    }

    #[test]
    fn test_iter_mut() {
        let mut bst = BST::new();
        bst.insert(2);
        bst.insert(1);
        bst.insert(3);

        let mut iterator = (&mut bst).into_iter();
        assert_eq!(iterator.next(), Some(&mut 2));
        assert_eq!(iterator.next(), Some(&mut 3));
        assert_eq!(iterator.next(), None); 
    }
    
    #[test]
    fn test_three_kinds_forloops() {
        let mut bst = BST::new();
        bst.insert(1);
        bst.insert(2);
        bst.insert(3);

        for elt in bst { // calls bst.into_iter()
            println!("{}", elt);
        }

        let mut bst = BST::new();
        bst.insert(1);
        bst.insert(2);
        bst.insert(3);

        for elt in &bst { // calls (&bst).into_iter()
            println!("{}", elt);
        }

        let mut bst = BST::new();
        bst.insert(1);
        bst.insert(2);
        bst.insert(3);

        for elt in &mut bst { // calls (&mut bst).into_iter()
            println!("{}", elt);
        }
    }
}
