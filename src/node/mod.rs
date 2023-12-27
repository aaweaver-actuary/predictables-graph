mod vector;

pub struct Node {
    pub id: u64,
    pub name: String,
    pub children: Vec<Node>,
}
