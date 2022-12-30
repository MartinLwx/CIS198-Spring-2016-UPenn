use std::cmp::Ordering;

#[derive(Debug)]
struct Node {
    val: i32,
    left: Link,
    right: Link,
}

#[derive(Debug)]
enum Link {
    Empty,
    More(Box<Node>),
}

#[derive(Debug)]
pub struct BST {
    root: Link,
}

impl Link {
    fn insert(&mut self, val: i32) -> bool {
        match self {
            Link::Empty => {
                *self = Link::More(Box::new(Node {
                    val: val,
                    left: Link::Empty,
                    right: Link::Empty,
                }));
                return true;
            }
            Link::More(node) => match node.val.cmp(&val) {
                Ordering::Equal => return false,
                Ordering::Less => node.right.insert(val),
                Ordering::Greater => node.left.insert(val),
            },
        }
    }
    fn search(&self, target: i32) -> bool {
        match self {
            Link::Empty => return false,
            Link::More(node) => match node.val.cmp(&target) {
                Ordering::Equal => return true,
                Ordering::Less => node.right.search(target),
                Ordering::Greater => node.left.search(target),
            },
        }
    }
}

impl BST {
    pub fn new() -> Self {
        BST { root: Link::Empty }
    }

    pub fn insert(&mut self, val: i32) -> bool {
        self.root.insert(val)
    }

    pub fn search(&self, target: i32) -> bool {
        self.root.search(target)
    }
}

#[cfg(test)]
mod test {
    use super::BST;

    #[test]
    fn test_insert_search() {
        let mut some_bst = BST::new();
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
}
