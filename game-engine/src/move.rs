//! Move representation for FreeCell game transitions.
//!
//! This module defines the `Move` enum which represents all possible moves in a FreeCell game.
//! Moves are used by solvers to represent edges in the game state graph, where each game 
//! state is a node and each move is a transition between states.
//!
//! # FreeCell Game Overview
//! 
//! FreeCell is a solitaire card game played with a standard 52-card deck. The game area consists of:
//! 
//! - **8 Tableau columns** - Main playing area where cards are arranged in descending sequences
//! - **4 FreeCells** - Temporary storage slots for individual cards  
//! - **4 Foundation piles** - Goal areas where cards are built up by suit from Ace to King
//!
//! # Move Types
//!
//! All moves represent transferring cards between these areas according to FreeCell rules:
//! - Cards can be moved individually or in valid sequences
//! - Tableau stacking follows alternating colors and descending rank
//! - Foundation building follows same suit and ascending rank
//! - FreeCells can hold any single card temporarily
//!
//! # Examples
//!
//! ```
//! use freecell_game_engine::{GameState, Move};
//!
//! // Create a game state
//! let mut game = GameState::new();
//!
//! // Create a move
//! let move_cmd = Move::TableauToFreecell { from_column: 0, to_cell: 0 };
//!
//! // Validate the move
//! if game.is_move_valid(&move_cmd).is_ok() {
//!     // Execute the move
//!     game.execute_move(&move_cmd).unwrap();
//! }
//!
//! // Get all available moves for solver
//! let available_moves = game.get_available_moves();
//! println!("Found {} possible moves", available_moves.len());
//! ```

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Move {
    /// Move the top card from a tableau column to a foundation pile
    TableauToFoundation { 
        /// Source tableau column (0-7)
        from_column: u8, 
        /// Destination foundation pile (0-3)
        to_pile: u8 
    },
    
    /// Move the top card from a tableau column to an empty freecell
    TableauToFreecell { 
        /// Source tableau column (0-7)
        from_column: u8, 
        /// Destination freecell (0-3)
        to_cell: u8 
    },
    
    /// Move a card from a freecell to a tableau column
    FreecellToTableau { 
        /// Source freecell (0-3)
        from_cell: u8, 
        /// Destination tableau column (0-7)
        to_column: u8 
    },
    
    /// Move a card from a freecell to a foundation pile
    FreecellToFoundation { 
        /// Source freecell (0-3)
        from_cell: u8, 
        /// Destination foundation pile (0-3)
        to_pile: u8 
    },
    
    /// Move a sequence of cards between tableau columns
    TableauToTableau { 
        /// Source tableau column (0-7)
        from_column: u8, 
        /// Destination tableau column (0-7)
        to_column: u8, 
        /// Number of cards in the sequence (1 for single card)
        card_count: u8 
    },
}

impl Move {
    /// Returns the source location of this move.
    ///
    /// This method extracts where the card(s) are being moved from, which is useful
    /// for solver algorithms that need to analyze move patterns or check for conflicts.
    ///
    /// # Examples
    ///
    /// ```
    /// use freecell_game_engine::r#move::{Move, Location};
    ///
    /// let move_cmd = Move::TableauToFoundation { from_column: 3, to_pile: 1 };
    /// assert_eq!(move_cmd.source(), Location::Tableau(3));
    ///
    /// let freecell_move = Move::FreecellToTableau { from_cell: 2, to_column: 5 };
    /// assert_eq!(freecell_move.source(), Location::FreeCells(2));
    /// ```
    pub fn source(&self) -> Location {
        match self {
            Move::TableauToFoundation { from_column, .. } => Location::Tableau(*from_column),
            Move::TableauToFreecell { from_column, .. } => Location::Tableau(*from_column),
            Move::FreecellToTableau { from_cell, .. } => Location::FreeCells(*from_cell),
            Move::FreecellToFoundation { from_cell, .. } => Location::FreeCells(*from_cell),
            Move::TableauToTableau { from_column, .. } => Location::Tableau(*from_column),
        }
    }

