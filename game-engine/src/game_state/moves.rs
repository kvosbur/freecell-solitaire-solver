//! Move generation logic for GameState.

use super::GameState;
use crate::action::Action;
use crate::rules::Rules;  // Add this import

impl GameState {
    /// Returns all valid single-card moves from the current state.
    pub fn get_available_moves(&self) -> Vec<Action> {
        let mut moves = Vec::new();
        moves.extend(self.get_tableau_to_foundation_moves());
        moves.extend(self.get_freecell_to_foundation_moves());
        moves.extend(self.get_freecell_to_tableau_moves());
        moves.extend(self.get_tableau_to_tableau_moves());
        moves.extend(self.get_tableau_to_freecell_moves());
        moves
    }

    fn get_tableau_to_foundation_moves(&self) -> Vec<Action> {
        let mut moves = Vec::new();

        for from_col in 0..self.tableau.column_count() {
            if let Some(card) = self.tableau.get_card(from_col) {
                for to_pile in 0..self.foundations.pile_count() {
                    let foundation_top = self.foundations.get_card(to_pile);
                    
                    // Pass card by reference
                    if Rules::can_move_to_foundation(&card, foundation_top) {
                        moves.push(Action::TableauToFoundation {
                            from_column: from_col,
                            to_pile,
                        });
                    }
                }
            }
        }

        moves
    }

    fn get_freecell_to_foundation_moves(&self) -> Vec<Action> {
        let mut moves = Vec::new();
    
        // For each freecell
        for from_cell in 0..self.freecells.cell_count() {
            // Only proceed if there's a card in this freecell
            if let Some(card) = self.freecells.get_card(from_cell) {
                // Check all foundation piles
                for to_pile in 0..self.foundations.pile_count() {
                    let foundation_top = self.foundations.get_card(to_pile);
                    
                    // Change this line to use the Rules struct
                    if Rules::can_move_to_foundation(card, foundation_top) {
                        moves.push(Action::FreecellToFoundation {
                            from_cell,
                            to_pile,
                        });
                    }
                }
            }
        }
    
        moves
    }

    fn get_freecell_to_tableau_moves(&self) -> Vec<Action> {
        let mut moves = Vec::new();
        
        for from_cell in 0..self.freecells.cell_count() {
            if let Some(card) = self.freecells.get_card(from_cell) {
                for to_col in 0..self.tableau.column_count() {
                    let tableau_top = self.tableau.get_card(to_col);
                    
                    if Rules::can_stack_on_tableau(card, tableau_top) {
                        moves.push(Action::FreecellToTableau {
                            from_cell,
                            to_column: to_col,
                        });
                    }
                }
            }
        }
        
        moves
    }

    fn get_tableau_to_tableau_moves(&self) -> Vec<Action> {
        let mut moves = Vec::new();
        
        for from_col in 0..self.tableau.column_count() {
            if let Some(card) = self.tableau.get_card(from_col) {
                for to_col in 0..self.tableau.column_count() {
                    if from_col == to_col {
                        continue;
                    }
                    
                    let tableau_top = self.tableau.get_card(to_col);
                    
                    if Rules::can_stack_on_tableau(card, tableau_top) {
                        moves.push(Action::TableauToTableau {
                            from_column: from_col,
                            to_column: to_col,
                            card_count: 1,
                        });
                    }
                }
            }
        }
        
        moves
    }

    fn get_tableau_to_freecell_moves(&self) -> Vec<Action> {
        let mut moves = Vec::new();
        
        for from_col in 0..self.tableau.column_count() {
            if let Some(card) = self.tableau.get_card(from_col) {
                for to_cell in 0..self.freecells.cell_count() {
                    let freecell_content = self.freecells.get_card(to_cell);
                    
                    if Rules::can_move_to_freecell(card, freecell_content) {
                        moves.push(Action::TableauToFreecell {
                            from_column: from_col,
                            to_cell,
                        });
                    }
                }
            }
        }
        
        moves
    }
}
