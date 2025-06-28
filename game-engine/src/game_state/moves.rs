//! Move generation logic for GameState.
//!
//! This module provides functionality to generate all valid moves from a given game state.
//! It contains methods to identify possible moves between tableau columns, freecells and foundations.

use super::GameState;
use crate::r#move::Move;
impl GameState {
    /// Returns all valid single-card moves from the current state.
    ///
    /// This method aggregates moves from various sources (tableau, freecells)
    /// to various destinations (foundations, tableau, freecells) based on
    /// the current game state and FreeCell rules.
    ///
    /// # Returns
    ///
    /// A `Vec<Move>` containing all legal moves that can be made from the
    /// current game state.
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
        let mut moves = Vec::new();
        moves.extend(self.get_tableau_to_foundation_moves());
        moves.extend(self.get_freecell_to_foundation_moves());
        moves.extend(self.get_freecell_to_tableau_moves());
        moves.extend(self.get_tableau_to_tableau_moves());
        moves.extend(self.get_tableau_to_freecell_moves());
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
    ///
    /// let mut game = GameState::new();
    /// // Assume game state is set up such that a move is possible
    /// // e.g., game.tableau_mut().place_card(0, Card::new(Rank::Ace, Suit::Clubs)).unwrap();
    /// // game.foundations_mut().place_card(0, Card::new(Rank::King, Suit::Clubs)).unwrap(); // This would be invalid
    ///
    /// let moves = game.get_tableau_to_foundation_moves();
    /// // assert!(moves.contains(&Move::TableauToFoundation { from_column: 0, to_pile: 0 }));
    /// ```
    pub fn get_tableau_to_foundation_moves(&self) -> Vec<Move> {
        let mut moves = Vec::new();

        for from_col in 0..self.tableau().column_count() {
            let card_result = self.tableau().get_card(from_col);
            let card = match card_result {
                Ok(Some(c)) => c,
                _ => continue,
            };

            for to_pile in 0..self.foundations().pile_count() {
                if self.foundations().validate_card_placement(to_pile, card).is_ok() {
                    moves.push(Move::TableauToFoundation {
                        from_column: from_col as u8,
                        to_pile: to_pile as u8,
                    });
                }
            }
        }

        moves
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
    ///
    /// let mut game = GameState::new();
    /// // Assume a card is in freecell 0 and can be moved to foundation 0
    /// // game.freecells_mut().place_card(0, Card::new(Rank::Ace, Suit::Diamonds)).unwrap();
    ///
    /// let moves = game.get_freecell_to_foundation_moves();
    /// // assert!(moves.contains(&Move::FreecellToFoundation { from_cell: 0, to_pile: 0 }));
    /// ```
    pub fn get_freecell_to_foundation_moves(&self) -> Vec<Move> {
        let mut moves = Vec::new();
    
        for from_cell in 0..self.freecells().cell_count() {
            let card_result = self.freecells().get_card(from_cell);
            let card = match card_result {
                Ok(Some(card)) => card,
                _ => continue, // Skip this cell if no card or error
            };

            for to_pile in 0..self.foundations().pile_count() {
                if self.foundations().validate_card_placement(to_pile, card).is_ok() {
                    moves.push(Move::FreecellToFoundation {
                        from_cell: from_cell as u8,
                        to_pile: to_pile as u8,
                    });
                }
            }
        }
    
        moves
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
    ///
    /// let mut game = GameState::new();
    /// // Assume a card is in freecell 0 and can be moved to tableau 0
    /// // game.freecells_mut().place_card(0, Card::new(Rank::King, Suit::Spades)).unwrap();
    /// // game.tableau_mut().place_card(0, Card::new(Rank::Queen, Suit::Hearts)).unwrap();
    ///
    /// let moves = game.get_freecell_to_tableau_moves();
    /// // assert!(moves.contains(&Move::FreecellToTableau { from_cell: 0, to_column: 0 }));
    /// ```
    pub fn get_freecell_to_tableau_moves(&self) -> Vec<Move> {
        let mut moves = Vec::new();
        
        for from_cell in 0..self.freecells().cell_count() {
            let card_result = self.freecells().get_card(from_cell);
            let card = match card_result {
                Ok(Some(card)) => card,
                _ => continue, // Skip this cell if no card or error
            };

            for to_col in 0..self.tableau().column_count() {
                if self.tableau().validate_card_placement(to_col, card).is_ok() {
                    moves.push(Move::FreecellToTableau {
                        from_cell: from_cell as u8,
                        to_column: to_col as u8,
                    });
                }
            }
        }
        
        moves
    }

    /// Generates all valid moves from one tableau column to another.
    ///
    /// This method identifies single-card moves between tableau columns.
    /// Multi-card moves are handled by `validate_card_placement` in Tableau.
    ///
    /// # Returns
    ///
    /// A `Vec<Move>` containing all legal tableau-to-tableau moves.
    ///
    /// # Examples
    ///
    /// ```
    /// use freecell_game_engine::{GameState, r#move::Move, Card, Rank, Suit};
    ///
    /// let mut game = GameState::new();
    /// // Assume cards are set up for a valid move
    /// // game.tableau_mut().place_card(0, Card::new(Rank::Five, Suit::Clubs)).unwrap();
    /// // game.tableau_mut().place_card(1, Card::new(Rank::Six, Suit::Diamonds)).unwrap();
    ///
    /// let moves = game.get_tableau_to_tableau_moves();
    /// // assert!(moves.contains(&Move::TableauToTableau { from_column: 0, to_column: 1, card_count: 1 }));
    /// ```
    pub fn get_tableau_to_tableau_moves(&self) -> Vec<Move> {
        let mut moves = Vec::new();
        
        for from_col in 0..self.tableau().column_count() {
            let card_result = self.tableau().get_card(from_col);
            let card = match card_result {
                Ok(Some(card)) => card,
                _ => continue, // Skip this column if no card or error
            };

            for to_col in 0..self.tableau().column_count() {
                if from_col == to_col {
                    continue;
                }
                
                if self.tableau().validate_card_placement(to_col, card).is_ok() {
                    moves.push(Move::TableauToTableau {
                        from_column: from_col as u8,
                        to_column: to_col as u8,
                        card_count: 1, // Only single card moves generated here
                    });
                }
            }
        }
        
        moves
    }

    /// Generates all valid moves from tableau columns to freecells.
    ///
    /// This method checks each tableau column and determines if its top card
    /// can be moved to an empty freecell.
    ///
    /// # Returns
    ///
    /// A `Vec<Move>` containing all legal tableau-to-freecell moves.
    ///
    /// # Examples
    ///
    /// ```
    /// use freecell_game_engine::{GameState, r#move::Move, Card, Rank, Suit};
    ///
    /// let mut game = GameState::new();
    /// // Assume a card is in tableau 0 and freecell 0 is empty
    /// // game.tableau_mut().place_card(0, Card::new(Rank::Ace, Suit::Spades)).unwrap();
    ///
    /// let moves = game.get_tableau_to_freecell_moves();
    /// // assert!(moves.contains(&Move::TableauToFreecell { from_column: 0, to_cell: 0 }));
    /// ```
    pub fn get_tableau_to_freecell_moves(&self) -> Vec<Move> {
        let mut moves = Vec::new();
        
        for from_col in 0..self.tableau().column_count() {
            let card_result = self.tableau().get_card(from_col);
            let _card = match card_result {
                Ok(Some(card)) => card,
                _ => continue, // Skip this column if no card or error
            };

            for to_cell in 0..self.freecells().cell_count() {
                if self.freecells().is_cell_empty(to_cell).unwrap_or(false) {
                    moves.push(Move::TableauToFreecell {
                        from_column: from_col as u8,
                        to_cell: to_cell as u8,
                    });
                }
            }
        }
        
        moves
    }
}
