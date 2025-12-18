use super::*;

pub struct Picolo {
    min_points: i16,
}

impl Score for Picolo {
    fn calculate_score(&self, tricks: i16) -> (i16, GameResult) {
        if tricks == 1 {
            return (self.min_points, GameResult::Win);
        }
        (self.min_points, GameResult::Lose)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PICOLO: Picolo = Picolo { min_points: 12 };
    
    #[test]
    fn win() {
        let tricks = 1;
        let expected_score = 12;

        assert_eq!(expected_score, PICOLO.get_score(tricks));
    }


    #[test]
    fn lose() {
        let tricks = 0;
        let expected_score = -24;
        
        assert_eq!(expected_score, PICOLO.get_score(tricks));
        
        let tricks = 3;
        assert_eq!(expected_score, PICOLO.get_score(tricks));
    }
}
