use crate::{tetromino::{GridPosition}, steering::Direction};

type Pos = GridPosition;

// for now, only direction DOWN implementation, as it is not needed to implement other 
pub fn are_colliding(block1 : &Vec<&Pos>, block2 : &Vec<Pos>) -> bool {
    let valid_xs : Vec<i16> = block1
        .iter()
        .map(|pos| pos.x).collect();
    let valid_b2_pos : Vec<&Pos> = block2
        .iter()
        .filter(|pos| valid_xs.contains(&pos.x))
        .collect();

    for b1_pos in block1 {
        for b2_pos in &valid_b2_pos {
            if b1_pos.x == b2_pos.x && b1_pos.y == b2_pos.y - 1 {
                return true
            }
        }
    }

    return false;

}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_collision_no_colision() {
        let moving_item = vec![
            Pos::new(0, 0),
            Pos::new(1, 0),
            Pos::new(1, 1),
            Pos::new(2, 1)
        ];
        let stationary_item = vec![
            Pos::new(0, 3),
            Pos::new(1, 3),
            Pos::new(2, 3),
            Pos::new(3, 3)
        ];
        
        assert!(!are_colliding(&moving_item, &stationary_item));
    }
    #[test]
    fn test_collision_collision() {
        let moving_item = vec![
            Pos::new(0, 0),
            Pos::new(1, 0),
            Pos::new(2, 0),
            Pos::new(1, 1)
        ];
        let stationary_item = vec![
            Pos::new(0, 1),
            Pos::new(0, 2),
            Pos::new(1, 2),
            Pos::new(1, 3)
        ];
        assert!(are_colliding(&moving_item, &stationary_item));
    }

    
}