    /// Returns the destination location of this move.
    ///
    /// This method extracts where the card(s) are being moved to, which is essential
    /// for solver algorithms that need to track state changes or validate move sequences.
    ///
    /// # Examples
    ///
    /// ```
    /// use freecell_game_engine::r#move::{Move, Location};
    ///
    /// let move_cmd = Move::TableauToFoundation { from_column: 3, to_pile: 1 };
    /// assert_eq!(move_cmd.destination(), Location::Foundation(1));
    ///
    /// let tableau_move = Move::FreecellToTableau { from_cell: 2, to_column: 5 };
    /// assert_eq!(tableau_move.destination(), Location::Tableau(5));
    /// ```
    pub fn destination(&self) -> Location {
        match self {
            Move::TableauToFoundation { to_pile, .. } => Location::Foundation(*to_pile),
            Move::TableauToFreecell { to_cell, .. } => Location::FreeCells(*to_cell),
            Move::FreecellToTableau { to_column, .. } => Location::Tableau(*to_column),
            Move::FreecellToFoundation { to_pile, .. } => Location::Foundation(*to_pile),
            Move::TableauToTableau { to_column, .. } => Location::Tableau(*to_column),
        }
    }

    /// Returns the type category of this move.
    ///
    /// This categorization is useful for solver heuristics, move ordering, and
    /// statistical analysis of solution paths. Foundation moves are typically
    /// prioritized as they represent progress toward the goal.
    ///
    /// # Examples
    ///
    /// ```
    /// use freecell_game_engine::r#move::{Move, MoveType};
    ///
    /// let foundation_move = Move::TableauToFoundation { from_column: 0, to_pile: 0 };
    /// assert_eq!(foundation_move.move_type(), MoveType::ToFoundation);
    ///
    /// let sequence_move = Move::TableauToTableau { 
    ///     from_column: 1, 
    ///     to_column: 2, 
    ///     card_count: 3 
    /// };
    /// assert_eq!(sequence_move.move_type(), MoveType::TableauSequence);
    /// ```
    pub fn move_type(&self) -> MoveType {
        match self {
            Move::TableauToFoundation { .. } | Move::FreecellToFoundation { .. } => MoveType::ToFoundation,
            Move::TableauToFreecell { .. } => MoveType::ToFreeCells,
            Move::FreecellToTableau { .. } => MoveType::ToTableau,
            Move::TableauToTableau { .. } => MoveType::TableauSequence,
        }
    }

    /// Returns true if this move places a card on a foundation pile.
    ///
    /// Foundation moves represent direct progress toward winning the game,
    /// making them valuable for solver heuristics and move prioritization.
    /// These moves are typically irreversible in optimal play.
    ///
    /// # Examples
    ///
    /// ```
    /// use freecell_game_engine::r#move::Move;
    ///
    /// let foundation_move = Move::TableauToFoundation { from_column: 0, to_pile: 0 };
    /// assert!(foundation_move.is_foundation_move());
    ///
    /// let freecell_move = Move::TableauToFreecell { from_column: 0, to_cell: 0 };
    /// assert!(!freecell_move.is_foundation_move());
    /// ```
    pub fn is_foundation_move(&self) -> bool {
        matches!(self, Move::TableauToFoundation { .. } | Move::FreecellToFoundation { .. })
    }

    /// Returns true if this move involves the specified tableau column.
    ///
    /// This method is useful for solver algorithms that need to:
    /// - Detect move conflicts (multiple moves affecting the same column)
    /// - Analyze column-specific patterns
    /// - Implement move ordering based on column priorities
    ///
    /// # Arguments
    ///
    /// * `column` - The tableau column index to check (0-7)
    ///
    /// # Examples
    ///
    /// ```
    /// use freecell_game_engine::r#move::Move;
    ///
    /// let move_cmd = Move::TableauToTableau { 
    ///     from_column: 2, 
    ///     to_column: 5, 
    ///     card_count: 1 
    /// };
    /// 
    /// assert!(move_cmd.affects_tableau_column(2));  // Source column
    /// assert!(move_cmd.affects_tableau_column(5));  // Destination column
    /// assert!(!move_cmd.affects_tableau_column(3)); // Unrelated column
    ///
    /// let foundation_move = Move::TableauToFoundation { from_column: 1, to_pile: 0 };
    /// assert!(foundation_move.affects_tableau_column(1));
    /// assert!(!foundation_move.affects_tableau_column(2));
    /// ```
    pub fn affects_tableau_column(&self, column: u8) -> bool {
        match self {
            Move::TableauToFoundation { from_column, .. } => *from_column == column,
            Move::TableauToFreecell { from_column, .. } => *from_column == column,
            Move::FreecellToTableau { to_column, .. } => *to_column == column,
            Move::TableauToTableau { from_column, to_column, .. } => {
                *from_column == column || *to_column == column
            }
            _ => false,
        }
    }

