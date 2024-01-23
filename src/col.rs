use crate::steering::Direction;
use crate::block::Pos;

pub fn are_colliding(block1 : &Vec<Pos>, block2 : &Vec<Pos>, dir : &Direction) -> bool {
    let valid_xs : Vec<i16> = block1
        .iter()
        .map(|pos| pos.x).collect();
    let valid_ys : Vec<i16> = block1
        .iter()
        .map(|pos| pos.y).collect();
    let valid_b2_pos_x : Vec<&Pos> = block2
        .iter()
        .filter(|pos| valid_xs.contains(&pos.x))
        .collect();
    let valid_b2_pos_y : Vec<&Pos> = block2
        .iter()
        .filter(|pos| valid_ys.contains(&pos.y))
        .collect();
    match dir {
        Direction::Down => {
            for b1_pos in block1 {
                for b2_pos in &valid_b2_pos_x {
                    if b1_pos.x == b2_pos.x && b1_pos.y == b2_pos.y - 1 {
                        return true
                    }
                }
            }
        },
        Direction::Left => {
            for b1_pos in block1 {
                for b2_pos in &valid_b2_pos_y {
                    if b1_pos.y == b2_pos.y && b1_pos.x == b2_pos.x + 1{
                        return true
                    }
                }
            }
        },
        Direction::Right => {
            for b1_pos in block1 {
                for b2_pos in &valid_b2_pos_y {
                    if b1_pos.y == b2_pos.y && b1_pos.x == b2_pos.x - 1 {
                        return true
                    }
                }
            }
        },
        _=> return false

    }
    return false;

}
















