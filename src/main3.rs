#[test]
fn main3() {
    use std::rc::Rc;
    use std::cell::RefCell;


    enum Exploration {
        Explored,
        UnExplored,
    }

    enum Maze {
        Branch {
            label: String,
            left: Rc<RefCell<Maze>>,
            right: Rc<RefCell<Maze>>,
            status: Exploration,
        },
        Leaf { label: String },
    }

    let leaf2 = Rc::new(RefCell::new(Maze::Leaf { label: "2".to_owned() }));
    let leaf4 = Rc::new(RefCell::new(Maze::Leaf { label: "4".to_owned() }));
    let leaf5 = Rc::new(RefCell::new(Maze::Leaf { label: "5".to_owned() }));
    let leaf8 = Rc::new(RefCell::new(Maze::Leaf { label: "8".to_owned() }));

    let branch3 = Rc::new(RefCell::new(Maze::Branch {
        label: "3".to_owned(),
        left: leaf4,
        right: leaf5.clone(),
        status: Exploration::UnExplored,
    }));

    let branch1 = Rc::new(RefCell::new(Maze::Branch {
        label: "1".to_owned(),
        left: leaf2,
        right: branch3.clone(),
        status: Exploration::UnExplored,
    }));

    let branch7 = Rc::new(RefCell::new(Maze::Branch {
        label: "7".to_owned(),
        left: leaf5.clone(),
        right: leaf8,
        status: Exploration::UnExplored,
    }));

    let branch6 = Rc::new(RefCell::new(Maze::Branch {
        label: "6".to_owned(),
        left: branch3.clone(),
        right: branch7,
        status: Exploration::UnExplored,
    }));

    let branch0 = Rc::new(RefCell::new(Maze::Branch {
        label: "0".to_owned(),
        left: branch1,
        right: branch6,
        status: Exploration::UnExplored,
    }));





    /*3.1*/

    fn explore(maze: &Rc<RefCell<Maze>>, trace: &mut Vec<String>) {
        let mut maze = maze.borrow_mut();
        match &mut *maze {
            Maze::Branch { label, left, right, status } => {
                trace.push(label.clone());
                match status{
                    Exploration::Explored => {}
                    Exploration::UnExplored => {
                        *status = Exploration::Explored;
                        explore(left, trace);
                        explore(right, trace);
                    }
                }
            },
            Maze::Leaf { label } => {
                trace.push(label.clone());
            },
        }
    }

    let mut trace = vec![];
    explore(&branch0, &mut trace);
    println!("Trace 1 : {:?}", trace);

}
