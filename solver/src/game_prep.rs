use std::str::MatchIndices;

use freecell_game_engine::game_state::Move;

pub fn get_game_solution(seed: u64) -> Vec<Move> {
    match seed {
        1 => vec![
            Move::TableauToFreecell {
                from_column: 5,
                to_cell: 0,
            }, // 1
            Move::TableauToFreecell {
                from_column: 5,
                to_cell: 1,
            }, // 2
            Move::TableauToFoundation {
                from_column: 5,
                to_pile: 0, // club
            }, // 3
            Move::FreecellToFoundation {
                from_cell: 1,
                to_pile: 0,
            }, // 4
            Move::TableauToFoundation {
                from_column: 5,
                to_pile: 1, // Spades
            }, // 5
            Move::TableauToFreecell {
                from_column: 6,
                to_cell: 1,
            }, // 6
            Move::TableauToTableau {
                from_column: 6,
                to_column: 5,
                card_count: 1,
            }, // 7
            Move::TableauToFreecell {
                from_column: 6,
                to_cell: 2,
            }, // 8
            Move::TableauToFoundation {
                from_column: 6,
                to_pile: 2, // hearts
            }, // 9
            Move::TableauToFoundation {
                from_column: 2,
                to_pile: 2,
            }, // 10
            Move::TableauToFreecell {
                from_column: 5,
                to_cell: 3,
            }, // 11
            Move::TableauToTableau {
                from_column: 5,
                to_column: 6,
                card_count: 1,
            }, // 11
            Move::FreecellToTableau {
                from_cell: 3,
                to_column: 6,
            }, // 11
            Move::TableauToTableau {
                from_column: 2,
                to_column: 1,
                card_count: 1,
            }, // 12
            Move::TableauToFreecell {
                from_column: 2,
                to_cell: 3,
            }, // 13
            Move::TableauToTableau {
                from_column: 2,
                to_column: 6,
                card_count: 1,
            }, // 14
            Move::TableauToTableau {
                from_column: 2,
                to_column: 6,
                card_count: 1,
            }, // 15
            Move::FreecellToTableau {
                from_cell: 1,
                to_column: 6,
            }, // 16
            Move::TableauToFreecell {
                from_column: 7,
                to_cell: 1,
            }, // 17
            Move::TableauToTableau {
                from_column: 7,
                to_column: 6,
                card_count: 1,
            }, // 18
            Move::TableauToTableau {
                from_column: 4,
                to_column: 6,
                card_count: 1,
            }, // 19
            Move::TableauToTableau {
                from_column: 4,
                to_column: 2,
                card_count: 1,
            }, // 20
            Move::TableauToTableau {
                from_column: 7,
                to_column: 2,
                card_count: 1,
            }, // 21
            Move::TableauToFoundation {
                from_column: 7,
                to_pile: 0,
            }, // 22
            Move::TableauToTableau {
                from_column: 0,
                to_column: 5,
                card_count: 1,
            }, // 23
            Move::TableauToTableau {
                from_column: 0,
                to_column: 2,
                card_count: 1,
            }, // 24
            Move::TableauToTableau {
                from_column: 0,
                to_column: 4,
                card_count: 1,
            }, // 25
            Move::TableauToFoundation {
                from_column: 0,
                to_pile: 0,
            }, // 26
            Move::TableauToFoundation {
                from_column: 0,
                to_pile: 1,
            }, // 27
            Move::TableauToFoundation {
                from_column: 7,
                to_pile: 2,
            }, // 28
            Move::TableauToFoundation {
                from_column: 4,
                to_pile: 1,
            }, // 29
            Move::TableauToFoundation {
                from_column: 4,
                to_pile: 2,
            }, // 30
            Move::FreecellToFoundation {
                from_cell: 3,
                to_pile: 1,
            }, // 31
            Move::TableauToFoundation {
                from_column: 7,
                to_pile: 2,
            }, // 32
            Move::TableauToFoundation {
                from_column: 3,
                to_pile: 2,
            }, // 33
            Move::TableauToTableau {
                from_column: 3,
                to_column: 0,
                card_count: 1,
            }, // 34
            Move::TableauToTableau {
                from_column: 3,
                to_column: 7,
                card_count: 1,
            }, // 35
            Move::TableauToFreecell {
                from_column: 1,
                to_cell: 3,
            }, // 36
            Move::TableauToTableau {
                from_column: 1,
                to_column: 7,
                card_count: 1,
            }, // 36
            Move::FreecellToTableau {
                from_cell: 3,
                to_column: 7,
            }, // 36
            Move::TableauToTableau {
                from_column: 4,
                to_column: 3,
                card_count: 1,
            }, // 37
            Move::TableauToFoundation {
                from_column: 4,
                to_pile: 3, //  diamonds
            }, // 38
            Move::TableauToTableau {
                from_column: 4,
                to_column: 6,
                card_count: 1,
            }, // 39
            Move::TableauToTableau {
                from_column: 1,
                to_column: 4,
                card_count: 1,
            }, // 40
            Move::TableauToTableau {
                from_column: 1,
                to_column: 3,
                card_count: 1,
            }, // 41
            Move::TableauToFoundation {
                from_column: 1,
                to_pile: 0,
            }, // 42
            Move::TableauToFreecell {
                from_column: 5,
                to_cell: 3,
            }, // 43
            Move::TableauToTableau {
                from_column: 5,
                to_column: 4,
                card_count: 1,
            }, // 43
            Move::FreecellToTableau {
                from_cell: 3,
                to_column: 4,
            }, // 43
            Move::TableauToTableau {
                from_column: 1,
                to_column: 5,
                card_count: 1,
            }, // 44
            Move::TableauToFreecell {
                from_column: 1,
                to_cell: 3,
            }, // 45
            Move::TableauToFoundation {
                from_column: 1,
                to_pile: 3,
            }, // 46
            Move::FreecellToFoundation {
                from_cell: 0,
                to_pile: 3,
            }, // 47
            Move::FreecellToFoundation {
                from_cell: 2,
                to_pile: 3,
            }, // 48
            Move::TableauToFoundation {
                from_column: 6,
                to_pile: 3,
            }, // 49
            Move::TableauToFoundation {
                from_column: 6,
                to_pile: 0,
            }, // 50
            Move::TableauToFreecell {
                from_column: 3,
                to_cell: 0,
            }, // 51
            Move::TableauToFreecell {
                from_column: 3,
                to_cell: 2,
            }, // 51
            Move::TableauToTableau {
                from_column: 3,
                to_column: 5,
                card_count: 1,
            }, // 51
            Move::FreecellToTableau {
                from_cell: 2,
                to_column: 5,
            }, // 51
            Move::FreecellToTableau {
                from_cell: 0,
                to_column: 5,
            }, // 51
            Move::TableauToFreecell {
                from_column: 3,
                to_cell: 0,
            }, // 52
            Move::TableauToFoundation {
                from_column: 3,
                to_pile: 1,
            }, // 53
            Move::TableauToFoundation {
                from_column: 2,
                to_pile: 3,
            }, // 54
            Move::TableauToFoundation {
                from_column: 4,
                to_pile: 1,
            }, // 55
            Move::TableauToFoundation {
                from_column: 2,
                to_pile: 1,
            }, // 56
            Move::TableauToFoundation {
                from_column: 4,
                to_pile: 2,
            }, // 57
            Move::TableauToFoundation {
                from_column: 6,
                to_pile: 3,
            }, // 58
            Move::TableauToFoundation {
                from_column: 4,
                to_pile: 1,
            }, // 59
            Move::TableauToFreecell {
                from_column: 0,
                to_cell: 2,
            }, // 60
            Move::TableauToTableau {
                from_column: 0,
                to_column: 4,
                card_count: 1,
            }, // 60
            Move::FreecellToTableau {
                from_cell: 2,
                to_column: 4,
            }, // 60
            Move::TableauToFreecell {
                from_column: 6,
                to_cell: 2,
            }, // 61  // moved 8
            Move::FreecellToTableau {
                from_cell: 1,
                to_column: 0,
            }, // 61
            Move::TableauToTableau {
                from_column: 6,
                to_column: 0,
                card_count: 1,
            }, // 61  // moved 9
            Move::FreecellToTableau {
                from_cell: 3,
                to_column: 1,
            }, // 61
            Move::FreecellToTableau {
                from_cell: 0,
                to_column: 1,
            }, // 61
            Move::TableauToFreecell {
                from_column: 6,
                to_cell: 0,
            }, // 61  // moved 10
            Move::TableauToTableau {
                from_column: 6,
                to_column: 4,
                card_count: 1,
            }, // 61// moved jack
            Move::TableauToFreecell {
                from_column: 6,
                to_cell: 3,
            }, // 61 // moved queen
            Move::TableauToFreecell {
                from_column: 6,
                to_cell: 1,
            }, // 61 // moved king
            Move::TableauToFoundation {
                from_column: 6,
                to_pile: 0,
            }, // 62
            Move::FreecellToFoundation {
                from_cell: 2,
                to_pile: 0,
            }, // 63
            Move::TableauToFoundation {
                from_column: 7,
                to_pile: 3,
            }, // 64
            Move::TableauToFoundation {
                from_column: 2,
                to_pile: 2,
            }, // 65
            Move::TableauToFoundation {
                from_column: 7,
                to_pile: 0,
            }, // 66
            Move::TableauToFoundation {
                from_column: 2,
                to_pile: 1,
            }, // 67
            Move::TableauToFoundation {
                from_column: 0,
                to_pile: 3,
            }, // 68
            Move::TableauToFoundation {
                from_column: 2,
                to_pile: 2,
            }, // 69
            Move::FreecellToFoundation {
                from_cell: 0,
                to_pile: 1,
            }, // 70
            Move::TableauToFoundation {
                from_column: 5,
                to_pile: 3,
            }, // 71
            Move::TableauToFoundation {
                from_column: 0,
                to_pile: 0,
            }, // 72
            Move::TableauToFoundation {
                from_column: 7,
                to_pile: 2,
            }, // 73
            Move::TableauToFoundation {
                from_column: 5,
                to_pile: 1,
            }, // 74
            Move::TableauToFoundation {
                from_column: 4,
                to_pile: 2,
            }, // 75
            Move::TableauToFoundation {
                from_column: 3,
                to_pile: 0,
            }, // 76
            Move::TableauToFoundation {
                from_column: 0,
                to_pile: 3,
            }, // 77
            Move::FreecellToFoundation {
                from_cell: 3,
                to_pile: 0,
            }, // 78
            Move::TableauToFoundation {
                from_column: 5,
                to_pile: 2,
            }, // 79
            Move::TableauToFoundation {
                from_column: 4,
                to_pile: 1,
            }, // 80
            Move::TableauToFoundation {
                from_column: 1,
                to_pile: 3,
            }, // 81
            Move::FreecellToFoundation {
                from_cell: 1,
                to_pile: 2,
            }, // 82
            Move::TableauToFoundation {
                from_column: 5,
                to_pile: 1,
            }, // 83
            Move::TableauToFoundation {
                from_column: 4,
                to_pile: 3,
            }, // 84
            Move::TableauToFoundation {
                from_column: 1,
                to_pile: 0,
            }, // 85
        ],
        _ => panic!("No solution for this seed"),
    }
}
