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
    /// use freecell_game_engine::{GameState, Move, Card, Rank, Suit};
    /// use freecell_game_engine::location::{TableauLocation, FreecellLocation};
    ///
    /// let mut game = GameState::new();
    /// // Assume game state is set up such that this move is valid.
    /// // For example, by dealing a specific game or manually placing cards.
    ///
    /// // Example: Define a move from Tableau column 0 to Freecell 0.
    /// let move_cmd = Move::tableau_to_freecell(0, 0).unwrap();
    ///
    /// // If the move is valid, execute it.
    /// if let Err(err) = game.execute_move(&move_cmd) {
    ///   println!("Error executing move: {}", err);
    /// }
    /// ```
    pub fn execute_move(&mut self, m: &Move) -> Result<(), GameError> {
        use crate::location::Location::*;
        match (m.source, m.destination) {
            (Tableau(from), Foundation(to)) => {
                self.execute_tableau_to_foundation(from.index(), to.index(), m)
            }
            (Tableau(from), Freecell(to)) => {
                self.execute_tableau_to_freecell(from.index(), to.index(), m)
            }
            (Freecell(from), Tableau(to)) => {
                self.execute_freecell_to_tableau(from.index(), to.index(), m)
            }
            (Freecell(from), Foundation(to)) => {
                self.execute_freecell_to_foundation(from.index(), to.index(), m)
            }
            (Tableau(from), Tableau(to)) => {
                self.execute_tableau_to_tableau(from.index(), to.index(), m.card_count, m)
            }
            _ => Err(GameError::InvalidMove {
                reason: "Moves between these locations are not supported".to_string(),
                attempted_move: *m,
            }),
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
        self.is_move_valid(m)?;
        let from_location = crate::location::TableauLocation::new(from_column).map_err(GameError::Location)?;
        let removed = self.tableau.remove_card(from_location).map_err(|e| GameError::Tableau {
            error: e,
            attempted_move: Some(*m),
            operation: "execute_tableau_to_foundation".to_string(),
        })?;
        let removed_card = removed.ok_or_else(|| GameError::InvalidMove {
            reason: "Source tableau column is empty".to_string(),
            attempted_move: *m,
        })?;
        let to_location = crate::location::FoundationLocation::new(to_pile).map_err(GameError::Location)?;
        self.foundations
            .place_card(to_location, removed_card)
            .map_err(|e| GameError::Foundation {
                error: e,
                attempted_move: Some(*m),
                operation: "execute_tableau_to_foundation".to_string(),
            })?;
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
        self.is_move_valid(m)?;
        let from_location = crate::location::TableauLocation::new(from_column).map_err(GameError::Location)?;
        let removed = self.tableau.remove_card(from_location).map_err(|e| GameError::Tableau {
            error: e,
            attempted_move: Some(*m),
            operation: "execute_tableau_to_freecell".to_string(),
        })?;
        let removed_card = removed.ok_or_else(|| GameError::InvalidMove {
            reason: "Source tableau column is empty".to_string(),
            attempted_move: *m,
        })?;
        let to_location = crate::location::FreecellLocation::new(to_cell).map_err(GameError::Location)?;
        self.freecells
            .place_card(to_location, removed_card)
            .map_err(|e| GameError::FreeCell {
                error: e,
                attempted_move: Some(*m),
                operation: "execute_tableau_to_freecell".to_string(),
            })?;
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
        self.is_move_valid(m)?;
        let from_location = crate::location::FreecellLocation::new(from_cell).map_err(GameError::Location)?;
        let removed = self.freecells.remove_card(from_location).map_err(|e| GameError::FreeCell {
            error: e,
            attempted_move: Some(*m),
            operation: "execute_freecell_to_tableau".to_string(),
        })?;
        let removed_card = removed.ok_or_else(|| GameError::InvalidMove {
            reason: "Source freecell is empty".to_string(),
            attempted_move: *m,
        })?;
        let to_location = crate::location::TableauLocation::new(to_column).map_err(GameError::Location)?;
        self.tableau
            .place_card(to_location, removed_card)
            .map_err(|e| GameError::Tableau {
                error: e,
                attempted_move: Some(*m),
                operation: "execute_freecell_to_tableau".to_string(),
            })?;
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
        self.is_move_valid(m)?;
        let from_location = crate::location::FreecellLocation::new(from_cell).map_err(GameError::Location)?;
        let removed = self.freecells.remove_card(from_location).map_err(|e| GameError::FreeCell {
            error: e,
            attempted_move: Some(*m),
            operation: "execute_freecell_to_foundation".to_string(),
        })?;
        let removed_card = removed.ok_or_else(|| GameError::InvalidMove {
            reason: "Source freecell is empty".to_string(),
            attempted_move: *m,
        })?;
        let to_location = crate::location::FoundationLocation::new(to_pile).map_err(GameError::Location)?;
        self.foundations
            .place_card(to_location, removed_card)
            .map_err(|e| GameError::Foundation {
                error: e,
                attempted_move: Some(*m),
                operation: "execute_freecell_to_foundation".to_string(),
            })?;
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
            return Err(GameError::OnlySingleCardMovesSupported);
        }
        self.is_move_valid(m)?;
        let from_location = crate::location::TableauLocation::new(from_column).map_err(GameError::Location)?;
        let removed = self.tableau.remove_card(from_location).map_err(|e| GameError::Tableau {
            error: e,
            attempted_move: Some(*m),
            operation: "execute_tableau_to_tableau".to_string(),
        })?;
        let removed_card = removed.ok_or_else(|| GameError::InvalidMove {
            reason: "Source tableau column is empty".to_string(),
            attempted_move: *m,
        })?;
        let to_location = crate::location::TableauLocation::new(to_column).map_err(GameError::Location)?;
        self.tableau
            .place_card(to_location, removed_card)
            .map_err(|e| GameError::Tableau {
                error: e,
                attempted_move: Some(*m),
                operation: "execute_tableau_to_tableau".to_string(),
            })?;
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
    /// use freecell_game_engine::{GameState, Move, Card, Rank, Suit};
    /// use freecell_game_engine::location::{TableauLocation, FreecellLocation};
    ///
    /// let mut game = GameState::new();
    /// // Assume game state is set up and a move has been executed.
    /// let move_cmd = Move::tableau_to_freecell(0, 0).unwrap();
    /// if game.execute_move(&move_cmd).is_ok() {
    ///    // Now, undo the move.
    ///    game.undo_move(&move_cmd);
    /// }
    ///
    /// // The game state should now be reverted.
    /// // let location = FreecellLocation::new(0).unwrap();
    /// // assert!(game.freecells().is_cell_empty(location).unwrap());
    /// // assert!(!game.tableau().get_card(TableauLocation::new(0).unwrap()).unwrap().is_none());
    /// ```
    pub fn undo_move(&mut self, m: &Move) {
        use crate::location::Location::*;
        match (m.source, m.destination) {
            (Tableau(from), Foundation(to)) => {
                let to_location = crate::location::FoundationLocation::new(to.index()).unwrap();
                let removed = self.foundations.remove_card(to_location).expect("Undo: foundation error");
                let card = removed.expect("Undo: foundation not empty");
                let from_location = crate::location::TableauLocation::new(from.index()).unwrap();
                self.tableau.place_card(from_location, card).expect("Undo: tableau error");
            }
            (Tableau(from), Freecell(to)) => {
                let to_location = crate::location::FreecellLocation::new(to.index()).unwrap();
                let removed = self.freecells.remove_card(to_location).expect("Undo: freecell error");
                let card = removed.expect("Undo: freecell not empty");
                let from_location = crate::location::TableauLocation::new(from.index()).unwrap();
                self.tableau.place_card(from_location, card).expect("Undo: tableau error");
            }
            (Freecell(from), Tableau(to)) => {
                let to_location = crate::location::TableauLocation::new(to.index()).unwrap();
                let removed = self.tableau.remove_card(to_location).expect("Undo: tableau error");
                let card = removed.expect("Undo: tableau not empty");
                let from_location = crate::location::FreecellLocation::new(from.index()).unwrap();
                self.freecells.place_card(from_location, card).expect("Undo: freecell error");
            }
            (Freecell(from), Foundation(to)) => {
                let to_location = crate::location::FoundationLocation::new(to.index()).unwrap();
                let removed = self.foundations.remove_card(to_location).expect("Undo: foundation error");
                let card = removed.expect("Undo: foundation not empty");
                let from_location = crate::location::FreecellLocation::new(from.index()).unwrap();
                self.freecells.place_card(from_location, card).expect("Undo: freecell error");
            }
            (Tableau(from), Tableau(to)) => {
                assert_eq!(m.card_count, 1, "Undo only supports single card moves");
                let to_location = crate::location::TableauLocation::new(to.index()).unwrap();
                let removed = self.tableau.remove_card(to_location).expect("Undo: tableau error");
                let card = removed.expect("Undo: tableau not empty");
                let from_location = crate::location::TableauLocation::new(from.index()).unwrap();
                self.tableau.place_card(from_location, card).expect("Undo: tableau error");
            }
            _ => {}
        }
    }
}
