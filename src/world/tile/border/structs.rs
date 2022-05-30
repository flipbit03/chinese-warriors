use crate::world::tile::terrain::BaseTerrain;

#[derive(Clone, Debug)]
pub enum TileBorderType {
    VerticalWall = 1,
    HorizontalWall = 2,
    InnerCorner = 4,
    OuterCorner = 8,
    Diagonal = 16,
}

#[derive(Clone, Debug)]
pub struct TileBorder {
    pub terrain: BaseTerrain,
    pub spec: BorderSpec
}

#[derive(Clone, Debug)]
pub struct BorderSpec {
    pub upper_left: Option<TileBorderType>,
    pub upper_right: Option<TileBorderType>,
    pub bottom_left: Option<TileBorderType>,
    pub bottom_right: Option<TileBorderType>,
}

pub enum OpenEnd {
    Top,
    Right,
    Bottom,
    Left
}

pub enum DiagonalLocation {
    TopLeft,
    TopRight,
    BottomRight,
    BottomLeft
}

pub enum TunnelOrientation {
    Horizontal,
    Vertical,
}


impl From<[Option<TileBorderType>; 4]> for BorderSpec {
    fn from(t: [Option<TileBorderType>; 4]) -> Self {
        Self { upper_left: t[0].clone(), upper_right: t[1].clone(), bottom_right: t[2].clone(), bottom_left: t[3].clone() }
    }
}

impl BorderSpec {
    pub fn open_c(open_end: OpenEnd) -> Self {
        let wall = match open_end {
            OpenEnd::Top | OpenEnd::Bottom => Some(TileBorderType::VerticalWall),
            OpenEnd::Right | OpenEnd::Left => Some(TileBorderType::HorizontalWall),
        };

        let mut c = [wall.clone(), wall, Some(TileBorderType::InnerCorner), Some(TileBorderType::InnerCorner)];

        match open_end {
            OpenEnd::Top => c.into(),
            OpenEnd::Right => { c.rotate_right(1); c.into() },
            OpenEnd::Bottom => { c.rotate_right(2); c.into() },
            OpenEnd::Left => { c.rotate_left(1); c.into() },
        }
    }

    pub fn all_inner_corners() -> Self {
        Self { upper_left: Some(TileBorderType::InnerCorner),
            upper_right: Some(TileBorderType::InnerCorner),
            bottom_left: Some(TileBorderType::InnerCorner),
            bottom_right: Some(TileBorderType::InnerCorner),
        }
    }

    pub fn diagonal(location: DiagonalLocation, outer_corner: bool) -> Self {
        let outer_corner_border = match outer_corner {
            true => Some(TileBorderType::OuterCorner),
            false => None,
        };

        let mut c = [Some(TileBorderType::Diagonal), None, outer_corner_border, None];
        
        match location {
            DiagonalLocation::TopLeft => c.into(),
            DiagonalLocation::TopRight => { c.rotate_right(1); c.into()},
            DiagonalLocation::BottomRight => { c.rotate_right(2); c.into()},
            DiagonalLocation::BottomLeft => { c.rotate_left(1); c.into()},
        }
    }

    pub fn tunnel(orientation: TunnelOrientation) -> Self {
        let wall = match orientation {
            TunnelOrientation::Vertical => Some(TileBorderType::VerticalWall),
            TunnelOrientation::Horizontal => Some(TileBorderType::HorizontalWall),
        };

        [wall.clone(), wall.clone(), wall.clone(), wall].into()
    }

}