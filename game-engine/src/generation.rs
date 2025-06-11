use crate::{Card, GameState, Suit};

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
                rank: 1,
                suit: Suit::Clubs,
            },
            &Card {
                rank: 1,
                suit: Suit::Diamonds,
            },
            &Card {
                rank: 1,
                suit: Suit::Hearts,
            },
            &Card {
                rank: 1,
                suit: Suit::Spades,
            },
            &Card {
                rank: 2,
                suit: Suit::Clubs,
            },
            &Card {
                rank: 2,
                suit: Suit::Diamonds,
            },
            &Card {
                rank: 2,
                suit: Suit::Hearts,
            },
            &Card {
                rank: 2,
                suit: Suit::Spades,
            },
            &Card {
                rank: 3,
                suit: Suit::Clubs,
            },
            &Card {
                rank: 3,
                suit: Suit::Diamonds,
            },
            &Card {
                rank: 3,
                suit: Suit::Hearts,
            },
            &Card {
                rank: 3,
                suit: Suit::Spades,
            },
            &Card {
                rank: 4,
                suit: Suit::Clubs,
            },
            &Card {
                rank: 4,
                suit: Suit::Diamonds,
            },
            &Card {
                rank: 4,
                suit: Suit::Hearts,
            },
            &Card {
                rank: 4,
                suit: Suit::Spades,
            },
            &Card {
                rank: 5,
                suit: Suit::Clubs,
            },
            &Card {
                rank: 5,
                suit: Suit::Diamonds,
            },
            &Card {
                rank: 5,
                suit: Suit::Hearts,
            },
            &Card {
                rank: 5,
                suit: Suit::Spades,
            },
            &Card {
                rank: 6,
                suit: Suit::Clubs,
            },
            &Card {
                rank: 6,
                suit: Suit::Diamonds,
            },
            &Card {
                rank: 6,
                suit: Suit::Hearts,
            },
            &Card {
                rank: 6,
                suit: Suit::Spades,
            },
            &Card {
                rank: 7,
                suit: Suit::Clubs,
            },
            &Card {
                rank: 7,
                suit: Suit::Diamonds,
            },
            &Card {
                rank: 7,
                suit: Suit::Hearts,
            },
            &Card {
                rank: 7,
                suit: Suit::Spades,
            },
            &Card {
                rank: 8,
                suit: Suit::Clubs,
            },
            &Card {
                rank: 8,
                suit: Suit::Diamonds,
            },
            &Card {
                rank: 8,
                suit: Suit::Hearts,
            },
            &Card {
                rank: 8,
                suit: Suit::Spades,
            },
            &Card {
                rank: 9,
                suit: Suit::Clubs,
            },
            &Card {
                rank: 9,
                suit: Suit::Diamonds,
            },
            &Card {
                rank: 9,
                suit: Suit::Hearts,
            },
            &Card {
                rank: 9,
                suit: Suit::Spades,
            },
            &Card {
                rank: 10,
                suit: Suit::Clubs,
            },
            &Card {
                rank: 10,
                suit: Suit::Diamonds,
            },
            &Card {
                rank: 10,
                suit: Suit::Hearts,
            },
            &Card {
                rank: 10,
                suit: Suit::Spades,
            },
            &Card {
                rank: 11,
                suit: Suit::Clubs,
            },
            &Card {
                rank: 11,
                suit: Suit::Diamonds,
            },
            &Card {
                rank: 11,
                suit: Suit::Hearts,
            },
            &Card {
                rank: 11,
                suit: Suit::Spades,
            },
            &Card {
                rank: 12,
                suit: Suit::Clubs,
            },
            &Card {
                rank: 12,
                suit: Suit::Diamonds,
            },
            &Card {
                rank: 12,
                suit: Suit::Hearts,
            },
            &Card {
                rank: 12,
                suit: Suit::Spades,
            },
            &Card {
                rank: 13,
                suit: Suit::Clubs,
            },
            &Card {
                rank: 13,
                suit: Suit::Diamonds,
            },
            &Card {
                rank: 13,
                suit: Suit::Hearts,
            },
            &Card {
                rank: 13,
                suit: Suit::Spades,
            },
        ];

        let mut current_column_for_insert: usize = 0;
        let max_columns: usize = 8;

        while cards.len() > 0 {
            let next_rng_value: usize = self.rng.next_value() as usize;
            println!("rng value {:?}", next_rng_value);
            let next_index = next_rng_value % cards.len();
            let end_index = cards.len() - 1;
            let card_at_index = cards[next_index];
            let card_at_end = cards[end_index];

            cards[next_index] = card_at_end;
            cards[end_index] = card_at_index;

            let next_card = cards.pop().unwrap();

            self.game_state
                .tableau
                .initial_addition_of_card(current_column_for_insert, next_card.clone());

            current_column_for_insert = (current_column_for_insert + 1) % max_columns
        }
    }
}