    /// Returns true if this move involves the specified freecell.
    ///
    /// This method helps solver algorithms track freecell usage patterns,
    /// which is crucial for FreeCell strategy since freecells are limited
    /// temporary storage that can become bottlenecks.
    ///
    /// # Arguments
    ///
    /// * `cell` - The freecell index to check (0-3)
    ///
    /// # Examples
    ///
    /// ```
    /// use freecell_game_engine::r#move::Move;
    ///
    /// let to_freecell = Move::TableauToFreecell { from_column: 0, to_cell: 2 };
    /// assert!(to_freecell.affects_freecell(2));
    /// assert!(!to_freecell.affects_freecell(1));
    ///
    /// let from_freecell = Move::FreecellToTableau { from_cell: 1, to_column: 3 };
    /// assert!(from_freecell.affects_freecell(1));
    /// assert!(!from_freecell.affects_freecell(2));
    ///
    /// let tableau_move = Move::TableauToTableau { 
    ///     from_column: 0, 
    ///     to_column: 1, 
    ///     card_count: 1 
    /// };
    /// assert!(!tableau_move.affects_freecell(0)); // Doesn't involve freecells
    /// ```
    pub fn affects_freecell(&self, cell: u8) -> bool {
        match self {
            Move::TableauToFreecell { to_cell, .. } => *to_cell == cell,
            Move::FreecellToTableau { from_cell, .. } => *from_cell == cell,
            Move::FreecellToFoundation { from_cell, .. } => *from_cell == cell,
            _ => false,
        }
    }

    /// Returns the number of cards moved by this action.
    ///
    /// Most moves transfer a single card, but tableau-to-tableau moves can
    /// transfer sequences of properly ordered cards. This information is
    /// important for:
    /// - Calculating move complexity
    /// - Validating sequence moves
    /// - Solver heuristics based on move efficiency
    ///
    /// # Examples
    ///
    /// ```
    /// use freecell_game_engine::r#move::Move;
    ///
    /// // Single card moves
    /// let single_move = Move::TableauToFoundation { from_column: 0, to_pile: 0 };
    /// assert_eq!(single_move.card_count(), 1);
    ///
    /// let freecell_move = Move::FreecellToTableau { from_cell: 0, to_column: 1 };
    /// assert_eq!(freecell_move.card_count(), 1);
    ///
    /// // Sequence move
    /// let sequence_move = Move::TableauToTableau { 
    ///     from_column: 0, 
    ///     to_column: 1, 
    ///     card_count: 4 
    /// };
    /// assert_eq!(sequence_move.card_count(), 4);
    /// ```
    pub fn card_count(&self) -> u8 {
        match self {
            Move::TableauToTableau { card_count, .. } => *card_count,
            _ => 1,
        }
    }

    /// Returns true if this move transfers multiple cards in a sequence.
    ///
    /// Sequence moves are more complex and can significantly change the game state.
    /// This method helps solvers identify high-impact moves that may require
    /// special consideration in search algorithms.
    ///
    /// # Examples
    ///
    /// ```
    /// use freecell_game_engine::r#move::Move;
    ///
    /// let single_move = Move::TableauToTableau { 
    ///     from_column: 0, 
    ///     to_column: 1, 
    ///     card_count: 1 
    /// };
    /// assert!(!single_move.is_sequence_move());
    ///
    /// let sequence_move = Move::TableauToTableau { 
    ///     from_column: 0, 
    ///     to_column: 1, 
    ///     card_count: 3 
    /// };
    /// assert!(sequence_move.is_sequence_move());
    ///
    /// let foundation_move = Move::TableauToFoundation { from_column: 0, to_pile: 0 };
    /// assert!(!foundation_move.is_sequence_move()); // Always single card
    /// ```
    pub fn is_sequence_move(&self) -> bool {
        matches!(self, Move::TableauToTableau { card_count, .. } if *card_count > 1)
    }
}

