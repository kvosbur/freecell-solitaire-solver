//! Move execution and undo logic for GameState.

use super::{GameState, GameError};
use crate::r#move::Move;

impl GameState {
    /// Executes a given move, applying its effects to the game state.
    ///
    /// This method first validates the move using `is_move_valid` and then
    /// performs the necessary mutations to the tableau, freecells, or foundations.
    ///
    /// # Arguments
    ///
    /// * `m` - A reference to the `Move` to be executed.
    ///
    /// # Returns
    ///
    /// * `Ok(())` if the move was successfully executed.
    /// * `Err(GameError)` if the move is invalid or an internal error occurs during execution.
    ///
    /// # Examples
    ///
    /// ```
    /// use freecell_game_engine::{GameState, r#move::Move, Card, Rank, Suit};
    ///
    /// let mut game = GameState::new();
    /// // Assume game state is set up such that this move is valid
    /// // For example, deal a specific game or manually set up cards
    ///
    /// // Example: Move a card from Tableau column 0 to Freecell 0
    /// // (Requires a card in Tableau 0 and Freecell 0 to be empty)
    /// // game.tableau_mut().place_card(0, Card::new(Rank::Ace, Suit::Spades)).unwrap();
    /// let move_cmd = Move::TableauToFreecell { from_column: 0, to_cell: 0 };
    ///
    /// // If the move is valid, execute it
    /// if game.is_move_valid(&move_cmd).is_ok() {
    ///     let result = game.execute_move(&move_cmd);
    ///     assert!(result.is_ok());
    /// }
    /// ```
    pub fn execute_move(&mut self, m: &Move) -> Result<(), GameError> {
        use Move::*;
        match m {
            TableauToFoundation { from_column, to_pile } => {
                self.execute_tableau_to_foundation(*from_column, *to_pile, m)
            }
            TableauToFreecell { from_column, to_cell } => {
                self.execute_tableau_to_freecell(*from_column, *to_cell, m)
            }
            FreecellToTableau { from_cell, to_column } => {
                self.execute_freecell_to_tableau(*from_cell, *to_column, m)
            }
            FreecellToFoundation { from_cell, to_pile } => {
                self.execute_freecell_to_foundation(*from_cell, *to_pile, m)
            }
            TableauToTableau { from_column, to_column, card_count } => {
                self.execute_tableau_to_tableau(*from_column, *to_column, *card_count, m)
            }
        }
    }

    /// Executes a move from a tableau column to a foundation pile.
    ///
    /// This is a private helper function called by `execute_move`. It assumes
    /// the move has already been validated.
    ///
    /// # Arguments
    ///
    /// * `from_column` - The 0-indexed source tableau column.
    /// * `to_pile` - The 0-indexed destination foundation pile.
    /// * `m` - The `Move` being executed (used for re-validation).
    ///
    /// # Returns
    ///
    /// * `Ok(())` if the card was successfully moved.
    /// * `Err(GameError)` if an unexpected error occurs during component interaction.
    fn execute_tableau_to_foundation(
        &mut self,
        from_column: u8,
        to_pile: u8,
        m: &Move,
    ) -> Result<(), GameError> {
        let _card = *self
            .tableau
            .get_card(from_column as usize)?
            .ok_or(GameError::EmptySource)?;
        self.is_move_valid(m)?;
        let removed = self.tableau.remove_card(from_column as usize)?;
        let removed_card = removed.ok_or(GameError::EmptySource)?;
        self.foundations.place_card(to_pile as usize, removed_card)?;
        Ok(())
    }

