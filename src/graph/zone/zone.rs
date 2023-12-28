use crate::graph::zone::major_zone::MajorZone;
use crate::graph::zone::minor_zone::MinorZone;
use crate::math::vector_2d::Vector2D;

enum ZoneType {
    Canvas,
    Major,
    Minor,
}

impl ZoneType {
    pub fn n_sub_zones(&self) -> Option<u8> {
        match self {
            ZoneType::Canvas => Some(9),
            ZoneType::Major => Some(4),
            ZoneType::Minor => Some(0),
            _ => None,
        }
    }

    pub fn sub_zone_type(&self) -> Option<ZoneType> {
        match self {
            ZoneType::Canvas => Some(ZoneType::Major),
            ZoneType::Major => Some(ZoneType::Minor),
            ZoneType::Minor => None,
            _ => None,
        }
    }

    pub fn sub_zone_vec(&self) -> Option<Vec<ZoneType>> {
        let mut vec = vec![];
        let n_sub_zones = self.n_sub_zones();
        let sub_zone_type = self.sub_zone_type();

        if n_sub_zones.is_some() && sub_zone_type.is_some() {
            for i in 0..n_sub_zones.unwrap() {
                vec.push(sub_zone_type.unwrap());
            }
            Some(vec)
        } else {
            None
        }
    }
}

struct Zone {
    zone_type: Option<ZoneType>,
    zone_id: Option<u8>,
    n_sub_zones: Option<u8>,
    sub_zones: Option<Vec<Zone>>,
    top_left: Option<Vector2D<f64>>,
    bottom_right: Option<Vector2D<f64>>,
}

struct ZoneBuilder {
    zone_type: Option<ZoneType>,
    zone_id: Option<u8>,
    n_sub_zones: Option<u8>,
    sub_zones: Option<Vec<Zone>>,
    top_left: Option<Vector2D<f64>>,
    bottom_right: Option<Vector2D<f64>>,
}

impl ZoneBuilder {
    pub fn new() -> Self {
        ZoneBuilder {
            zone_type: None,
            zone_id: None,
            n_sub_zones: None,
            sub_zones: None,
            top_left: None,
            bottom_right: None,
        }
    }

    pub fn from_zone_type(zone_type: ZoneType, zone_id: Option<u8>) -> Self {
        ZoneBuilder {
            zone_type: Some(zone_type),
            zone_id: Some(zone_id),
            n_sub_zones: zone_type.n_sub_zones(),
            sub_zones: zone_type.sub_zone_vec(),
            top_left: None,
            bottom_right: None,
        }
    }

    pub fn zone_type(mut self, zone_type: ZoneType) -> Self {
        self.zone_type = Some(zone_type);
        self
    }

    pub fn n_sub_zones(mut self, n_sub_zones: u8) -> Self {
        self.n_sub_zones = Some(n_sub_zones);
        self
    }

    pub fn sub_zones(mut self, sub_zones: Vec<Zone>) -> Self {
        self.sub_zones = Some(sub_zones);
        self
    }

    pub fn top_left(mut self, top_left: Vector2D<f64>) -> Self {
        self.top_left = Some(top_left);
        self
    }

    pub fn bottom_right(mut self, bottom_right: Vector2D<f64>) -> Self {
        self.bottom_right = Some(bottom_right);
        self
    }

    pub fn build(self) -> Zone {
        Zone {
            zone_type: self.zone_type,
            n_sub_zones: self.n_sub_zones,
            sub_zones: self.sub_zones,
            top_left: self.top_left,
            bottom_right: self.bottom_right,
        }
    }
}
