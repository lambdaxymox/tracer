extern crate tracer;


#[cfg(test)]
mod canvas_tests {
    use tracer::*;


    #[test]
    fn test_read_write() {
        let width = 20;
        let height = 10;
        
        let expected = vec![
            vec![Rgba::new(1, 1, 1); width],
            vec![Rgba::new(2, 2, 2); width],
            vec![Rgba::new(3, 3, 3); width],
            vec![Rgba::new(4, 4, 4); width],
            vec![Rgba::new(5, 5, 5); width],
            vec![Rgba::new(6, 6, 6); width],
            vec![Rgba::new(7, 7, 7); width],
            vec![Rgba::new(8, 8, 8); width],
            vec![Rgba::new(9, 9, 9); width],
            vec![Rgba::new(10, 10, 10); width]
        ];
       
        let mut result = Canvas::new(width, height);
        for row in 0..height {
            for column in 0..width {
                result[row][column] = expected[row][column];
            }
        }

        for row in 0..result.height {
            for column in 0..result.width {
                assert_eq!(result[row][column], expected[row][column]);
            }
        }
    }

    #[test]
    fn test_clear() {
        let width = 720;
        let height = 480;
        let zero = Rgba::zero();
        let one = Rgba::new(1, 1, 1);
        let mut result = Canvas::new(width, height);
        for row in 0..height {
            for column in 0..width {
                result[row][column] = one; 
            }
        }

        result.clear();

        for row in 0..result.height {
            for column in 0..result.width {
                assert_eq!(result[row][column], zero);
            }
        }
    }
}
