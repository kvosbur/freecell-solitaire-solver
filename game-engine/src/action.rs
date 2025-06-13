use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    TableauToFoundation { from_column: usize, to_pile: usize },
    TableauToFreecell { from_column: usize, to_cell: usize },
    FreecellToTableau { from_cell: usize, to_column: usize },
    FreecellToFoundation { from_cell: usize, to_pile: usize },
    TableauToTableau { from_column: usize, to_column: usize, card_count: usize },
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Action::TableauToFoundation { from_column, to_pile } => {
                write!(f, "Tableau {} to Foundation {}", from_column, to_pile)
            },
            Action::TableauToFreecell { from_column, to_cell } => {
                write!(f, "Tableau {} to Freecell {}", from_column, to_cell)
            },
            Action::FreecellToTableau { from_cell, to_column } => {
                write!(f, "Freecell {} to Tableau {}", from_cell, to_column)
            },
            Action::FreecellToFoundation { from_cell, to_pile } => {
                write!(f, "Freecell {} to Foundation {}", from_cell, to_pile)
            },
            Action::TableauToTableau { from_column, to_column, card_count } => {
                write!(f, "Tableau {} to Tableau {}, moving {} cards", from_column, to_column, card_count)
            },
        }
    }
}