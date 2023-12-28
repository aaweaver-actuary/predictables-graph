use crate::graph::node::Node;
use crate::math::vector_2d::Vector2D;

/// Each node that occupies space in the graph also occupies space in one of 9 zones.
/// The zones are equal in size, numbered 1-9, and are distributed from the top left going clockwise:
///
/// +---+---+---+
/// | 1 | 2 | 3 |
/// +---+---+---+
/// | 4 | 5 | 6 |
/// +---+---+---+
/// | 7 | 8 | 9 |
/// +---+---+---+
///
/// The point of the zone is to treat points outside of the zone the node is in differently,
/// and in particular:
/// 1. Nodes in the same zone are treated as point masses
/// 2. Nodes in adjacent zones are further assigned to a minor zone, and all the points in the
///    minor zone are treated as a single point mass
/// 3. Nodes in zones that are not adjacent are treated as point masses without any further
///    subdivision
///
/// Zones are considered adjacent if they share a side or a corner. For example, zone 1 is
/// adjacent to zones 2, 4, and 5. Each of these three major zones is further subdivided into 4
/// minor zones, and each minor zone acts as a single point mass for the purposes of calculating
/// the force between a node in zone 1 and a node in zone 2, 4, or 5.
///
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
pub enum MajorZone {
    TopLeft,
    TopMiddle,
    TopRight,
    MiddleLeft,
    MiddleMiddle,
    MiddleRight,
    BottomLeft,
    BottomMiddle,
    BottomRight,
}

impl MajorZone {
    /// Returns an element from the MajorZone enum based on the index provided. The index must be
    /// between 0 and 8, inclusive, or else this function will panic.
    ///
    /// Major zones are organized as follows:
    ///
    /// +---+---+---+
    /// | 1 | 2 | 3 |
    /// +---+---+---+
    /// | 4 | 5 | 6 |
    /// +---+---+---+
    /// | 7 | 8 | 9 |
    /// +---+---+---+
    ///
    /// And they are indexed as follows (0-indexed):
    ///
    /// +---+---+---+
    /// | 0 | 1 | 2 |
    /// +---+---+---+
    /// | 3 | 4 | 5 |
    /// +---+---+---+
    /// | 6 | 7 | 8 |
    /// +---+---+---+
    ///
    pub fn from_index(index: usize) -> Self {
        match index {
            0 => MajorZone::TopLeft,
            1 => MajorZone::TopMiddle,
            2 => MajorZone::TopRight,
            3 => MajorZone::MiddleLeft,
            4 => MajorZone::MiddleMiddle,
            5 => MajorZone::MiddleRight,
            6 => MajorZone::BottomLeft,
            7 => MajorZone::BottomMiddle,
            8 => MajorZone::BottomRight,
            _ => panic!("Invalid index for MajorZone"),
        }
    }

    /// Returns an element from the MajorZone enum based on the zone number provided.
    /// The zone number must be between 1 and 9, inclusive, or else this function will panic.
    ///
    /// Major zones are organized as follows:
    ///
    /// +---+---+---+
    /// | 1 | 2 | 3 |
    /// +---+---+---+
    /// | 4 | 5 | 6 |
    /// +---+---+---+
    /// | 7 | 8 | 9 |
    /// +---+---+---+
    ///
    pub fn from_zone_number(zone_number: usize) -> Self {
        match zone_number {
            1 => MajorZone::TopLeft,
            2 => MajorZone::TopMiddle,
            3 => MajorZone::TopRight,
            4 => MajorZone::MiddleLeft,
            5 => MajorZone::MiddleMiddle,
            6 => MajorZone::MiddleRight,
            7 => MajorZone::BottomLeft,
            8 => MajorZone::BottomMiddle,
            9 => MajorZone::BottomRight,
            _ => panic!("Invalid zone number for MajorZone"),
        }
    }

    /// Returns the zone number of the major zone.
    /// The zone number is between 1 and 9, inclusive.
    /// ### See also
    ///
    /// [MajorZone::from_zone_number](MajorZone::from_zone_number)
    ///
    pub fn get_zone_number(&self) -> usize {
        match self {
            MajorZone::TopLeft => 1,
            MajorZone::TopMiddle => 2,
            MajorZone::TopRight => 3,
            MajorZone::MiddleLeft => 4,
            MajorZone::MiddleMiddle => 5,
            MajorZone::MiddleRight => 6,
            MajorZone::BottomLeft => 7,
            MajorZone::BottomMiddle => 8,
            MajorZone::BottomRight => 9,
        }
    }

