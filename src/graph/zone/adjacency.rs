pub enum Adjacency {
    Adjacent,
    NotAdjacent,
}

impl Adjacency {
    pub fn from_int(adj: i8) -> Self {
        match adj {
            0 => Adjacency::NotAdjacent,
            1 => Adjacency::Adjacent,
            _ => panic!("Invalid adjacency value"),
        }
    }

    pub fn to_int(&self) -> i8 {
        match self {
            Adjacency::Adjacent => 1,
            Adjacency::NotAdjacent => 0,
        }
    }

    pub fn from_bool(adj: bool) -> Self {
        match adj {
            true => Adjacency::Adjacent,
            false => Adjacency::NotAdjacent,
        }
    }

    pub fn to_bool(&self) -> bool {
        match self {
            Adjacency::Adjacent => true,
            Adjacency::NotAdjacent => false,
        }
    }
}
