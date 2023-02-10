use std::cell::RefCell;

#[test]
fn main4() {
    use std::rc::Rc;

    enum Exploration {
        Explored,
        UnExplored,
        PartiallyExplored,
    }

    enum Maze {
        Branch {
            label: String,
            left: Rc<Maze>,
            right: Rc<Maze>,
            status: RefCell<Exploration>,
        },
        Leaf { label: String },
    }

    let leaf2 = Rc::new(Maze::Leaf { label: "2".to_owned() });
    let leaf4 = Rc::new(Maze::Leaf { label: "4".to_owned() });
    let leaf5 = Rc::new(Maze::Leaf { label: "5".to_owned() });
    let leaf8 = Rc::new(Maze::Leaf { label: "8".to_owned() });

    let branch3 = Rc::new(Maze::Branch {
        label: "3".to_owned(),
        left: leaf4,
        right: leaf5.clone(),
        status: RefCell::from(Exploration::UnExplored),
    });

    let branch1 = Rc::new(Maze::Branch {
        label: "1".to_owned(),
        left: leaf2,
        right: branch3.clone(),
        status: RefCell::from(Exploration::UnExplored),
    });

    let branch7 = Rc::new(Maze::Branch {
        label: "7".to_owned(),
        left: leaf5.clone(),
        right: leaf8,
        status: RefCell::from(Exploration::UnExplored),
    });

    let branch6 = Rc::new(Maze::Branch {
        label: "6".to_owned(),
        left: branch3.clone(),
        right: branch7,
        status: RefCell::from(Exploration::UnExplored),
    });

    let branch0 = Rc::new(Maze::Branch {
        label: "0".to_owned(),
        left: branch1,
        right: branch6,
        status: RefCell::from(Exploration::UnExplored),
    });
    /* 4 */

    impl Maze {
        fn explore(&self, node: Rc<Self>, work: &mut Vec<Rc<Self>>, trace: &mut Vec<String>) {
            match self {
                Maze::Leaf { label } => {
                    trace.push(label.clone());
                }
                Maze::Branch { label, left, right, status } => {
                    trace.push(label.to_owned());
                    let mut status = status.borrow_mut();
                    match *status {
                        Exploration::Explored => {}
                        Exploration::PartiallyExplored => {
                            *status = Exploration::Explored;
                            work.push(right.clone());
                        }
                        Exploration::UnExplored => {
                            *status = Exploration::PartiallyExplored;
                            work.push(node.clone());
                            work.push(left.clone());
                        }
                    }
                }
            }
        }
    }

    let mut work = vec![Rc::clone(&branch0)]; //Racine de l'arbre
    let mut trace = vec![];
    while work.len() != 0 {
        let node = work.pop().expect("unexpected");
        node.explore(Rc::clone(&node), &mut work, &mut trace);
        println!("trace so far: {:?}", trace);
    }
}
