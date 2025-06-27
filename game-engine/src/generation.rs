use crate::{Card, GameState, Rank, Suit};

struct GameRngGenerator {
    state: u64,
}

impl GameRngGenerator {
    pub fn new(seed: u64) -> GameRngGenerator {
        GameRngGenerator { state: seed }
    }

    pub fn next_value(&mut self) -> u64 {
        self.state = (self.state * 214013 + 2531011) % (2 as u64).pow(31);
        return self.state / (2 as u64).pow(16);
    }
}

pub struct GameGenerator {
    rng: GameRngGenerator,
    pub game_state: GameState,
}

impl GameGenerator {
    pub fn new(seed: u64) -> Self {
        Self {
            rng: GameRngGenerator::new(seed),
            game_state: GameState::new(),
        }
    }

    pub fn generate(&mut self) {
        let mut cards: Vec<&Card> = vec![
            &Card {
                rank: Rank::Ace,
                suit: Suit::Clubs,
            },
            &Card {
                rank: Rank::Ace,
                suit: Suit::Diamonds,
            },
            &Card {
                rank: Rank::Ace,
                suit: Suit::Hearts,
            },
            &Card {
                rank: Rank::Ace,
                suit: Suit::Spades,
            },
            &Card {
                rank: Rank::Two,
                suit: Suit::Clubs,
            },
            &Card {
                rank: Rank::Two,
                suit: Suit::Diamonds,
            },
            &Card {
                rank: Rank::Two,
                suit: Suit::Hearts,
            },
            &Card {
                rank: Rank::Two,
                suit: Suit::Spades,
            },
            &Card {
                rank: Rank::Three,
                suit: Suit::Clubs,
            },
            &Card {
                rank: Rank::Three,
                suit: Suit::Diamonds,
            },
            &Card {
                rank: Rank::Three,
                suit: Suit::Hearts,
            },
            &Card {
                rank: Rank::Three,
                suit: Suit::Spades,
            },
            &Card {
                rank: Rank::Four,
                suit: Suit::Clubs,
            },
            &Card {
                rank: Rank::Four,
                suit: Suit::Diamonds,
            },
            &Card {
                rank: Rank::Four,
                suit: Suit::Hearts,
            },
            &Card {
                rank: Rank::Four,
                suit: Suit::Spades,
            },
            &Card {
                rank: Rank::Five,
                suit: Suit::Clubs,
            },
            &Card {
                rank: Rank::Five,
                suit: Suit::Diamonds,
            },
            &Card {
                rank: Rank::Five,
                suit: Suit::Hearts,
            },
            &Card {
                rank: Rank::Five,
                suit: Suit::Spades,
            },
            &Card {
                rank: Rank::Six,
                suit: Suit::Clubs,
            },
            &Card {
                rank: Rank::Six,
                suit: Suit::Diamonds,
            },
            &Card {
                rank: Rank::Six,
                suit: Suit::Hearts,
            },
            &Card {
                rank: Rank::Six,
                suit: Suit::Spades,
            },
            &Card {
                rank: Rank::Seven,
                suit: Suit::Clubs,
            },
            &Card {
                rank: Rank::Seven,
                suit: Suit::Diamonds,
            },
            &Card {
                rank: Rank::Seven,
                suit: Suit::Hearts,
            },
            &Card {
                rank: Rank::Seven,
                suit: Suit::Spades,
            },
            &Card {
                rank: Rank::Eight,
                suit: Suit::Clubs,
            },
            &Card {
                rank: Rank::Eight,
                suit: Suit::Diamonds,
            },
            &Card {
                rank: Rank::Eight,
                suit: Suit::Hearts,
            },
            &Card {
                rank: Rank::Eight,
                suit: Suit::Spades,
            },
            &Card {
                rank: Rank::Nine,
                suit: Suit::Clubs,
            },
            &Card {
                rank: Rank::Nine,
                suit: Suit::Diamonds,
            },
            &Card {
                rank: Rank::Nine,
                suit: Suit::Hearts,
            },
            &Card {
                rank: Rank::Nine,
                suit: Suit::Spades,
            },
            &Card {
                rank: Rank::Ten,
                suit: Suit::Clubs,
            },
            &Card {
                rank: Rank::Ten,
                suit: Suit::Diamonds,
            },
            &Card {
                rank: Rank::Ten,
                suit: Suit::Hearts,
            },
            &Card {
                rank: Rank::Ten,
                suit: Suit::Spades,
            },
            &Card {
                rank: Rank::Jack,
                suit: Suit::Clubs,
            },
            &Card {
                rank: Rank::Jack,
                suit: Suit::Diamonds,
            },
            &Card {
                rank: Rank::Jack,
                suit: Suit::Hearts,
            },
            &Card {
                rank: Rank::Jack,
                suit: Suit::Spades,
            },
            &Card {
                rank: Rank::Queen,
                suit: Suit::Clubs,
            },
            &Card {
                rank: Rank::Queen,
                suit: Suit::Diamonds,
            },
            &Card {
                rank: Rank::Queen,
                suit: Suit::Hearts,
            },
            &Card {
                rank: Rank::Queen,
                suit: Suit::Spades,
            },
            &Card {
                rank: Rank::King,
                suit: Suit::Clubs,
            },
            &Card {
                rank: Rank::King,
                suit: Suit::Diamonds,
            },
            &Card {
                rank: Rank::King,
                suit: Suit::Hearts,
            },
            &Card {
                rank: Rank::King,
                suit: Suit::Spades,
            },
        ];

        let mut column: usize = 0;
        let max_columns: usize = 8;

        while cards.len() > 0 {
            let next_rng_value: usize = self.rng.next_value() as usize;
            let next_index = next_rng_value % cards.len();
            let end_index = cards.len() - 1;
            let card_at_index = cards[next_index];
            let card_at_end = cards[end_index];

            cards[next_index] = card_at_end;
            cards[end_index] = card_at_index;

            let next_card = cards.pop().unwrap();

            self.game_state.tableau_mut().place_card(column, next_card.clone());

            column = (column + 1) % max_columns
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Card, Rank, Suit};

    #[test]
    fn test_game_1_layout() {
        let mut generator = GameGenerator::new(1);
        generator.generate();
        let game = generator.game_state;

        // Verify the layout matches the expected game #1 layout

        // Define expected cards for each column
        let expected_layout = [
            // Column 0
            vec![
                Card {
                    rank: Rank::Jack,
                    suit: Suit::Diamonds,
                }, // J♦
                Card {
                    rank: Rank::King,
                    suit: Suit::Diamonds,
                }, // K♦
                Card {
                    rank: Rank::Two,
                    suit: Suit::Spades,
                }, // 2♠
                Card {
                    rank: Rank::Four,
                    suit: Suit::Clubs,
                }, // 4♣
                Card {
                    rank: Rank::Three,
                    suit: Suit::Spades,
                }, // 3♠
                Card {
                    rank: Rank::Six,
                    suit: Suit::Diamonds,
                }, // 6♦
                Card {
                    rank: Rank::Six,
                    suit: Suit::Spades,
                }, // 6♠
            ],
            // Column 1
            vec![
                Card {
                    rank: Rank::Two,
                    suit: Suit::Diamonds,
                }, // 2♦
                Card {
                    rank: Rank::King,
                    suit: Suit::Clubs,
                }, // K♣
                Card {
                    rank: Rank::King,
                    suit: Suit::Spades,
                }, // K♠
                Card {
                    rank: Rank::Five,
                    suit: Suit::Clubs,
                }, // 5♣
                Card {
                    rank: Rank::Ten,
                    suit: Suit::Diamonds,
                }, // 10♦
                Card {
                    rank: Rank::Eight,
                    suit: Suit::Spades,
                }, // 8♠
                Card {
                    rank: Rank::Nine,
                    suit: Suit::Clubs,
                }, // 9♣
            ],
            // Column 2
            vec![
                Card {
                    rank: Rank::Nine,
                    suit: Suit::Hearts,
                }, // 9♥
                Card {
                    rank: Rank::Nine,
                    suit: Suit::Spades,
                }, // 9♠
                Card {
                    rank: Rank::Nine,
                    suit: Suit::Diamonds,
                }, // 9♦
                Card {
                    rank: Rank::Ten,
                    suit: Suit::Spades,
                }, // 10♠
                Card {
                    rank: Rank::Four,
                    suit: Suit::Spades,
                }, // 4♠
                Card {
                    rank: Rank::Eight,
                    suit: Suit::Diamonds,
                }, // 8♦
                Card {
                    rank: Rank::Two,
                    suit: Suit::Hearts,
                }, // 2♥
            ],
            // Column 3
            vec![
                Card {
                    rank: Rank::Jack,
                    suit: Suit::Clubs,
                }, // J♣
                Card {
                    rank: Rank::Five,
                    suit: Suit::Spades,
                }, // 5♠
                Card {
                    rank: Rank::Queen,
                    suit: Suit::Diamonds,
                }, // Q♦
                Card {
                    rank: Rank::Queen,
                    suit: Suit::Hearts,
                }, // Q♥
                Card {
                    rank: Rank::Ten,
                    suit: Suit::Hearts,
                }, // 10♥
                Card {
                    rank: Rank::Queen,
                    suit: Suit::Spades,
                }, // Q♠
                Card {
                    rank: Rank::Six,
                    suit: Suit::Hearts,
                }, // 6♥
            ],
            // Column 4
            vec![
                Card {
                    rank: Rank::Five,
                    suit: Suit::Diamonds,
                }, // 5♦
                Card {
                    rank: Rank::Ace,
                    suit: Suit::Diamonds,
                }, // A♦
                Card {
                    rank: Rank::Jack,
                    suit: Suit::Spades,
                }, // J♠
                Card {
                    rank: Rank::Four,
                    suit: Suit::Hearts,
                }, // 4♥
                Card {
                    rank: Rank::Eight,
                    suit: Suit::Hearts,
                }, // 8♥
                Card {
                    rank: Rank::Six,
                    suit: Suit::Clubs,
                }, // 6♣
            ],
            // Column 5
            vec![
                Card {
                    rank: Rank::Seven,
                    suit: Suit::Hearts,
                }, // 7♥
                Card {
                    rank: Rank::Queen,
                    suit: Suit::Clubs,
                }, // Q♣
                Card {
                    rank: Rank::Ace,
                    suit: Suit::Spades,
                }, // A♠
                Card {
                    rank: Rank::Ace,
                    suit: Suit::Clubs,
                }, // A♣
                Card {
                    rank: Rank::Two,
                    suit: Suit::Clubs,
                }, // 2♣
                Card {
                    rank: Rank::Three,
                    suit: Suit::Diamonds,
                }, // 3♦
            ],
            // Column 6
            vec![
                Card {
                    rank: Rank::Seven,
                    suit: Suit::Clubs,
                }, // 7♣
                Card {
                    rank: Rank::King,
                    suit: Suit::Hearts,
                }, // K♥
                Card {
                    rank: Rank::Ace,
                    suit: Suit::Hearts,
                }, // A♥
                Card {
                    rank: Rank::Four,
                    suit: Suit::Diamonds,
                }, // 4♦
                Card {
                    rank: Rank::Jack,
                    suit: Suit::Hearts,
                }, // J♥
                Card {
                    rank: Rank::Eight,
                    suit: Suit::Clubs,
                }, // 8♣
            ],
            // Column 7
            vec![
                Card {
                    rank: Rank::Five,
                    suit: Suit::Hearts,
                }, // 5♥
                Card {
                    rank: Rank::Three,
                    suit: Suit::Hearts,
                }, // 3♥
                Card {
                    rank: Rank::Three,
                    suit: Suit::Clubs,
                }, // 3♣
                Card {
                    rank: Rank::Seven,
                    suit: Suit::Spades,
                }, // 7♠
                Card {
                    rank: Rank::Seven,
                    suit: Suit::Diamonds,
                }, // 7♦
                Card {
                    rank: Rank::Ten,
                    suit: Suit::Clubs,
                }, // 10♣
            ],
        ];

        // Check that each column has the expected cards
        for (col_idx, expected_column) in expected_layout.iter().enumerate() {
            assert_eq!(
                game.tableau().column_length(col_idx),
                expected_column.len(),
                "Column {} has wrong number of cards",
                col_idx
            );

            for (card_idx, expected_card) in expected_column.iter().enumerate() {
                assert_eq!(
                    game.tableau().get_card_at(col_idx, card_idx).unwrap(),
                    expected_card,
                    "Mismatch at column {}, card {}",
                    col_idx,
                    card_idx
                );
            }
        }
    }

    #[test]
    fn test_additional_game_layouts() {
        // Test games known for being interesting
        let test_cases = [
            // Game #617
            (
                617,
                vec![
                    Card {
                        rank: Rank::Seven,
                        suit: Suit::Diamonds,
                    }, // 7♦
                    Card {
                        rank: Rank::Ten,
                        suit: Suit::Diamonds,
                    }, // 10♦
                    Card {
                        rank: Rank::Ten,
                        suit: Suit::Hearts,
                    }, // 10♥
                    Card {
                        rank: Rank::King,
                        suit: Suit::Diamonds,
                    }, // K♦
                    Card {
                        rank: Rank::Four,
                        suit: Suit::Clubs,
                    }, // 4♣
                    Card {
                        rank: Rank::Four,
                        suit: Suit::Spades,
                    }, // 4♠
                    Card {
                        rank: Rank::Jack,
                        suit: Suit::Diamonds,
                    }, // J♦
                ],
            ),
            // Game #11982 (famously unsolvable with standard FreeCell rules)
            (
                11982,
                vec![
                    Card {
                        rank: Rank::Ace,
                        suit: Suit::Hearts,
                    }, // A♥
                    Card {
                        rank: Rank::Three,
                        suit: Suit::Diamonds,
                    }, // 3♦
                    Card {
                        rank: Rank::King,
                        suit: Suit::Diamonds,
                    }, // K♦
                    Card {
                        rank: Rank::Jack,
                        suit: Suit::Clubs,
                    }, // J♣
                    Card {
                        rank: Rank::Six,
                        suit: Suit::Clubs,
                    }, // 6♣
                    Card {
                        rank: Rank::Jack,
                        suit: Suit::Diamonds,
                    }, // J♦
                    Card {
                        rank: Rank::King,
                        suit: Suit::Clubs,
                    }, // K♣
                ],
            ),
        ];

        for (seed, expected_column) in test_cases {
            let mut generator = GameGenerator::new(seed);
            generator.generate();
            let game = generator.game_state;

            // Test just the first column
            assert_eq!(
                game.tableau().column_length(0),
                expected_column.len(),
                "Game #{} column 0 has wrong number of cards",
                seed
            );

            for (card_idx, expected_card) in expected_column.iter().enumerate() {
                assert_eq!(
                    game.tableau().get_card_at(0, card_idx).unwrap(),
                    expected_card,
                    "Game #{} mismatch at column 0, card {}",
                    seed,
                    card_idx
                );
            }
        }
    }
}
