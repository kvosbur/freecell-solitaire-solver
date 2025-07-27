//! Move generation logic for GameState.
//!
//! This module provides functionality to generate all valid moves from a given game state.
//! It contains methods to identify possible moves between tableau columns, freecells and foundations.

use super::GameState;
use crate::{
    freecells::FREECELL_COUNT, location::FoundationLocation,
    r#move::Move, tableau::TABLEAU_COLUMN_COUNT,
};

impl GameState {
    /// Returns all valid moves from the current state.
    ///
    /// This method aggregates moves from various sources (tableau, freecells)
    /// to various destinations (foundations, tableau, freecells) based on
    /// the current game state and FreeCell rules. Multi-card tableau-to-tableau
    /// moves are supported when sufficient freecells and empty columns are available.
    ///
    /// # Returns
    ///
    /// A `Vec<Move>` containing all legal moves that can be made from the
    /// current game state, including both single-card and multi-card sequences.
    ///
    /// # Examples
    ///
    /// ```
    /// use freecell_game_engine::GameState;
    ///
    /// let game = GameState::new(); // Represents a new, shuffled game
    /// let moves = game.get_available_moves();
    /// println!("Found {} possible moves", moves.len());
    /// // Solvers would typically iterate through these moves to explore the game tree.
    /// ```
    pub fn get_available_moves(&self) -> Vec<Move> {
        let mut moves = Vec::with_capacity(20);
        self.get_tableau_to_foundation_moves(&mut moves);
        self.get_freecell_to_foundation_moves(&mut moves);
        self.get_freecell_to_tableau_moves(&mut moves);
        self.get_tableau_to_tableau_moves_single_card(&mut moves);
        self.get_tableau_to_freecell_moves(&mut moves);
        moves
    }

    /// Generates all valid moves from tableau columns to foundation piles.
    ///
    /// This method iterates through all tableau columns and checks if their
    /// top card can be moved to any of the foundation piles according to
    /// FreeCell rules (same suit, ascending rank).
    ///
    /// # Returns
    ///
    /// A `Vec<Move>` containing all legal tableau-to-foundation moves.
    ///
    /// # Examples
    ///
    /// ```
    /// use freecell_game_engine::{GameState, r#move::Move, Card, Rank, Suit};
    /// use freecell_game_engine::location::{TableauLocation, FoundationLocation};
    ///
    /// let mut game = GameState::new();
    /// // Assume game state is set up such that a move is possible
    /// // let location = TableauLocation::new(0).unwrap();
    /// // game.tableau_mut().place_card(location, Card::new(Rank::Ace, Suit::Clubs)).unwrap();
    ///
    /// let moves = game.get_tableau_to_foundation_moves();
    /// // assert!(moves.contains(&Move::TableauToFoundation { from: TableauLocation::new(0).unwrap(), to: FoundationLocation::new(0).unwrap() }));
    /// ```
    pub fn get_tableau_to_foundation_moves(&self, moves: &mut Vec<Move>) {
        for from_col in 0..TABLEAU_COLUMN_COUNT {
            let location = crate::location::TableauLocation::new(from_col as u8).unwrap();
            let card_result = self.tableau().get_card(location);
            let card = match card_result {
                Ok(Some(c)) => c,
                _ => continue,
            };

            // Directly compute the target foundation based on card suit
            let to_pile = card.suit().foundation_index();
            let foundation_location = FoundationLocation::new(to_pile).unwrap();
            if self
                .foundations()
                .validate_card_placement(foundation_location, card)
                .is_ok()
            {
                if let Ok(m) = Move::tableau_to_foundation(from_col as u8, to_pile) {
                    moves.push(m);
                }
            }
        }
    }

    /// Generates all valid moves from freecells to foundation piles.
    ///
    /// This method checks each occupied freecell and determines if its card
    /// can be moved to any of the foundation piles.
    ///
    /// # Returns
    ///
    /// A `Vec<Move>` containing all legal freecell-to-foundation moves.
    ///
    /// # Examples
    ///
    /// ```
    /// use freecell_game_engine::{GameState, r#move::Move, Card, Rank, Suit};
    /// use freecell_game_engine::location::FreecellLocation;
    ///
    /// let mut game = GameState::new();
    /// // Assume a card is in freecell 0 and can be moved to foundation 0
    /// // let location = FreecellLocation::new(0).unwrap();
    /// // game.freecells_mut().place_card(location, Card::new(Rank::Ace, Suit::Diamonds)).unwrap();
    ///
    /// let moves = game.get_freecell_to_foundation_moves();
    /// // assert!(moves.contains(&Move::FreecellToFoundation { from: FreecellLocation::new(0).unwrap(), to: FoundationLocation::new(0).unwrap() }));
    /// ```
    pub fn get_freecell_to_foundation_moves(&self, moves: &mut Vec<Move>) {
        for from_cell in 0..FREECELL_COUNT {
            let location = crate::location::FreecellLocation::new(from_cell as u8).unwrap();
            let card_result = self.freecells().get_card(location);
            let card = match card_result {
                Ok(Some(c)) => c,
                _ => continue,
            };

            // Directly compute the target foundation based on card suit
            let to_pile = card.suit().foundation_index();
            let foundation_location = FoundationLocation::new(to_pile).unwrap();
            if self
                .foundations()
                .validate_card_placement(foundation_location, card)
                .is_ok()
            {
                if let Ok(m) = Move::freecell_to_foundation(from_cell as u8, to_pile) {
                    moves.push(m);
                }
            }
        }
    }