    /// Executes a move from a tableau column to a freecell.
    ///
    /// This is a private helper function called by `execute_move`. It assumes
    /// the move has already been validated.
    ///
    /// # Arguments
    ///
    /// * `from_column` - The 0-indexed source tableau column.
    /// * `to_cell` - The 0-indexed destination freecell.
    /// * `m` - The `Move` being executed (used for re-validation).
    ///
    /// # Returns
    ///
    /// * `Ok(())` if the card was successfully moved.
    /// * `Err(GameError)` if an unexpected error occurs during component interaction.
    fn execute_tableau_to_freecell(
        &mut self,
        from_column: u8,
        to_cell: u8,
        m: &Move,
    ) -> Result<(), GameError> {
        let _card = *self
            .tableau
            .get_card(from_column as usize)?
            .ok_or(GameError::EmptySource)?;
        let _card = *self
            .tableau
            .get_card(from_column as usize)?
            .ok_or(GameError::EmptySource)?;
        self.is_move_valid(m)?;
        let removed = self.tableau.remove_card(from_column as usize)?;
        let removed_card = removed.ok_or(GameError::EmptySource)?;
        self.freecells.place_card(to_cell as usize, removed_card)?;
        Ok(())
    }

    /// Executes a move from a freecell to a tableau column.
    ///
    /// This is a private helper function called by `execute_move`. It assumes
    /// the move has already been validated.
    ///
    /// # Arguments
    ///
    /// * `from_cell` - The 0-indexed source freecell.
    /// * `to_column` - The 0-indexed destination tableau column.
    /// * `m` - The `Move` being executed (used for re-validation).
    ///
    /// # Returns
    ///
    /// * `Ok(())` if the card was successfully moved.
    /// * `Err(GameError)` if an unexpected error occurs during component interaction.
    fn execute_freecell_to_tableau(
        &mut self,
        from_cell: u8,
        to_column: u8,
        m: &Move,
    ) -> Result<(), GameError> {
        let _card = *self
            .freecells
            .get_card(from_cell as usize)?
            .ok_or(GameError::InvalidMove("No card in freecell".to_string()))?;
        let _card = *self
            .freecells
            .get_card(from_cell as usize)?
            .ok_or(GameError::InvalidMove("No card in freecell".to_string()))?;
        self.is_move_valid(m)?;
        let removed = self.freecells.remove_card(from_cell as usize)?;
        let removed_card = removed.ok_or(GameError::EmptySource)?;
        self.tableau.place_card(to_column as usize, removed_card)?;
        Ok(())
    }

    /// Executes a move from a freecell to a foundation pile.
    ///
    /// This is a private helper function called by `execute_move`. It assumes
    /// the move has already been validated.
    ///
    /// # Arguments
    ///
    /// * `from_cell` - The 0-indexed source freecell.
    /// * `to_pile` - The 0-indexed destination foundation pile.
    /// * `m` - The `Move` being executed (used for re-validation).
    ///
    /// # Returns
    ///
    /// * `Ok(())` if the card was successfully moved.
    /// * `Err(GameError)` if an unexpected error occurs during component interaction.
    fn execute_freecell_to_foundation(
        &mut self,
        from_cell: u8,
        to_pile: u8,
        m: &Move,
    ) -> Result<(), GameError> {
        let _card = *self
            .freecells
            .get_card(from_cell as usize)?
            .ok_or(GameError::InvalidMove("No card in freecell".to_string()))?;
        self.is_move_valid(m)?;
        let removed = self.freecells.remove_card(from_cell as usize)?;
        let removed_card = removed.ok_or(GameError::EmptySource)?;
        self.foundations.place_card(to_pile as usize, removed_card)?;
        Ok(())
    }

