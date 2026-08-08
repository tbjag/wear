#[derive(Debug, PartialEq)]
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

    fn find_smallest(&self) -> i32 {
        match self {
            Tree::Empty => { unreachable!() }
            Tree::Node(val, left, _) => {
                match &**left {
                    Tree::Empty => *val,
                    _ => left.find_smallest()
                }
            }
        }
    }

    fn delete(&mut self, n:i32) -> bool {
        match self {
            Tree::Empty => false,
            Tree::Node(val, left, right) => {
                if *val == n {
                    match (&**left, &**right) {
                        (Tree::Empty, Tree::Empty) => { 
                            *self = Tree::Empty;
                        }
                        (Tree::Empty, _) => { 
                            let surviving_subtree = std::mem::replace(right, Box::new(Tree::Empty));
                            *self = *surviving_subtree;
                        }
                        (_, Tree::Empty) => { 
                            let surviving_subtree = std::mem::replace(left, Box::new(Tree::Empty));
                            *self = *surviving_subtree;
                        }
                        (Tree::Node(_, _, _), Tree::Node(_, _, _)) => { 
                            *val = right.find_smallest();
                            right.delete(*val);
                        }
                    }
                    true
                } else if *val > n {
                    left.delete(n)
                } else {
                    right.delete(n)
                }
            }
        }
    }
}

fn main() {
    let mut t = Tree::Empty;
    t.insert(5);
    t.insert(4);
    t.insert(6);

    let in_order = t.traverse_in_order();
    println!("in order: {in_order:?}");

    t.delete(5);

    let in_order = t.traverse_in_order();
    println!("in order: {in_order:?}");
    println!("{t:#?}");
}
