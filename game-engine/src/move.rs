use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Move {
    TableauToFoundation { from_column: usize, to_pile: usize },
    TableauToFreecell { from_column: usize, to_cell: usize },
    FreecellToTableau { from_cell: usize, to_column: usize },
    FreecellToFoundation { from_cell: usize, to_pile: usize },
    TableauToTableau { from_column: usize, to_column: usize, card_count: usize },
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Move::TableauToFoundation { from_column, to_pile } => {
                write!(f, "Tableau {} to Foundation {}", from_column, to_pile)
            },
            Move::TableauToFreecell { from_column, to_cell } => {
                write!(f, "Tableau {} to Freecell {}", from_column, to_cell)
            },
            Move::FreecellToTableau { from_cell, to_column } => {
                write!(f, "Freecell {} to Tableau {}", from_cell, to_column)
            },
            Move::FreecellToFoundation { from_cell, to_pile } => {
                write!(f, "Freecell {} to Foundation {}", from_cell, to_pile)
            },
            Move::TableauToTableau { from_column, to_column, card_count } => {
                write!(f, "Tableau {} to Tableau {}, moving {} cards", from_column, to_column, card_count)
            },
        }
    }
}