/// Represents a location in the game where cards can be placed.
///
/// This enum is used to identify the source and destination of moves,
/// enabling solver algorithms to analyze move patterns and detect conflicts.
/// Each variant contains the specific index within that area type.
///
/// # Examples
///
/// ```
/// use freecell_game_engine::r#move::Location;
///
/// // Tableau column 3 (0-indexed, so the 4th column)
/// let tableau_loc = Location::Tableau(3);
///
/// // FreeCells slot 1 (0-indexed, so the 2nd freecell)
/// let freecell_loc = Location::FreeCells(1);
///
/// // Foundation pile 0 (0-indexed, so the 1st foundation pile)
/// let foundation_loc = Location::Foundation(0);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Location {
    /// A tableau column (0-7)
    /// 
    /// Tableau columns are the main playing area where cards are stacked
    /// in descending rank with alternating colors.
    Tableau(u8),
    
    /// A freecell (0-3)
    /// 
    /// FreeCells are temporary storage slots that can hold any single card.
    /// They are crucial for maneuvering cards in complex situations.
    FreeCells(u8),
    
    /// A foundation pile (0-3)
    /// 
    /// Foundation piles are the goal areas where cards are built up by suit
    /// from Ace to King. Completing all four foundations wins the game.
    Foundation(u8),
}

/// Categorizes moves by their destination type.
///
/// This categorization is essential for solver heuristics and move ordering.
/// Different move types have different strategic values and computational costs.
///
/// # Examples
///
/// ```
/// use freecell_game_engine::r#move::{Move, MoveType};
///
/// let move_cmd = Move::TableauToFoundation { from_column: 0, to_pile: 0 };
/// match move_cmd.move_type() {
///     MoveType::ToFoundation => println!("Progress move!"),
///     MoveType::ToFreeCells => println!("Temporary storage"),
///     MoveType::ToTableau => println!("Rearrangement move"),
///     MoveType::TableauSequence => println!("Complex sequence move"),
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MoveType {
    /// Moves that place cards on foundation piles
    /// 
    /// These moves represent direct progress toward winning and are typically
    /// prioritized by solvers. Foundation moves are usually irreversible.
    ToFoundation,
    
    /// Moves that place cards in freecells
    /// 
    /// These moves use limited temporary storage and should be used strategically.
    /// Excessive freecell usage can lead to deadlock situations.
    ToFreeCells,
    
    /// Moves that place cards on tableau columns
    /// 
    /// These moves rearrange the tableau to expose hidden cards or create
    /// better stacking opportunities.
    ToTableau,
    
    /// Moves that transfer card sequences between tableau columns
    /// 
    /// These are the most complex moves, potentially moving multiple cards
    /// at once and significantly changing the game state.
    TableauSequence,
}

impl std::fmt::Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Move::TableauToFoundation { from_column, to_pile } => {
                write!(f, "Tableau {} → Foundation {}", from_column, to_pile)
            }
            Move::TableauToFreecell { from_column, to_cell } => {
                write!(f, "Tableau {} → FreeCells {}", from_column, to_cell)
            }
            Move::FreecellToTableau { from_cell, to_column } => {
                write!(f, "FreeCells {} → Tableau {}", from_cell, to_column)
            }
            Move::FreecellToFoundation { from_cell, to_pile } => {
                write!(f, "FreeCells {} → Foundation {}", from_cell, to_pile)
            }
            Move::TableauToTableau { from_column, to_column, card_count } => {
                if *card_count == 1 {
                    write!(f, "Tableau {} → Tableau {}", from_column, to_column)
                } else {
                    write!(f, "Tableau {} → Tableau {} ({} cards)", from_column, to_column, card_count)
                }
            }
        }
    }
}
