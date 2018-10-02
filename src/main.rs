fn main() {
    println!("Hello, world!");
}

fn validate_game(game: &Vec<(u16, u16)>) -> Result<(), ()> {
    let game_len = game.len();

    if game_len < 10 {
        println!("Game incomplete!, expected 10 frames, found {}", game_len);
        return Err(());
    }

    if game_len > 11 {
        println!(
            "Game over-complete!, expected 10 frames, found {}",
            game_len
        );
        return Err(());
    }
    return Ok(());
}

fn validate_frame(frame: &(u16, u16)) -> Result<u16, (())> {
    let pins = frame.0 + frame.1;
    if pins > 10 {
        return Err(());
    } else {
        return Ok(pins);
    }
}

fn is_strike(frame: &(u16, u16)) -> bool {
    return frame.0 == 10;
}

fn is_spare(frame: &(u16, u16)) -> bool {
    return frame.0 + frame.1 == 10;
}

fn score_game(game: &Vec<(u16, u16)>) -> u16 {
    let mut result = vec![0; 13];
    let mut score = 0;

    match validate_game(&game) {
        Ok(()) => println!("seems legit"),
        Err(()) => return 0,
    }

    // Calculate open frames (naive score)
    for (i, frame) in game.iter().enumerate() {
        result[i] = match validate_frame(&frame) {
            Ok(frame_score) => frame_score,
            Err(()) => {
                return 0;
            }
        };
    }
    // Correct for spares
    for (i, frame) in game.iter().enumerate() {
        if is_spare(&frame) {
            let bonus = game.get(i + 1).unwrap();
            result[i] += bonus.0;
        }
        if is_strike(&frame) {
            let bonus = game.get(i + 1).unwrap();
            result[i] += bonus.0 + bonus.1;
        }
    }

    // // Correct for strikes
    // for (i, frame) in game.iter().enumerate() {
    //
    // }
    //
    // tally up final score
    for frame_score in result {
        score += frame_score;
    }
    return score;
}

#[cfg(test)]
mod test {
    use score_game;
    #[test]
    fn all_gutterballs() {
        let game = vec![
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0),
        ];
        assert_eq!(score_game(&game), 0)
    }

    #[test]
    fn invalid_frame1() {
        let game = vec![
            (10, 1),
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0),
        ];
        assert_eq!(score_game(&game), 0)
    }

    #[test]
    fn all_open_frames() {
        let game = vec![
            (1, 1),
            (1, 1),
            (1, 1),
            (1, 1),
            (1, 1),
            (1, 1),
            (1, 1),
            (1, 1),
            (1, 1),
            (1, 1),
        ];
        assert_eq!(score_game(&game), 20)
    }

    #[test]
    fn spare_test1() {
        let game = vec![
            (5, 5),
            (5, 0),
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0),
        ];
        assert_eq!(score_game(&game), 20)
    }

    #[test]
    fn spare_test2() {
        let game = vec![
            (5, 5),
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0),
        ];
        assert_eq!(score_game(&game), 10)
    }

    #[test]
    fn strike_test1() {
        let game = vec![
            (10, 0),
            (5, 0),
            (5, 0),
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0),
        ];
        assert_eq!(score_game(&game), 30)
    }

    #[test]
    fn strike_test2() {
        let game = vec![
            (10, 0),
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0),
        ];
        assert_eq!(score_game(&game), 10)
    }

    #[test]
    fn perfect_game() {
        let game = vec![
            (10, 0),
            (10, 0),
            (10, 0),
            (10, 0),
            (10, 0),
            (10, 0),
            (10, 0),
            (10, 0),
            (10, 0),
            (10, 0),
            (10, 0),
        ];
        assert_eq!(score_game(&game), 300)
    }

    #[test]
    fn example_game() {
        let game = vec![
            (10, 0),
            (7, 3),
            (7, 2),
            (9, 1),
            (10, 0),
            (10, 0),
            (10, 0),
            (2, 3),
            (6, 4),
            (7, 3),
            (3, 0),
        ];
        assert_eq!(score_game(&game), 168)
    }
}
