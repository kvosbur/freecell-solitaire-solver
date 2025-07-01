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
    pub fn get_tableau_to_foundation_moves(&self) -> Vec<Move> {
        let mut moves = Vec::new();

        for from_col in 0..self.tableau().column_count() {
            let location = crate::location::TableauLocation::new(from_col as u8).unwrap();
            let card_result = self.tableau().get_card(location);
            let card = match card_result {
                Ok(Some(c)) => c,
                _ => continue,
            };

            for to_pile in 0..self.foundations().pile_count() {
                if self.foundations().validate_card_placement(to_pile, card).is_ok() {
                    if let Ok(m) = Move::tableau_to_foundation(from_col as u8, to_pile as u8) {
                        moves.push(m);
                    }
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
    pub fn get_freecell_to_foundation_moves(&self) -> Vec<Move> {
        let mut moves = Vec::new();
    
        for from_cell in 0..self.freecells().cell_count() {
            let location = crate::location::FreecellLocation::new(from_cell as u8).unwrap();
            let card_result = self.freecells().get_card(location);
            let card = match card_result {
                Ok(Some(card)) => card,
                _ => continue, // Skip this cell if no card or error
            };

            for to_pile in 0..self.foundations().pile_count() {
                if self.foundations().validate_card_placement(to_pile, card).is_ok() {
                    if let Ok(m) = Move::freecell_to_foundation(from_cell as u8, to_pile as u8) {
                        moves.push(m);
                    }
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
    pub fn get_freecell_to_tableau_moves(&self) -> Vec<Move> {
        let mut moves = Vec::new();
        
        for from_cell in 0..self.freecells().cell_count() {
            let location = crate::location::FreecellLocation::new(from_cell as u8).unwrap();
            let card_result = self.freecells().get_card(location);
            let card = match card_result {
                Ok(Some(card)) => card,
                _ => continue, // Skip this cell if no card or error
            };

            for to_col in 0..self.tableau().column_count() {
                if self.tableau().validate_card_placement(to_col, card).is_ok() {
                    if let Ok(m) = Move::freecell_to_tableau(from_cell as u8, to_col as u8) {
                        moves.push(m);
                    }
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
    /// use freecell_game_engine::location::TableauLocation;
    ///
    /// let mut game = GameState::new();
    /// // Assume cards are set up for a valid move
    /// // let location0 = TableauLocation::new(0).unwrap();
    /// // game.tableau_mut().place_card(location0, Card::new(Rank::Five, Suit::Clubs)).unwrap();
    /// // let location1 = TableauLocation::new(1).unwrap();
    /// // game.tableau_mut().place_card(location1, Card::new(Rank::Six, Suit::Diamonds)).unwrap();
    ///
    /// let moves = game.get_tableau_to_tableau_moves();
    /// // assert!(moves.contains(&Move::TableauToTableau { from: TableauLocation::new(0).unwrap(), to: TableauLocation::new(1).unwrap(), card_count: 1 }));
    /// ```
    pub fn get_tableau_to_tableau_moves(&self) -> Vec<Move> {
        let mut moves = Vec::new();
        
        for from_col in 0..self.tableau().column_count() {
            let location = crate::location::TableauLocation::new(from_col as u8).unwrap();
            let card_result = self.tableau().get_card(location);
            let card = match card_result {
                Ok(Some(card)) => card,
                _ => continue, // Skip this column if no card or error
            };

            for to_col in 0..self.tableau().column_count() {
                if from_col == to_col {
                    continue;
                }
                
                if self.tableau().validate_card_placement(to_col, card).is_ok() {
                    if let Ok(m) = Move::tableau_to_tableau(from_col as u8, to_col as u8, 1) {
                        moves.push(m);
                    }
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
    pub fn get_tableau_to_freecell_moves(&self) -> Vec<Move> {
        let mut moves = Vec::new();
        
        for from_col in 0..self.tableau().column_count() {
            let location = crate::location::TableauLocation::new(from_col as u8).unwrap();
            let card_result = self.tableau().get_card(location);
            let _card = match card_result {
                Ok(Some(card)) => card,
                _ => continue, // Skip this column if no card or error
            };

            for to_cell in 0..self.freecells().cell_count() {
                let location = crate::location::FreecellLocation::new(to_cell as u8).unwrap();
                if self.freecells().is_cell_empty(location).unwrap_or(false) {
                    if let Ok(m) = Move::tableau_to_freecell(from_col as u8, to_cell as u8) {
                        moves.push(m);
                    }
                }
            }
        }
        
        moves
    }
}
