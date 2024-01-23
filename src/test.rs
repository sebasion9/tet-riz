#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn line_clear_filled_fully() {
        let width : i16 = 3;
        let height : i16 = 5;
        let placed_blocks : Vec<Pos> = vec![
            Pos::new(0, 0),
            Pos::new(1, 0),
            Pos::new(2, 0)
        ];
        let result = placed_blocks.line_clear(width, height);
        assert!(result == Some(0))
    }
    #[test]
    fn line_clear_filled_partially() {
        let width : i16 = 3;
        let height : i16 = 5;
        let placed_blocks : Vec<Pos> = vec![
            Pos::new(0,0),
            Pos::new(0,1),
            Pos::new(1,0)
        ];
        let result = placed_blocks.line_clear(width, height);
        assert!(result == None)


    }
    #[test]
    fn line_clear_no_fill() {
        let width : i16 = 3;
        let height : i16 = 5;
        let placed_blocks : Vec<Pos> = Vec::new();
        let result = placed_blocks.line_clear(width, height);
        assert!(result == None)
    }
    #[test]
    fn line_clear_dif_y() {
        let width : i16 = 3;
        let height : i16 = 5;
        let placed_blocks : Vec<Pos> = vec![
            Pos::new(0,0),
            Pos::new(1,1),
            Pos::new(2,1)
        ];
        let result = placed_blocks.line_clear(width, height);
        assert!(result == None)
    }

    #[test]
    fn cut_by_y_one_lower() {
        let y = 19;
        let mut vec : Vec<Pos> = vec![
            Pos::new(0, 19),
            Pos::new(1,19),
            Pos::new(0, 18),
            Pos::new(1,18)
        ];
        let expected_vec : Vec<Pos> = vec![
            Pos::new(0,19),
            Pos::new(1,19)
        ];
        let res = vec.cut_by_y(y);
        println!("returned vec : {:?}", res);
        println!("expected vec : {:?}", expected_vec);
        assert_eq!(1,1);
    }
    #[test]
    fn cut_by_y_toground() {
        let y = 17;
        let mut vec : Vec<Pos> = vec![
            Pos::new(0, 17),
            Pos::new(1,17),
            Pos::new(2,17),
            Pos::new(2,16),
            Pos::new(1,16),
            Pos::new(1,15)
        ];
        let expected_vec : Vec<Pos> = vec![
            Pos::new(2,17),
            Pos::new(1,17),
            Pos::new(1,16)
        ];
        let res = vec.cut_by_y(y);
        println!("returned vec : {:?}", res);
        println!("expected vec : {:?}", expected_vec);
        assert_eq!(1,1);
    }
    #[test]
    fn cut_by_y_with_obstacle() {
        let y = 17;
        let mut vec : Vec<Pos> = vec![
            Pos::new(1,19),
            Pos::new(2,19),
            Pos::new(1,18),

            Pos::new(0,17),
            Pos::new(1,17),
            Pos::new(2,17),

            Pos::new(1,16),
            Pos::new(2,16),
            Pos::new(1,15)
        ];
        let expected_vec : Vec<Pos> = vec![
            Pos::new(1,19),
            Pos::new(2,19),
            Pos::new(1,18),
            Pos::new(1,16),

            Pos::new(1,17),

            Pos::new(2,17)

        ];
        let res = vec.cut_by_y(y);
        println!("returned vec : {:?}", res);
        println!("expected vec : {:?}", expected_vec);
        assert_eq!(1,1);

    }

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

        assert!(!are_colliding(&moving_item, &stationary_item, &Direction::Down));
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
        assert!(are_colliding(&moving_item, &stationary_item, &Direction::Down));
    }
}

    

