use super::structs::TileBorderType;

pub fn get_border_from_neighbor_effects(
    horizontal_neighbor: bool,
    vertical_neighbor: bool,
    diagonal_neighbor: bool,
) -> Option<TileBorderType> {
    match (horizontal_neighbor, vertical_neighbor, diagonal_neighbor) {
        (true, true, true) | (true, true, false) => Some(TileBorderType::InnerCorner),
        (true, false, true) | (true, false, false) => Some(TileBorderType::VerticalWall),
        (false, true, true) | (false, true, false) => {
            Some(TileBorderType::HorizontalWall)
        }
        (false, false, true) => Some(TileBorderType::OuterCorner),
        (false, false, false) => None,
    }
}
