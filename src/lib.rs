pub mod rules;

const TOTAL_TRICKS: i16 = 13;

pub struct GameSet {
    pub gamemode: GameMode,
    pub params: ModeParams,
}

pub enum GameMode {
    Emballage,
    Seul,
    Misere,
}

pub struct ModeParams {
    pub player_number: usize,
    pub tricks_to_win: i16,
    pub max_tricks_allowed: Option<i16>,
    pub min_points: i16,
    pub points_per_suppl_trick: i16,
}

impl ModeParams {
    pub fn calculate_points(&self, mut tricks: i16) -> i16 {
        let mut double_points = false;
        if let Some(max_tricks) = self.max_tricks_allowed {
            tricks = tricks.clamp(0, max_tricks);
        } else if tricks == TOTAL_TRICKS {
            double_points = true;
        }

        let suppl_tricks = tricks - self.tricks_to_win;
        let points = self.min_points + suppl_tricks.abs() * self.points_per_suppl_trick;
        match suppl_tricks {
            0.. if double_points => 2 * (points - self.points_per_suppl_trick),
            0.. => points,
            _ => -2 * points,
        }
    }

    pub fn adjust_tricks_with_auction(&self, bid: usize) {
        todo!()
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    const EMBALLAGE: ModeParams = ModeParams {
        player_number: 2,
        tricks_to_win: 8,
        min_points: 2,
        points_per_suppl_trick: 1,
        max_tricks_allowed: None,
    };

    const SEUL: ModeParams = ModeParams {
        player_number: 1,
        tricks_to_win: 6,
        min_points: 6,
        points_per_suppl_trick: 3,
        max_tricks_allowed: Some(8),
    };

    const MISERE: ModeParams = ModeParams {
        player_number: 1,
        tricks_to_win: 0,
        max_tricks_allowed: Some(1),
        min_points: 12,
        points_per_suppl_trick: -36,
    };

    const PICOLO: ModeParams = ModeParams {
        player_number: 1,
        tricks_to_win: 1,
        max_tricks_allowed: Some(2),
        min_points: 12,
        points_per_suppl_trick: -36,
    };

    #[test]
    fn emballage_win_case() {
        let tricks = 10;
        let expected_points = 4;

        assert_eq!(expected_points, EMBALLAGE.calculate_points(tricks));
    }

    #[test]
    fn seul_win_case() {
        let tricks = 8;
        let expected_points = 12;

        assert_eq!(expected_points, SEUL.calculate_points(tricks));
    }

    #[test]
    fn emballage_lose_case() {
        let tricks = 6;
        let expected_points = -8;

        assert_eq!(expected_points, EMBALLAGE.calculate_points(tricks));
    }

    #[test]
    fn seul_lose_case() {
        let tricks = 5;
        let expected_points = -18;

        assert_eq!(expected_points, SEUL.calculate_points(tricks));
    }

    #[test]
    fn capot() {
        let tricks = TOTAL_TRICKS;
        let expected_points = 12;

        assert_eq!(expected_points, EMBALLAGE.calculate_points(tricks))
    }

    #[test]
    fn misere() {
        let tricks = 0;
        let expected_points = 12;
        
        assert_eq!(expected_points, MISERE.calculate_points(tricks));
        
        let tricks = 1;
        let expected_points = -24;
        assert_eq!(expected_points, MISERE.calculate_points(tricks));

        let tricks = 2;
        let expected_points = -24;
        assert_eq!(expected_points, MISERE.calculate_points(tricks));
    }

    #[test]
    fn picolo() {
        let tricks = 1;
        let expected_points = 12;
        assert_eq!(expected_points, PICOLO.calculate_points(tricks));

        let tricks = 2;
        let expected_points = -24;
        assert_eq!(expected_points, PICOLO.calculate_points(tricks));

        let tricks = 0;
        let expected_points = -24;
        assert_eq!(expected_points, PICOLO.calculate_points(tricks));
    }
}
