#[macro_use]
extern crate quick_error;

fn main() {
    println!("Hello, world!");
}

use std::result::Result as StdResult;
quick_error! {
    #[derive(Debug, PartialEq)]
    pub enum BowlingError{
        InvalidFrameScore{description("Impossible frame score detected")}
        TooManyFrames{description("Too many frames detected")}
        TooFewFrames{description("Too few frames detected")}
        MiscError{description("Misc bad thing")}
    }
}
type Result<T> = StdResult<T, BowlingError>;

#[derive(Clone, Copy, Debug)]
struct Frame {
    roll1: u8,
    roll2: Option<u8>,
    fill_ball: Option<u8>,
}

impl Default for Frame {
    fn default() -> Frame {
        Frame {
            roll1: 0,
            roll2: Some(0),
            fill_ball: None,
        }
    }
}

fn validate_game(num_frames: usize, game: &Vec<(Frame)>) -> Result<()> {
    match game.len() {
        game_len if game_len == num_frames => Ok(()),
        game_len if game_len > num_frames => Err(BowlingError::TooManyFrames),
        _ => Err(BowlingError::TooFewFrames),
    }
}

fn validate_frame(is_last_frame: bool, frame: &Frame) -> Result<u16> {
    let pins = match frame.roll2 {
        Some(roll) => u16::from(frame.roll1 + roll),
        None => u16::from(frame.roll1),
    };

    return if pins > 20 && is_last_frame {
        Err(BowlingError::InvalidFrameScore)
    } else if pins > 10 && !is_last_frame {
        Err(BowlingError::InvalidFrameScore)
    } else {
        if is_strike(&frame) || is_spare(&frame) {
            Ok(10)
        } else {
            Ok(pins)
        }
    };
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

fn score_game(num_frames: usize, game: &Vec<Frame>) -> Result<u16> {
    let mut result = vec![0; num_frames];
    let mut score = 0;

    validate_game(num_frames, &game)?;

    // Calculate open frames (naive score)
    println!("Calculating Naive Score:");
    for (i, frame) in game.iter().enumerate() {
        result[i] = validate_frame(i == (num_frames - 1), &frame)?;
        println!("Frame[{}]\tScore[{}]", i + 1, result[i]);
    }

    // Correct for spares and strikes
    println!("\nApplying Bonuses:");
    for (i, frame) in game.iter().enumerate() {
        if is_spare(&frame) {
            if i < num_frames - 1 {
                let bonus = match game.get(i + 1) {
                    Some(bonus_frame) => bonus_frame,
                    None => panic!("Illegal frame requested!"),
                };
                println!("Frame Bonus(spare)[{}] + {}", i + 1, bonus.roll1);
                result[i] += u16::from(bonus.roll1);
            } else {
                // if the tenth frame
                // first check for additional rolls this frame
                let bonus = match frame.roll2 {
                    Some(roll) => roll,
                    None => 0,
                };
                println!("Frame Bonus(spare)[{}] + {}", i + 1, bonus);
                result[i] += u16::from(bonus);
            }
        }

        if is_strike(&frame) {
            if i < num_frames - 1 {
                let bonus = match game.get(i + 1) {
                    Some(bonus_frame) => bonus_frame,
                    None => panic!("Illegal frame requested!"),
                };

                match bonus.roll2 {
                    Some(roll2) => {
                        println!("Frame Bonus(strike)[{}] + {}", i + 1, bonus.roll1 + roll2);
                        result[i] += u16::from(bonus.roll1 + roll2);
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
                        result[i] += u16::from(bonus.roll1 + extra_bonus.roll1);
                    }
                }
            } else {
                // if the tenth frame
                // first check for additional rolls this frame
                let extra_roll = match frame.roll2 {
                    Some(roll) => roll,
                    None => 0,
                };

                let fill_ball = match frame.fill_ball {
                    Some(roll) => roll,
                    None => panic!("Illegal frame requested!"),
                };
                println!(
                    "Frame Bonus(strike--10th bonus)[{}] + {}",
                    i + 1,
                    extra_roll + fill_ball
                );
                result[i] += u16::from(extra_roll + fill_ball);
            }
        }
    }

    // tally up final score
    println!("\nFinal Score:");
    for (i, frame_score) in result.iter().enumerate() {
        score += frame_score;
        println!("[{}]  {}", i + 1, score);
    }
    return Ok(score);
}

#[cfg(test)]
mod test {
    use score_game;
    use BowlingError;
    use Frame;

    #[test]
    fn all_gutterballs() {
        let game = vec![Frame::default(); 10];
        assert_eq!(score_game(10, &game), Ok(0))
    }

    #[test]
    fn too_many_frames() {
        let game = vec![Frame::default(); 10];
        assert_eq!(score_game(2, &game), Err(BowlingError::TooManyFrames))
    }

    #[test]
    fn too_few_frames() {
        let game = vec![Frame::default(); 2];
        assert_eq!(score_game(10, &game), Err(BowlingError::TooFewFrames))
    }

    #[test]
    fn invalid_frame1() {
        let game = vec![
            (Frame {
                roll1: 10,
                roll2: Some(1),
                ..Default::default()
            }),
            (Frame {
                ..Default::default()
            }),
        ];
        assert_eq!(score_game(2, &game), Err(BowlingError::InvalidFrameScore))
    }

    #[test]
    fn invalid_frame2() {
        let game = vec![
            (Frame {
                roll1: 6,
                roll2: Some(5),
                ..Default::default()
            }),
            (Frame {
                ..Default::default()
            }),
        ];
        assert_eq!(score_game(2, &game), Err(BowlingError::InvalidFrameScore))
    }

    #[test]
    fn all_open_frames() {
        let game = vec![
            (Frame {
                roll1: 1,
                roll2: Some(1),
                ..Default::default()
            }),
            (Frame {
                roll1: 1,
                roll2: Some(1),
                ..Default::default()
            }),
            (Frame {
                roll1: 1,
                roll2: Some(1),
                ..Default::default()
            }),
            (Frame {
                roll1: 1,
                roll2: Some(1),
                ..Default::default()
            }),
            (Frame {
                roll1: 1,
                roll2: Some(1),
                ..Default::default()
            }),
            (Frame {
                roll1: 1,
                roll2: Some(1),
                ..Default::default()
            }),
            (Frame {
                roll1: 1,
                roll2: Some(1),
                ..Default::default()
            }),
            (Frame {
                roll1: 1,
                roll2: Some(1),
                ..Default::default()
            }),
            (Frame {
                roll1: 1,
                roll2: Some(1),
                ..Default::default()
            }),
            (Frame {
                roll1: 1,
                roll2: Some(1),
                ..Default::default()
            }),
        ];
        assert_eq!(score_game(10, &game), Ok(20))
    }

    #[test]
    fn spare_test1() {
        let game = vec![
            (Frame {
                roll1: 5,
                roll2: Some(5),
                ..Default::default()
            }),
            (Frame {
                roll1: 5,
                roll2: None,
                ..Default::default()
            }),
            (Frame {
                ..Default::default()
            }),
            (Frame {
                ..Default::default()
            }),
            (Frame {
                ..Default::default()
            }),
            (Frame {
                ..Default::default()
            }),
            (Frame {
                ..Default::default()
            }),
            (Frame {
                ..Default::default()
            }),
            (Frame {
                ..Default::default()
            }),
            (Frame {
                ..Default::default()
            }),
        ];
        assert_eq!(score_game(10, &game), Ok(20))
    }

    #[test]
    fn spare_test2() {
        let game = vec![
            (Frame {
                roll1: 5,
                roll2: Some(5),
                ..Default::default()
            }),
            (Frame {
                ..Default::default()
            }),
            (Frame {
                ..Default::default()
            }),
            (Frame {
                ..Default::default()
            }),
            (Frame {
                ..Default::default()
            }),
            (Frame {
                ..Default::default()
            }),
            (Frame {
                ..Default::default()
            }),
            (Frame {
                ..Default::default()
            }),
            (Frame {
                ..Default::default()
            }),
            (Frame {
                ..Default::default()
            }),
        ];
        assert_eq!(score_game(10, &game), Ok(10))
    }

    #[test]
    fn strike_test1() {
        let game = vec![
            (Frame {
                roll1: 10,
                roll2: None,
                ..Default::default()
            }),
            (Frame {
                roll1: 10,
                roll2: None,
                ..Default::default()
            }),
            (Frame {
                roll1: 5,
                roll2: Some(0),
                ..Default::default()
            }),
            (Frame {
                ..Default::default()
            }),
            (Frame {
                ..Default::default()
            }),
            (Frame {
                ..Default::default()
            }),
            (Frame {
                ..Default::default()
            }),
            (Frame {
                ..Default::default()
            }),
            (Frame {
                ..Default::default()
            }),
            (Frame {
                ..Default::default()
            }),
        ];
        assert_eq!(score_game(10, &game), Ok(25 + 15 + 5))
    }

    #[test]
    fn strike_test2() {
        let game = vec![
            (Frame {
                roll1: 10,
                roll2: None,
                ..Default::default()
            }),
            (Frame {
                ..Default::default()
            }),
            (Frame {
                ..Default::default()
            }),
            (Frame {
                ..Default::default()
            }),
            (Frame {
                ..Default::default()
            }),
            (Frame {
                ..Default::default()
            }),
            (Frame {
                ..Default::default()
            }),
            (Frame {
                ..Default::default()
            }),
            (Frame {
                ..Default::default()
            }),
            (Frame {
                ..Default::default()
            }),
        ];
        assert_eq!(score_game(10, &game), Ok(10))
    }

    #[test]
    fn perfect_game() {
        let game = vec![
            (Frame {
                roll1: 10,
                roll2: None,
                ..Default::default()
            }),
            (Frame {
                roll1: 10,
                roll2: None,
                ..Default::default()
            }),
            (Frame {
                roll1: 10,
                roll2: None,
                ..Default::default()
            }),
            (Frame {
                roll1: 10,
                roll2: None,
                ..Default::default()
            }),
            (Frame {
                roll1: 10,
                roll2: None,
                ..Default::default()
            }),
            (Frame {
                roll1: 10,
                roll2: None,
                ..Default::default()
            }),
            (Frame {
                roll1: 10,
                roll2: None,
                ..Default::default()
            }),
            (Frame {
                roll1: 10,
                roll2: None,
                ..Default::default()
            }),
            (Frame {
                roll1: 10,
                roll2: None,
                ..Default::default()
            }),
            (Frame {
                roll1: 10,
                roll2: Some(10),
                fill_ball: Some(10),
            }),
        ];
        assert_eq!(score_game(10, &game), Ok(300))
    }

    #[test]
    fn perfect_game_5_frame() {
        let game = vec![
            (Frame {
                roll1: 10,
                roll2: None,
                ..Default::default()
            }),
            (Frame {
                roll1: 10,
                roll2: None,
                ..Default::default()
            }),
            (Frame {
                roll1: 10,
                roll2: None,
                ..Default::default()
            }),
            (Frame {
                roll1: 10,
                roll2: None,
                ..Default::default()
            }),
            (Frame {
                roll1: 10,
                roll2: Some(10),
                fill_ball: Some(10),
            }),
        ];
        assert_eq!(score_game(5, &game), Ok(150))
    }

    #[test]
    fn example_game1() {
        let game = vec![
            (Frame {
                roll1: 10,
                roll2: None,
                ..Default::default()
            }),
            (Frame {
                roll1: 7,
                roll2: Some(3),
                ..Default::default()
            }),
            (Frame {
                roll1: 7,
                roll2: Some(2),
                ..Default::default()
            }),
            (Frame {
                roll1: 9,
                roll2: Some(1),
                ..Default::default()
            }),
            (Frame {
                roll1: 10,
                roll2: None,
                ..Default::default()
            }),
            (Frame {
                roll1: 10,
                roll2: None,
                ..Default::default()
            }),
            (Frame {
                roll1: 10,
                roll2: None,
                ..Default::default()
            }),
            (Frame {
                roll1: 2,
                roll2: Some(3),
                ..Default::default()
            }),
            (Frame {
                roll1: 6,
                roll2: Some(4),
                ..Default::default()
            }),
            (Frame {
                roll1: 7,
                roll2: Some(3),
                fill_ball: Some(3),
            }),
        ];
        assert_eq!(score_game(10, &game), Ok(168))
    }

    #[test]
    fn example_game2() {
        let game = vec![
            (Frame {
                roll1: 10,
                roll2: None,
                ..Default::default()
            }),
            (Frame {
                roll1: 7,
                roll2: Some(3),
                ..Default::default()
            }),
            (Frame {
                roll1: 9,
                roll2: Some(0),
                ..Default::default()
            }),
            (Frame {
                roll1: 10,
                roll2: None,
                ..Default::default()
            }),
            (Frame {
                roll1: 0,
                roll2: Some(8),
                ..Default::default()
            }),
            (Frame {
                roll1: 8,
                roll2: Some(2),
                ..Default::default()
            }),
            (Frame {
                roll1: 0,
                roll2: Some(6),
                ..Default::default()
            }),
            (Frame {
                roll1: 10,
                roll2: None,
                ..Default::default()
            }),
            (Frame {
                roll1: 10,
                roll2: None,
                ..Default::default()
            }),
            (Frame {
                roll1: 10,
                roll2: Some(8),
                fill_ball: Some(1),
            }),
        ];
        assert_eq!(score_game(10, &game), Ok(167))
    }
}
