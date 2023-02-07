enum Maze {
    Branch {
        label: String,
        left: Box<Tree>,
        right: Box<Tree>,
        status: Status,
    },
    Leaf { label: String },
}