    /// Generates all valid moves from freecells to tableau columns.
    ///
    /// This method checks each occupied freecell and determines if its card
    /// can be moved to any tableau column.
    ///
    /// # Returns
    ///
    /// A `Vec<Move>` containing all legal freecell-to-tableau moves.
    ///
    /// # Examples
    ///
    /// ```
    /// use freecell_game_engine::{GameState, r#move::Move, Card, Rank, Suit};
    /// use freecell_game_engine::location::{FreecellLocation, TableauLocation};
    ///
    /// let mut game = GameState::new();
    /// // Assume a card is in freecell 0 and can be moved to tableau 0
    /// // let freecell_location = FreecellLocation::new(0).unwrap();
    /// // game.freecells_mut().place_card(freecell_location, Card::new(Rank::King, Suit::Spades)).unwrap();
    /// // let tableau_location = TableauLocation::new(0).unwrap();
    /// // game.tableau_mut().place_card(tableau_location, Card::new(Rank::Queen, Suit::Hearts)).unwrap();
    ///
    /// let moves = game.get_freecell_to_tableau_moves();
    /// // assert!(moves.contains(&Move::FreecellToTableau { from: FreecellLocation::new(0).unwrap(), to: TableauLocation::new(0).unwrap() }));
    /// ```
    pub fn get_freecell_to_tableau_moves(&self, moves: &mut Vec<Move>) {
        for from_cell in 0..crate::freecells::FREECELL_COUNT {
            let location = crate::location::FreecellLocation::new(from_cell as u8).unwrap();
            let card_result = self.freecells().get_card(location);
            let card = match card_result {
                Ok(Some(card)) => card,
                _ => continue, // Skip this cell if no card or error
            };

            for to_col in 0..TABLEAU_COLUMN_COUNT {
                let to_location = crate::location::TableauLocation::new(to_col as u8).unwrap();
                if self
                    .tableau()
                    .validate_card_placement(to_location, card)
                    .is_ok()
                {
                    if let Ok(m) = Move::freecell_to_tableau(from_cell as u8, to_col as u8) {
                        moves.push(m);
                    }
                }
            }
        }
    }

    /// Calculates the maximum number of cards that can be moved as a sequence.
    ///
    /// In FreeCell, the number of cards that can be moved together depends on:
    /// - Available freecells (temporary storage during multi-card moves)
    /// - Empty tableau columns (can hold entire sequences)
    ///
    /// Formula: `(empty_freecells + 1) × 2^empty_tableau_columns`
    ///
    /// # Returns
    ///
    /// The maximum number of cards that can be moved as a single sequence.
    ///
    /// # Performance
    ///
    /// This method has O(1) complexity as it only counts empty cells/columns.
    ///
    /// # Examples
    ///
    /// ```
    /// use freecell_game_engine::GameState;
    ///
    /// let game = GameState::new();
    /// let max_movable = game.calculate_max_movable_cards();
    /// // With 4 empty freecells and 0 empty columns: (4+1) * 2^0 = 5
    /// ```
    fn calculate_max_movable_cards(&self) -> usize {
        // Count empty freecells
        let mut empty_freecells = 0;
        for cell in 0..crate::freecells::FREECELL_COUNT {
            let location = crate::location::FreecellLocation::new(cell as u8).unwrap();
            if self
                .freecells()
                .get_card(location)
                .unwrap_or(None)
                .is_none()
            {
                empty_freecells += 1;
            }
        }

        // Count empty tableau columns
        let empty_tableau_columns = self.tableau().empty_columns_count();

        // Cap empty_tableau_columns to prevent overflow (2^20 is reasonable upper bound)
        let capped_empty_columns = empty_tableau_columns.min(20);
        (empty_freecells + 1) * (1_usize << capped_empty_columns)
    }