    /// Returns the zone index of the major zone.
    /// The zone index is between 0 and 8, inclusive.
    ///
    /// ### See also
    ///
    /// [MajorZone::from_index](MajorZone::from_index)
    ///
    pub fn get_zone_index(&self) -> usize {
        match self {
            MajorZone::TopLeft => 0,
            MajorZone::TopMiddle => 1,
            MajorZone::TopRight => 2,
            MajorZone::MiddleLeft => 3,
            MajorZone::MiddleMiddle => 4,
            MajorZone::MiddleRight => 5,
            MajorZone::BottomLeft => 6,
            MajorZone::BottomMiddle => 7,
            MajorZone::BottomRight => 8,
        }
    }

    /// Returns a vector of the major zones that are adjacent to the major zone.
    ///
    pub fn adjacent(&self) -> Vec<MajorZone> {
        match self {
            MajorZone::TopLeft => vec![
                MajorZone::TopMiddle,
                MajorZone::MiddleLeft,
                MajorZone::MiddleMiddle,
            ],
            MajorZone::TopMiddle => vec![
                MajorZone::TopLeft,
                MajorZone::TopRight,
                MajorZone::MiddleLeft,
                MajorZone::MiddleMiddle,
                MajorZone::MiddleRight,
            ],
            MajorZone::TopRight => vec![
                MajorZone::TopMiddle,
                MajorZone::MiddleMiddle,
                MajorZone::MiddleRight,
            ],
            MajorZone::MiddleLeft => vec![
                MajorZone::TopLeft,
                MajorZone::TopMiddle,
                MajorZone::MiddleMiddle,
                MajorZone::BottomLeft,
                MajorZone::BottomMiddle,
            ],
            MajorZone::MiddleMiddle => vec![
                MajorZone::TopLeft,
                MajorZone::TopMiddle,
                MajorZone::TopRight,
                MajorZone::MiddleLeft,
                MajorZone::MiddleRight,
                MajorZone::BottomLeft,
                MajorZone::BottomMiddle,
                MajorZone::BottomRight,
            ],
            MajorZone::MiddleRight => vec![
                MajorZone::TopMiddle,
                MajorZone::TopRight,
                MajorZone::MiddleMiddle,
                MajorZone::BottomMiddle,
                MajorZone::BottomRight,
            ],
            MajorZone::BottomLeft => vec![
                MajorZone::MiddleLeft,
                MajorZone::MiddleMiddle,
                MajorZone::BottomMiddle,
            ],
            MajorZone::BottomMiddle => vec![
                MajorZone::MiddleLeft,
                MajorZone::MiddleMiddle,
                MajorZone::MiddleRight,
                MajorZone::BottomLeft,
                MajorZone::BottomRight,
            ],
            MajorZone::BottomRight => vec![
                MajorZone::MiddleMiddle,
                MajorZone::MiddleRight,
                MajorZone::BottomMiddle,
            ],
        }
    }

    /// Returns a vector of the major zones that are NOT adjacent to the major zone.
    ///
    /// ### See also
    ///
    /// [MajorZone::adjacent](MajorZone::adjacent)
    ///
    pub fn not_adjacent(&self) -> Vec<MajorZone> {
        match self {
            MajorZone::TopLeft => vec![
                MajorZone::TopRight,
                MajorZone::BottomLeft,
                MajorZone::BottomMiddle,
                MajorZone::BottomRight,
            ],
            MajorZone::TopMiddle => vec![MajorZone::BottomLeft, MajorZone::BottomRight],
            MajorZone::TopRight => vec![
                MajorZone::BottomLeft,
                MajorZone::BottomMiddle,
                MajorZone::BottomRight,
            ],
            MajorZone::MiddleLeft => vec![MajorZone::TopRight, MajorZone::BottomRight],
            MajorZone::MiddleMiddle => vec![MajorZone::BottomRight],
            MajorZone::MiddleRight => vec![
                MajorZone::TopLeft,
                MajorZone::BottomLeft,
                MajorZone::BottomRight,
            ],
            MajorZone::BottomLeft => vec![
                MajorZone::TopRight,
                MajorZone::TopMiddle,
                MajorZone::TopLeft,
            ],
            MajorZone::BottomMiddle => vec![MajorZone::TopLeft, MajorZone::TopRight],
            MajorZone::BottomRight => vec![
                MajorZone::TopLeft,
                MajorZone::TopMiddle,
                MajorZone::TopRight,
            ],
        }
    }

    /// Boolean to test if a major zone is adjacent to another major zone.
    ///
    /// ### See also
    ///
    /// [MajorZone::adjacent](MajorZone::adjacent)
    ///
    pub fn is_adjacent_to(&self, other: &MajorZone) -> bool {
        self.adjacent().contains(other)
    }

    /// Boolean to test if a major zone is NOT adjacent to another major zone.
    ///
    /// ### See also
    ///
    /// [MajorZone::not_adjacent](MajorZone::not_adjacent)
    ///
    pub fn not_adjacent_to(&self, other: &MajorZone) -> bool {
        !self.is_adjacent_to(other)
    }
}
