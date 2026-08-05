#[derive(Debug)]
enum Tree {
    Node(i32, Box<Tree>, Box<Tree>),
    Empty,
}

impl Tree {
    fn insert(&mut self, n: i32) {
        match self {
            Tree::Empty => *self = Tree::Node(n, Box::new(Tree::Empty), Box::new(Tree::Empty)),
            Tree::Node(val, left, right) => if n < *val {
                left.insert(n);
            } else {
                right.insert(n);
            }
        }
    }

    fn traverse_in_order(&self) -> Vec<i32>{
        match self {
            Tree::Empty => vec![],
            Tree::Node(val, left, right) => {
                [
                    left.traverse_in_order(), 
                    vec![*val], 
                    right.traverse_in_order()
                ].concat()
            }
        }
    }

    fn traverse_pre_order(&self) -> Vec<i32>{
        match self {
            Tree::Empty => vec![],
            Tree::Node(val, left, right) => {
                [
                    vec![*val],
                    left.traverse_pre_order() ,
                    right.traverse_pre_order(),
                ].concat()
            }
        }
    }

    fn traverse_post_order(&self) -> Vec<i32>{
        match self {
            Tree::Empty => vec![],
            Tree::Node(val, left, right) => {
                [
                    left.traverse_post_order() ,
                    right.traverse_post_order(),
                    vec![*val],
                ].concat()
            }
        }
    }

    fn contains(&self, n: i32) -> bool {
        match self {
            Tree::Empty => false,
            Tree::Node(val, left, right) => {
                if *val == n {
                    true
                } else if *val > n {
                    left.contains(n)
                } else {
                    right.contains(n)
                }
            }
        }
    }
}

fn main() {
    let mut t = Tree::Empty;
    t.insert(100);
    t.insert(20);
    t.insert(200);
    t.insert(10);
    t.insert(30);
    t.insert(150);
    t.insert(300);

    let in_order = t.traverse_in_order();
    println!("in order: {in_order:?}");

    let pre_order = t.traverse_pre_order();
    println!("pre order: {pre_order:?}");

    let post_order = t.traverse_post_order();
    println!("post order: {post_order:?}");

    let found = t.contains(30);
    println!("contains 30: {found}");

    let found = t.contains(45);
    println!("contains 45: {found}");
}