    /// Extracts the longest valid movable sequence from the top of a tableau column.
    ///
    /// A valid sequence consists of cards in descending rank order with alternating colors.
    /// Only sequences that can actually be moved (considering freecell capacity) are returned.
    ///
    /// # Arguments
    ///
    /// * `column` - The tableau column index to examine
    ///
    /// # Returns
    ///
    /// A vector of cards forming the longest movable sequence from the top of the column.
    /// The first card is the topmost card, the last card is the bottom of the sequence.
    /// Returns an empty vector if no cards can be moved.
    ///
    /// # Performance
    ///
    /// This method has O(n) complexity where n is the number of cards in the column.
    /// It stops early when an invalid sequence is found or max capacity is reached.
    ///
    /// # Examples
    ///
    /// ```
    /// use freecell_game_engine::GameState;
    ///
    /// let game = GameState::new();
    /// let sequence = game.get_movable_sequence_from_column(0);
    /// // Returns cards that form a valid descending, alternating-color sequence
    /// ```
    fn get_movable_sequence_from_column(&self, column: usize) -> Vec<crate::Card> {
        let max_movable = self.calculate_max_movable_cards();
        let mut sequence = Vec::new();

        // Early exit if no cards can be moved
        if max_movable == 0 {
            return sequence;
        }

        let column_location = match crate::location::TableauLocation::new(column as u8) {
            Ok(loc) => loc,
            Err(_) => return sequence, // Invalid column
        };

        let column_length = match self.tableau().column_length(column_location) {
            Ok(length) => length,
            Err(_) => return sequence, // Error getting column length
        };

        if column_length == 0 {
            return sequence; // Empty column
        }

        // Start from the top card and work backwards to build the sequence
        // The top card is always valid to move (if any card can be moved)
        let top_card_index = column_length - 1;

        // Get the top card first
        if let Ok(top_card) = self.tableau().get_card_at(column_location, top_card_index) {
            sequence.push(*top_card);
        } else {
            return sequence; // Can't get top card
        }

        // Build sequence by checking cards below the top card
        let max_sequence_length = max_movable.min(column_length);

        for sequence_length in 2..=max_sequence_length {
            let next_card_index = column_length - sequence_length;

            if let Ok(next_card) = self.tableau().get_card_at(column_location, next_card_index) {
                let last_card_in_sequence = sequence[sequence.len() - 1];

                // Check if this card can be part of the sequence
                if Self::forms_valid_tableau_sequence(*next_card, last_card_in_sequence) {
                    sequence.push(*next_card);
                } else {
                    break; // Sequence is broken
                }
            } else {
                break; // Can't access this card
            }
        }

        // Reverse the sequence so it's ordered from top to bottom
        sequence.reverse();
        sequence
    }

    /// Checks if two cards form a valid tableau sequence (descending rank, alternating color).
    ///
    /// # Arguments
    ///
    /// * `top_card` - The card that would be on top (higher in the column)
    /// * `bottom_card` - The card that would be underneath (lower in the column)
    ///
    /// # Returns
    ///
    /// `true` if the cards form a valid sequence for tableau play.
    ///
    /// # Examples
    ///
    /// ```
    /// use freecell_game_engine::{Card, Rank, Suit};
    /// use freecell_game_engine::GameState;
    ///
    /// let red_queen = Card::new(Rank::Queen, Suit::Hearts);
    /// let black_jack = Card::new(Rank::Jack, Suit::Spades);
    ///
    /// assert!(GameState::forms_valid_tableau_sequence(red_queen, black_jack));
    /// ```
    #[inline]
    fn forms_valid_tableau_sequence(top_card: crate::Card, bottom_card: crate::Card) -> bool {
        // Check if the top card is one rank higher than the bottom card
        // and they have alternating colors
        top_card.is_one_higher_than(&bottom_card) && top_card.color() != bottom_card.color()
    }

