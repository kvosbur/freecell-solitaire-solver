//! Move generation logic for GameState.
//!
//! This module provides functionality to generate all valid moves from a given game state.
//! It contains methods to identify possible moves between tableau columns, freecells and foundations.

use super::GameState;
use crate::action::Action;
use crate::rules::Rules;

impl GameState {
    /// Returns all valid single-card moves from the current state.
    ///
    /// # Returns
    ///
    /// A vector containing all legal moves that can be made from the current game state.
    ///
    /// # Examples
    ///
    /// ```
    /// # use freecell_game_engine::game_state::GameState;
    /// #
    /// # let game = GameState::new();
    /// let moves = game.get_available_moves();
    /// println!("Found {} possible moves", moves.len());
    /// ```
    pub fn get_available_moves(&self) -> Vec<Action> {
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
    /// # Returns
    ///
    /// A vector of valid tableau-to-foundation moves.
    fn get_tableau_to_foundation_moves(&self) -> Vec<Action> {
        let mut moves = Vec::new();

        for from_col in 0..self.tableau().column_count() {
            // Properly handle the Result<Option<&Card>, TableauError>
            let card_result = self.tableau().get_card(from_col);
            let card = match card_result {
                Ok(Some(card)) => card,
                _ => continue, // Skip this column if no card or error
            };

            for to_pile in 0..self.foundations().pile_count() {
                // Properly handle the Result<Option<&Card>, FoundationError>
                let foundation_result = self.foundations().get_card(to_pile);
                let foundation_top = match foundation_result {
                    Ok(top) => top,
                    _ => continue, // Skip this pile if error
                };
                
                if Rules::can_move_to_foundation(card, foundation_top) {
                    moves.push(Action::TableauToFoundation {
                        from_column: from_col,
                        to_pile,
                    });
                }
            }
        }

        moves
    }

    /// Generates all valid moves from freecells to foundation piles.
    ///
    /// # Returns
    ///
    /// A vector of valid freecell-to-foundation moves.
    fn get_freecell_to_foundation_moves(&self) -> Vec<Action> {
        let mut moves = Vec::new();
    
        // For each freecell
        for from_cell in 0..self.freecells().cell_count() {
            // Properly handle the Result<Option<&Card>, FreeCellError>
            let card_result = self.freecells().get_card(from_cell);
            let card = match card_result {
                Ok(Some(card)) => card,
                _ => continue, // Skip this cell if no card or error
            };

            // Check all foundation piles
            for to_pile in 0..self.foundations().pile_count() {
                let foundation_result = self.foundations().get_card(to_pile);
                let foundation_top = match foundation_result {
                    Ok(top) => top,
                    _ => continue, // Skip this pile if error
                };
                
                if Rules::can_move_to_foundation(card, foundation_top) {
                    moves.push(Action::FreecellToFoundation {
                        from_cell,
                        to_pile,
                    });
                }
            }
        }
    
        moves
    }

    /// Generates all valid moves from freecells to tableau columns.
    ///
    /// # Returns
    ///
    /// A vector of valid freecell-to-tableau moves.
    fn get_freecell_to_tableau_moves(&self) -> Vec<Action> {
        let mut moves = Vec::new();
        
        for from_cell in 0..self.freecells().cell_count() {
            // Properly handle the Result<Option<&Card>, FreeCellError>
            let card_result = self.freecells().get_card(from_cell);
            let card = match card_result {
                Ok(Some(card)) => card,
                _ => continue, // Skip this cell if no card or error
            };

            for to_col in 0..self.tableau().column_count() {
                let tableau_result = self.tableau().get_card(to_col);
                let tableau_top = match tableau_result {
                    Ok(top) => top,
                    _ => continue, // Skip this column if error
                };
                
                if Rules::can_stack_on_tableau(card, tableau_top) {
                    moves.push(Action::FreecellToTableau {
                        from_cell,
                        to_column: to_col,
                    });
                }
            }
        }
        
        moves
    }

    /// Generates all valid moves from one tableau column to another.
    ///
    /// # Returns
    ///
    /// A vector of valid tableau-to-tableau moves.
    fn get_tableau_to_tableau_moves(&self) -> Vec<Action> {
        let mut moves = Vec::new();
        
        for from_col in 0..self.tableau().column_count() {
            // Properly handle the Result<Option<&Card>, TableauError>
            let card_result = self.tableau().get_card(from_col);
            let card = match card_result {
                Ok(Some(card)) => card,
                _ => continue, // Skip this column if no card or error
            };

            for to_col in 0..self.tableau().column_count() {
                if from_col == to_col {
                    continue;
                }
                
                let tableau_result = self.tableau().get_card(to_col);
                let tableau_top = match tableau_result {
                    Ok(top) => top,
                    _ => continue, // Skip this column if error
                };
                
                if Rules::can_stack_on_tableau(card, tableau_top) {
                    moves.push(Action::TableauToTableau {
                        from_column: from_col,
                        to_column: to_col,
                        card_count: 1,
                    });
                }
            }
        }
        
        moves
    }

    /// Generates all valid moves from tableau columns to freecells.
    ///
    /// # Returns
    ///
    /// A vector of valid tableau-to-freecell moves.
    fn get_tableau_to_freecell_moves(&self) -> Vec<Action> {
        let mut moves = Vec::new();
        
        for from_col in 0..self.tableau().column_count() {
            // Properly handle the Result<Option<&Card>, TableauError>
            let card_result = self.tableau().get_card(from_col);
            let card = match card_result {
                Ok(Some(card)) => card,
                _ => continue, // Skip this column if no card or error
            };

            for to_cell in 0..self.freecells().cell_count() {
                let freecell_result = self.freecells().get_card(to_cell);
                let freecell_content = match freecell_result {
                    Ok(content) => content,
                    _ => continue, // Skip this cell if error
                };
                
                if Rules::can_move_to_freecell(card, freecell_content) {
                    moves.push(Action::TableauToFreecell {
                        from_column: from_col,
                        to_cell,
                    });
                }
            }
        }
        
        moves
    }
}
