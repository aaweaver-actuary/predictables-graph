/// Minor zones are numbered 1-4, and are distributed from the top left going clockwise:
///
/// +---+---+
/// | 1 | 2 |
/// +---+---+
/// | 3 | 4 |
/// +---+---+
///
/// Minor zones do not have any special meaning outside of the context of determining which nodes
/// are grouped together for the purposes of calculating the force between two nodes. For example,
/// if a node is in zone 1 (and thus adjacent to zone 2), then for the purpose of calculating the
/// force between the node in zone 1 and any node in zone 2, the node in zone 2 is considered to
/// be in some minor zone of zone 2, and all the nodes in that minor zone are treated as a single
/// point mass. However, looking at the opposite side of the equation, that a node is in major zone
/// 2, minor zone 1, has no special meaning, and does not affect the force calculation between a
/// different node in major zone 2, minor zone 2.
///
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
pub enum MinorZone {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}