    /// Executes a move from one tableau column to another.
    ///
    /// This is a private helper function called by `execute_move`. It assumes
    /// the move has already been validated.
    ///
    /// # Arguments
    ///
    /// * `from_column` - The 0-indexed source tableau column.
    /// * `to_column` - The 0-indexed destination tableau column.
    /// * `card_count` - The number of cards to move.
    /// * `m` - The `Move` being executed (used for re-validation).
    ///
    /// # Returns
    ///
    /// * `Ok(())` if the card(s) were successfully moved.
    /// * `Err(GameError)` if an unexpected error occurs during component interaction.
    fn execute_tableau_to_tableau(
        &mut self,
        from_column: u8,
        to_column: u8,
        card_count: u8,
        m: &Move,
    ) -> Result<(), GameError> {
        if card_count != 1 {
            return Err(GameError::InvalidMove(
                "Only single card moves supported".to_string(),
            ));
        }
        let _card = *self
            .tableau
            .get_card(from_column as usize)?
            .ok_or(GameError::EmptySource)?;
        self.is_move_valid(m)?;
        let removed = self.tableau.remove_card(from_column as usize)?;
        let removed_card = removed.ok_or(GameError::EmptySource)?;
        self.tableau.place_card(to_column as usize, removed_card)?;
        Ok(())
    }

    /// Undoes a move, reversing its effect on the game state.
    ///
    /// This method is primarily used by solver algorithms for backtracking.
    /// It assumes the move was previously executed and that the game state
    /// is in a valid condition to reverse the move.
    ///
    /// # Arguments
    ///
    /// * `m` - A reference to the `Move` to be undone.
    ///
    /// # Panics
    ///
    /// This method uses `expect()` on component operations, meaning it will panic
    /// if the game state is not as expected (e.g., trying to remove a card from
    /// an empty pile during undo). This is by design, as undo operations should
    /// only be called on states that were previously validly reached.
    ///
    /// # Examples
    ///
    /// ```
    /// use freecell_game_engine::{GameState, r#move::Move, Card, Rank, Suit};
    ///
    /// let mut game = GameState::new();
    /// // Assume game state is set up and a move is executed
    /// // game.tableau_mut().place_card(0, Card::new(Rank::Ace, Suit::Spades)).unwrap();
    /// let move_cmd = Move::TableauToFreecell { from_column: 0, to_cell: 0 };
    /// // game.execute_move(&move_cmd).unwrap();
    ///
    /// // Now, undo the move
    /// // game.undo_move(&move_cmd);
    /// // assert!(game.freecells().is_cell_empty(0).unwrap());
    /// // assert!(!game.tableau().get_card(0).unwrap().is_none());
    /// ```
    pub fn undo_move(&mut self, m: &Move) {
        use Move::*;
        match m {
            TableauToFoundation {
                from_column,
                to_pile,
            } => {
                let removed = self.foundations.remove_card(*to_pile as usize).expect("Undo: foundation error");
                let card = removed.expect("Undo: foundation not empty");
                self.tableau.place_card(*from_column as usize, card).expect("Undo: tableau error");
            }
            TableauToFreecell {
                from_column,
                to_cell,
            } => {
                let removed = self.freecells.remove_card(*to_cell as usize).expect("Undo: freecell error");
                let card = removed.expect("Undo: freecell not empty");
                self.tableau.place_card(*from_column as usize, card).expect("Undo: tableau error");
            }
            FreecellToTableau {
                from_cell,
                to_column,
            } => {
                let removed = self.tableau.remove_card(*to_column as usize).expect("Undo: tableau error");
                let card = removed.expect("Undo: tableau not empty");
                self.freecells.place_card(*from_cell as usize, card).expect("Undo: freecell error");
            }
            FreecellToFoundation { from_cell, to_pile } => {
                let removed = self.foundations.remove_card(*to_pile as usize).expect("Undo: foundation error");
                let card = removed.expect("Undo: foundation not empty");
                self.freecells.place_card(*from_cell as usize, card).expect("Undo: freecell error");
            }
            TableauToTableau {
                from_column,
                to_column,
                card_count,
            } => {
                assert_eq!(*card_count, 1, "Undo only supports single card moves");
                let removed = self.tableau.remove_card(*to_column as usize).expect("Undo: tableau error");
                let card = removed.expect("Undo: tableau not empty");
                self.tableau.place_card(*from_column as usize, card).expect("Undo: tableau error");
            }
        }
    }
}
