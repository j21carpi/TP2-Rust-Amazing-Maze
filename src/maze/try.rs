use std::collections::VecDeque;

enum Tree {
    Branch {
        label: String,
        left: Box<Tree>,
        right: Box<Tree>,
        status: Status,
    },
    Leaf { label: String },
}

enum Status {
    UnExplored,
    Explored,
}


let leaf2 = Tree::Leaf(String::from("2"))

let leaf2 = Leaf{
    label: "2".to_string(),
};

let leaf4 = Leaf {
    label: "4".to_string(),
};
let leaf5 = Tree::Leaf {
    label: "5".to_string(),
};
let leaf8 = Tree::Leaf {
    label: "8".to_string(),
};
let branch3 = Tree::Branch {
    label: "3".to_string(),
    left: Box::new(leaf4),
    right: Box::new(leaf5),
    status: Status::UnExplored,
};
let branch1 = Tree::Branch {
    label: "1".to_string(),
    left: Box::new(leaf2),
    right: Box::new(branch3),
    status: Status::UnExplored,
};
let branch7 = Tree::Branch {
    label: "7".to_string(),
    left: Box::new(leaf5),
    right: Box::new(leaf8),
    status: Status::UnExplored,
};
let branch6 = Tree::Branch {
    label: "6".to_string(),
    left: Box::new(branch3),
    right: Box::new(branch7),
    status: Status::UnExplored,
};
let branch0 = Tree::Branch {
    label: "0".to_string(),
    left: Box::new(branch1),
    right: Box::new(branch6),
    status: Status::UnExplored,
};




impl Tree {
    fn explore(&mut self, trace: &mut VecDeque<String>) {
        match self {
            Tree::Branch {
                label,
                left,
                right,
                status,
            } => match status {
                Status::UnExplored => {
                    *status = Status::Explored;
                    trace.push_back(label.clone());
                    left.explore(trace);
                    right.explore(trace);
                }
                Status::Explored => trace.push_back(label.clone()),
            },
            Tree::Leaf { label } => trace.push_back(label.clone()),
        }
    }
}