    /// Generates all valid moves from one tableau column to another.
    ///
    /// This method identifies both single-card and multi-card moves between tableau columns.
    /// Multi-card moves are only generated when there are sufficient freecells and empty
    /// tableau columns to support the temporary storage required for the move.
    ///
    /// The method prioritizes longer sequences as they are typically more valuable for solving,
    /// and avoids generating redundant shorter moves when a longer move to the same destination
    /// is possible.
    ///
    /// # Returns
    ///
    /// A `Vec<Move>` containing all legal tableau-to-tableau moves, with longer sequences
    /// prioritized over shorter ones to the same destination.
    ///
    /// # Performance
    ///
    /// This method has O(n²m) complexity where n is the number of tableau columns
    /// and m is the average number of cards per column. The implementation is optimized
    /// to avoid generating redundant moves and unnecessary sequence calculations.
    ///
    /// # Examples
    ///
    /// ```
    /// use freecell_game_engine::{GameState, r#move::Move, Card, Rank, Suit};
    /// use freecell_game_engine::location::TableauLocation;
    ///
    /// let mut game = GameState::new();
    /// // Multi-card sequences will be detected and valid moves generated
    /// let moves = game.get_tableau_to_tableau_moves();
    /// // Moves may include single cards or multi-card sequences
    /// ```
    pub fn get_tableau_to_tableau_moves(&self, moves: &mut Vec<Move>) {
        let max_movable = self.calculate_max_movable_cards();

        // Early exit if no cards can be moved
        if max_movable == 0 {
            return;
        }

        for from_col in 0..TABLEAU_COLUMN_COUNT {
            let sequence = self.get_movable_sequence_from_column(from_col);
            if sequence.is_empty() {
                continue;
            }

            for to_col in 0..TABLEAU_COLUMN_COUNT {
                if from_col == to_col {
                    continue;
                }
                // Try sequence lengths from longest to shortest
                // This prioritizes more valuable moves and avoids generating redundant shorter moves
                let max_sequence_length = sequence.len().min(max_movable);

                for card_count in (1..=max_sequence_length).rev() {
                    // The bottom card of the sequence is what we're trying to place
                    let bottom_card = sequence[card_count - 1];

                    let to_location = crate::location::TableauLocation::new(to_col as u8).unwrap();
                    if self
                        .tableau()
                        .validate_card_placement(to_location, &bottom_card)
                        .is_ok()
                    {
                        if let Ok(m) =
                            Move::tableau_to_tableau(from_col as u8, to_col as u8, card_count as u8)
                        {
                            moves.push(m);
                        }
                        break; // Only add the longest valid sequence to avoid redundant moves
                    }
                }
            }
        }
    }

    pub fn get_tableau_to_tableau_moves_single_card(&self, moves: &mut Vec<Move>) {
        for from_col in 0..TABLEAU_COLUMN_COUNT {
            let location = crate::location::TableauLocation::new(from_col as u8).unwrap();
            let card_result = self.tableau().get_card(location);
            let card = match card_result {
                Ok(Some(card)) => card,
                _ => continue, // Skip this cell if no card or error
            };

            for to_col in 0..TABLEAU_COLUMN_COUNT {
                let to_location = crate::location::TableauLocation::new(to_col as u8).unwrap();
                if from_col == to_col {
                    continue;
                }
                if self
                    .tableau()
                    .validate_card_placement(to_location, card)
                    .is_ok()
                {
                    if let Ok(m) = Move::tableau_to_tableau(from_col as u8, to_col as u8, 1) {
                        moves.push(m);
                    }
                }
            }
        }
    }

    /// Generates all valid moves from tableau columns to freecells.
    ///
    /// This method checks each tableau column and determines if its top card
    /// can be moved to the first available freecell. Only one move per tableau
    /// column is generated (to the first empty freecell) to avoid redundant moves.
    ///
    /// # Returns
    ///
    /// A `Vec<Move>` containing all legal tableau-to-freecell moves.
    ///
    /// # Examples
    ///
    /// ```
    /// use freecell_game_engine::{GameState, r#move::Move, Card, Rank, Suit};
    /// use freecell_game_engine::location::TableauLocation;
    ///
    /// let mut game = GameState::new();
    /// // Assume a card is in tableau 0 and freecell 0 is empty
    /// // let location = TableauLocation::new(0).unwrap();
    /// // game.tableau_mut().place_card(location, Card::new(Rank::Ace, Suit::Spades)).unwrap();
    ///
    /// let moves = game.get_tableau_to_freecell_moves();
    /// // assert!(moves.contains(&Move::TableauToFreecell { from: TableauLocation::new(0).unwrap(), to: FreecellLocation::new(0).unwrap() }));
    /// ```
    pub fn get_tableau_to_freecell_moves(&self, moves: &mut Vec<Move>) {
        for from_col in 0..TABLEAU_COLUMN_COUNT {
            let location = crate::location::TableauLocation::new(from_col as u8).unwrap();
            let card_result = self.tableau().get_card(location);
            let _card = match card_result {
                Ok(Some(card)) => card,
                _ => continue, // Skip this column if no card or error
            };

            // Find the first available freecell and add only one move per tableau column
            for to_cell in 0..crate::freecells::FREECELL_COUNT {
                let location = crate::location::FreecellLocation::new(to_cell as u8).unwrap();
                if self
                    .freecells()
                    .get_card(location)
                    .unwrap_or(None)
                    .is_none()
                {
                    if let Ok(m) = Move::tableau_to_freecell(from_col as u8, to_cell as u8) {
                        moves.push(m);
                        break; // Only add move to first available freecell, then move to next tableau column
                    }
                }
            }
        }
    }
}
