fn main() {
    println!("Hello, world!");
}

struct Frame {
    roll1: u8,
    roll2: Option<u8>,
}

fn validate_game(game: &Vec<(Frame)>) -> Result<(), ()> {
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

fn validate_frame(frame: &Frame) -> u16 {
    if is_spare(frame) || is_strike(frame) {
        return 10;
    } else {
        let pins = match frame.roll2 {
            Some(roll) => (frame.roll1 + roll) as u16,
            None => frame.roll1 as u16,
        };

        if pins > 10 {
            panic!("Invalid frame score!");
        } else {
            return pins;
        }
    }
}

fn is_strike(frame: &Frame) -> bool {
    return frame.roll1 == 10;
}

fn is_spare(frame: &Frame) -> bool {
    if frame.roll1 == 10 {
        return false;
    }
    match frame.roll2 {
        Some(roll2) => return (frame.roll1 + roll2 == 10) && (frame.roll1 != 10),
        None => return false,
    };
}

fn score_game(game: &Vec<Frame>) -> u16 {
    let mut result = vec![0; 11];
    let mut score = 0;

    match validate_game(&game) {
        Ok(()) => println!("seems legit"),
        Err(()) => return 0,
    }

    // Calculate open frames (naive score)
    println!("Calculating Naive Score:");
    for (i, frame) in game.iter().enumerate() {
        if i >= 10 {
            break;
        }
        result[i] = match validate_frame(&frame) {
            frame_score => {
                println!("Frame[{}]\tScore[{}]", i + 1, frame_score);
                frame_score
            }
        };
    }
    // Correct for spares and strikes
    println!("\nApplying Bonuses:");
    for (i, frame) in game.iter().enumerate() {
        if i >= 10 {
            break;
        }
        if is_spare(&frame) {
            let bonus = match game.get(i + 1) {
                Some(bonus_frame) => bonus_frame,
                None => panic!("Illegal frame requested!"),
            };
            println!("Frame Bonus(spare)[{}] + {}", i + 1, bonus.roll1);
            result[i] += bonus.roll1 as u16;
        }

        if is_strike(&frame) {
            if i < 9 {
                let bonus = match game.get(i + 1) {
                    Some(bonus_frame) => bonus_frame,
                    None => panic!("Illegal frame requested!"),
                };

                match bonus.roll2 {
                    Some(roll2) => {
                        println!("Frame Bonus(strike)[{}] + {}", i + 1, bonus.roll1 + roll2);
                        result[i] += (bonus.roll1 + roll2) as u16;
                    }
                    None => {
                        let extra_bonus = match game.get(i + 2) {
                            Some(bonus_frame) => bonus_frame,
                            None => panic!("Illegal frame requested!"),
                        };
                        println!(
                            "Frame Bonus(strike--extra bonus)[{}] + {}",
                            i + 1,
                            bonus.roll1 + extra_bonus.roll1
                        );
                        result[i] += (bonus.roll1 + extra_bonus.roll1) as u16;
                    }
                }
            } else {
                // if the tenth frame
                // first check for additional rolls this frame
                let extra_roll = match frame.roll2 {
                    Some(roll) => roll,
                    None => 0,
                };

                let fill_ball = match game.get(i + 1) {
                    Some(bonus_frame) => bonus_frame.roll1,
                    None => panic!("Illegal frame requested!"),
                };
                println!(
                    "Frame Bonus(strike--10th bonus)[{}] + {}",
                    i + 1,
                    extra_roll + fill_ball
                );
                result[i] += (extra_roll + fill_ball) as u16;
            }
        }
    }

    // tally up final score
    println!("\nFinal Score:");
    for (i, frame_score) in result.iter().enumerate() {
        score += frame_score;
        println!("[{}]  {}", i + 1, score);
    }
    return score;
}

#[cfg(test)]
mod test {
    use score_game;
    use Frame;

    #[test]
    fn all_gutterballs() {
        let game = vec![
            (Frame {
                roll1: 0,
                roll2: None,
            }),
            (Frame {
                roll1: 0,
                roll2: None,
            }),
            (Frame {
                roll1: 0,
                roll2: None,
            }),
            (Frame {
                roll1: 0,
                roll2: None,
            }),
            (Frame {
                roll1: 0,
                roll2: None,
            }),
            (Frame {
                roll1: 0,
                roll2: None,
            }),
            (Frame {
                roll1: 0,
                roll2: None,
            }),
            (Frame {
                roll1: 0,
                roll2: None,
            }),
            (Frame {
                roll1: 0,
                roll2: None,
            }),
            (Frame {
                roll1: 0,
                roll2: None,
            }),
        ];
        assert_eq!(score_game(&game), 0)
    }

    #[test]
    #[should_panic]
    fn invalid_frame1() {
        let game = vec![
            (Frame {
                roll1: 10,
                roll2: Some(1),
            }),
            (Frame {
                roll1: 0,
                roll2: None,
            }),
            (Frame {
                roll1: 0,
                roll2: None,
            }),
            (Frame {
                roll1: 0,
                roll2: None,
            }),
            (Frame {
                roll1: 0,
                roll2: None,
            }),
            (Frame {
                roll1: 0,
                roll2: None,
            }),
            (Frame {
                roll1: 0,
                roll2: None,
            }),
            (Frame {
                roll1: 0,
                roll2: None,
            }),
            (Frame {
                roll1: 0,
                roll2: None,
            }),
            (Frame {
                roll1: 0,
                roll2: None,
            }),
        ];
        assert_eq!(score_game(&game), 0)
    }

    #[test]
    fn all_open_frames() {
        let game = vec![
            (Frame {
                roll1: 1,
                roll2: Some(1),
            }),
            (Frame {
                roll1: 1,
                roll2: Some(1),
            }),
            (Frame {
                roll1: 1,
                roll2: Some(1),
            }),
            (Frame {
                roll1: 1,
                roll2: Some(1),
            }),
            (Frame {
                roll1: 1,
                roll2: Some(1),
            }),
            (Frame {
                roll1: 1,
                roll2: Some(1),
            }),
            (Frame {
                roll1: 1,
                roll2: Some(1),
            }),
            (Frame {
                roll1: 1,
                roll2: Some(1),
            }),
            (Frame {
                roll1: 1,
                roll2: Some(1),
            }),
            (Frame {
                roll1: 1,
                roll2: Some(1),
            }),
        ];
        assert_eq!(score_game(&game), 20)
    }

    #[test]
    fn spare_test1() {
        let game = vec![
            (Frame {
                roll1: 5,
                roll2: Some(5),
            }),
            (Frame {
                roll1: 5,
                roll2: None,
            }),
            (Frame {
                roll1: 0,
                roll2: None,
            }),
            (Frame {
                roll1: 0,
                roll2: None,
            }),
            (Frame {
                roll1: 0,
                roll2: None,
            }),
            (Frame {
                roll1: 0,
                roll2: None,
            }),
            (Frame {
                roll1: 0,
                roll2: None,
            }),
            (Frame {
                roll1: 0,
                roll2: None,
            }),
            (Frame {
                roll1: 0,
                roll2: None,
            }),
            (Frame {
                roll1: 0,
                roll2: None,
            }),
        ];
        assert_eq!(score_game(&game), 20)
    }

    #[test]
    fn spare_test2() {
        let game = vec![
            (Frame {
                roll1: 5,
                roll2: Some(5),
            }),
            (Frame {
                roll1: 0,
                roll2: None,
            }),
            (Frame {
                roll1: 0,
                roll2: None,
            }),
            (Frame {
                roll1: 0,
                roll2: None,
            }),
            (Frame {
                roll1: 0,
                roll2: None,
            }),
            (Frame {
                roll1: 0,
                roll2: None,
            }),
            (Frame {
                roll1: 0,
                roll2: None,
            }),
            (Frame {
                roll1: 0,
                roll2: None,
            }),
            (Frame {
                roll1: 0,
                roll2: None,
            }),
            (Frame {
                roll1: 0,
                roll2: None,
            }),
        ];
        assert_eq!(score_game(&game), 10)
    }

    #[test]
    fn strike_test1() {
        let game = vec![
            (Frame {
                roll1: 10,
                roll2: None,
            }),
            (Frame {
                roll1: 10,
                roll2: None,
            }),
            (Frame {
                roll1: 5,
                roll2: Some(0),
            }),
            (Frame {
                roll1: 0,
                roll2: Some(0),
            }),
            (Frame {
                roll1: 0,
                roll2: Some(0),
            }),
            (Frame {
                roll1: 0,
                roll2: Some(0),
            }),
            (Frame {
                roll1: 0,
                roll2: Some(0),
            }),
            (Frame {
                roll1: 0,
                roll2: Some(0),
            }),
            (Frame {
                roll1: 0,
                roll2: Some(0),
            }),
            (Frame {
                roll1: 0,
                roll2: Some(0),
            }),
        ];
        assert_eq!(score_game(&game), 25 + 15 + 5)
    }

    #[test]
    fn strike_test2() {
        let game = vec![
            (Frame {
                roll1: 10,
                roll2: None,
            }),
            (Frame {
                roll1: 0,
                roll2: Some(0),
            }),
            (Frame {
                roll1: 0,
                roll2: Some(0),
            }),
            (Frame {
                roll1: 0,
                roll2: Some(0),
            }),
            (Frame {
                roll1: 0,
                roll2: Some(0),
            }),
            (Frame {
                roll1: 0,
                roll2: Some(0),
            }),
            (Frame {
                roll1: 0,
                roll2: Some(0),
            }),
            (Frame {
                roll1: 0,
                roll2: Some(0),
            }),
            (Frame {
                roll1: 0,
                roll2: Some(0),
            }),
            (Frame {
                roll1: 0,
                roll2: Some(0),
            }),
        ];
        assert_eq!(score_game(&game), 10)
    }

    #[test]
    fn perfect_game() {
        let game = vec![
            (Frame {
                roll1: 10,
                roll2: None,
            }),
            (Frame {
                roll1: 10,
                roll2: None,
            }),
            (Frame {
                roll1: 10,
                roll2: None,
            }),
            (Frame {
                roll1: 10,
                roll2: None,
            }),
            (Frame {
                roll1: 10,
                roll2: None,
            }),
            (Frame {
                roll1: 10,
                roll2: None,
            }),
            (Frame {
                roll1: 10,
                roll2: None,
            }),
            (Frame {
                roll1: 10,
                roll2: None,
            }),
            (Frame {
                roll1: 10,
                roll2: None,
            }),
            (Frame {
                roll1: 10,
                roll2: Some(10),
            }),
            (Frame {
                roll1: 10,
                roll2: None,
            }),
        ];
        assert_eq!(score_game(&game), 300)
    }

    #[test]
    fn example_game1() {
        let game = vec![
            (Frame {
                roll1: 10,
                roll2: None,
            }),
            (Frame {
                roll1: 7,
                roll2: Some(3),
            }),
            (Frame {
                roll1: 7,
                roll2: Some(2),
            }),
            (Frame {
                roll1: 9,
                roll2: Some(1),
            }),
            (Frame {
                roll1: 10,
                roll2: None,
            }),
            (Frame {
                roll1: 10,
                roll2: None,
            }),
            (Frame {
                roll1: 10,
                roll2: None,
            }),
            (Frame {
                roll1: 2,
                roll2: Some(3),
            }),
            (Frame {
                roll1: 6,
                roll2: Some(4),
            }),
            (Frame {
                roll1: 7,
                roll2: Some(3),
            }),
            (Frame {
                roll1: 3,
                roll2: Some(0),
            }),
        ];
        assert_eq!(score_game(&game), 168)
    }

    #[test]
    fn example_game2() {
        let game = vec![
            (Frame {
                roll1: 10,
                roll2: None,
            }),
            (Frame {
                roll1: 7,
                roll2: Some(3),
            }),
            (Frame {
                roll1: 9,
                roll2: Some(0),
            }),
            (Frame {
                roll1: 10,
                roll2: None,
            }),
            (Frame {
                roll1: 0,
                roll2: Some(8),
            }),
            (Frame {
                roll1: 8,
                roll2: Some(2),
            }),
            (Frame {
                roll1: 0,
                roll2: Some(6),
            }),
            (Frame {
                roll1: 10,
                roll2: None,
            }),
            (Frame {
                roll1: 10,
                roll2: None,
            }),
            (Frame {
                roll1: 10,
                roll2: Some(8),
            }),
            (Frame {
                roll1: 1,
                roll2: Some(0),
            }),
        ];
        assert_eq!(score_game(&game), 167)
    }
}
