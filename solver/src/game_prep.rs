use freecell_game_engine::action::Action;

pub fn get_game_solution(seed: u64) -> Vec<Action> {
    match seed {
        1 => vec![
            Action::TableauToFreecell {
                from_column: 5,
                to_cell: 0,
            }, // 1
            Action::TableauToFreecell {
                from_column: 5,
                to_cell: 1,
            }, // 2
            Action::TableauToFoundation {
                from_column: 5,
                to_pile: 0, // club
            }, // 3
            Action::FreecellToFoundation {
                from_cell: 1,
                to_pile: 0,
            }, // 4
            Action::TableauToFoundation {
                from_column: 5,
                to_pile: 1, // Spades
            }, // 5
            Action::TableauToFreecell {
                from_column: 6,
                to_cell: 1,
            }, // 6
            Action::TableauToTableau {
                from_column: 6,
                to_column: 5,
                card_count: 1,
            }, // 7
            Action::TableauToFreecell {
                from_column: 6,
                to_cell: 2,
            }, // 8
            Action::TableauToFoundation {
                from_column: 6,
                to_pile: 2, // hearts
            }, // 9
            Action::TableauToFoundation {
                from_column: 2,
                to_pile: 2,
            }, // 10
            Action::TableauToFreecell {
                from_column: 5,
                to_cell: 3,
            }, // 11
            Action::TableauToTableau {
                from_column: 5,
                to_column: 6,
                card_count: 1,
            }, // 11
            Action::FreecellToTableau {
                from_cell: 3,
                to_column: 6,
            }, // 11
            Action::TableauToTableau {
                from_column: 2,
                to_column: 1,
                card_count: 1,
            }, // 12
            Action::TableauToFreecell {
                from_column: 2,
                to_cell: 3,
            }, // 13
            Action::TableauToTableau {
                from_column: 2,
                to_column: 6,
                card_count: 1,
            }, // 14
            Action::TableauToTableau {
                from_column: 2,
                to_column: 6,
                card_count: 1,
            }, // 15
            Action::FreecellToTableau {
                from_cell: 1,
                to_column: 6,
            }, // 16
            Action::TableauToFreecell {
                from_column: 7,
                to_cell: 1,
            }, // 17
            Action::TableauToTableau {
                from_column: 7,
                to_column: 6,
                card_count: 1,
            }, // 18
            Action::TableauToTableau {
                from_column: 4,
                to_column: 6,
                card_count: 1,
            }, // 19
            Action::TableauToTableau {
                from_column: 4,
                to_column: 2,
                card_count: 1,
            }, // 20
            Action::TableauToTableau {
                from_column: 7,
                to_column: 2,
                card_count: 1,
            }, // 21
            Action::TableauToFoundation {
                from_column: 7,
                to_pile: 0,
            }, // 22
            Action::TableauToTableau {
                from_column: 0,
                to_column: 5,
                card_count: 1,
            }, // 23
            Action::TableauToTableau {
                from_column: 0,
                to_column: 2,
                card_count: 1,
            }, // 24
            Action::TableauToTableau {
                from_column: 0,
                to_column: 4,
                card_count: 1,
            }, // 25
            Action::TableauToFoundation {
                from_column: 0,
                to_pile: 0,
            }, // 26
            Action::TableauToFoundation {
                from_column: 0,
                to_pile: 1,
            }, // 27
            Action::TableauToFoundation {
                from_column: 7,
                to_pile: 2,
            }, // 28
            Action::TableauToFoundation {
                from_column: 4,
                to_pile: 1,
            }, // 29
            Action::TableauToFoundation {
                from_column: 4,
                to_pile: 2,
            }, // 30
            Action::FreecellToFoundation {
                from_cell: 3,
                to_pile: 1,
            }, // 31
            Action::TableauToFoundation {
                from_column: 7,
                to_pile: 2,
            }, // 32
            Action::TableauToFoundation {
                from_column: 3,
                to_pile: 2,
            }, // 33
            Action::TableauToTableau {
                from_column: 3,
                to_column: 0,
                card_count: 1,
            }, // 34
            Action::TableauToTableau {
                from_column: 3,
                to_column: 7,
                card_count: 1,
            }, // 35
            Action::TableauToFreecell {
                from_column: 1,
                to_cell: 3,
            }, // 36
            Action::TableauToTableau {
                from_column: 1,
                to_column: 7,
                card_count: 1,
            }, // 36
            Action::FreecellToTableau {
                from_cell: 3,
                to_column: 7,
            }, // 36
            Action::TableauToTableau {
                from_column: 4,
                to_column: 3,
                card_count: 1,
            }, // 37
            Action::TableauToFoundation {
                from_column: 4,
                to_pile: 3, //  diamonds
            }, // 38
            Action::TableauToTableau {
                from_column: 4,
                to_column: 6,
                card_count: 1,
            }, // 39
            Action::TableauToTableau {
                from_column: 1,
                to_column: 4,
                card_count: 1,
            }, // 40
            Action::TableauToTableau {
                from_column: 1,
                to_column: 3,
                card_count: 1,
            }, // 41
            Action::TableauToFoundation {
                from_column: 1,
                to_pile: 0,
            }, // 42
            Action::TableauToFreecell {
                from_column: 5,
                to_cell: 3,
            }, // 43
            Action::TableauToTableau {
                from_column: 5,
                to_column: 4,
                card_count: 1,
            }, // 43
            Action::FreecellToTableau {
                from_cell: 3,
                to_column: 4,
            }, // 43
            Action::TableauToTableau {
                from_column: 1,
                to_column: 5,
                card_count: 1,
            }, // 44
            Action::TableauToFreecell {
                from_column: 1,
                to_cell: 3,
            }, // 45
            Action::TableauToFoundation {
                from_column: 1,
                to_pile: 3,
            }, // 46
            Action::FreecellToFoundation {
                from_cell: 0,
                to_pile: 3,
            }, // 47
            Action::FreecellToFoundation {
                from_cell: 2,
                to_pile: 3,
            }, // 48
            Action::TableauToFoundation {
                from_column: 6,
                to_pile: 3,
            }, // 49
            Action::TableauToFoundation {
                from_column: 6,
                to_pile: 0,
            }, // 50
            Action::TableauToFreecell {
                from_column: 3,
                to_cell: 0,
            }, // 51
            Action::TableauToFreecell {
                from_column: 3,
                to_cell: 2,
            }, // 51
            Action::TableauToTableau {
                from_column: 3,
                to_column: 5,
                card_count: 1,
            }, // 51
            Action::FreecellToTableau {
                from_cell: 2,
                to_column: 5,
            }, // 51
            Action::FreecellToTableau {
                from_cell: 0,
                to_column: 5,
            }, // 51
            Action::TableauToFreecell {
                from_column: 3,
                to_cell: 0,
            }, // 52
            Action::TableauToFoundation {
                from_column: 3,
                to_pile: 1,
            }, // 53
            Action::TableauToFoundation {
                from_column: 2,
                to_pile: 3,
            }, // 54
            Action::TableauToFoundation {
                from_column: 4,
                to_pile: 1,
            }, // 55
            Action::TableauToFoundation {
                from_column: 2,
                to_pile: 1,
            }, // 56
            Action::TableauToFoundation {
                from_column: 4,
                to_pile: 2,
            }, // 57
            Action::TableauToFoundation {
                from_column: 6,
                to_pile: 3,
            }, // 58
            Action::TableauToFoundation {
                from_column: 4,
                to_pile: 1,
            }, // 59
            Action::TableauToFreecell {
                from_column: 0,
                to_cell: 2,
            }, // 60
            Action::TableauToTableau {
                from_column: 0,
                to_column: 4,
                card_count: 1,
            }, // 60
            Action::FreecellToTableau {
                from_cell: 2,
                to_column: 4,
            }, // 60
            Action::TableauToFreecell {
                from_column: 6,
                to_cell: 2,
            }, // 61  // moved 8
            Action::FreecellToTableau {
                from_cell: 1,
                to_column: 0,
            }, // 61
            Action::TableauToTableau {
                from_column: 6,
                to_column: 0,
                card_count: 1,
            }, // 61  // moved 9
            Action::FreecellToTableau {
                from_cell: 3,
                to_column: 1,
            }, // 61
            Action::FreecellToTableau {
                from_cell: 0,
                to_column: 1,
            }, // 61
            Action::TableauToFreecell {
                from_column: 6,
                to_cell: 0,
            }, // 61  // moved 10
            Action::TableauToTableau {
                from_column: 6,
                to_column: 4,
                card_count: 1,
            }, // 61// moved jack
            Action::TableauToFreecell {
                from_column: 6,
                to_cell: 3,
            }, // 61 // moved queen
            Action::TableauToFreecell {
                from_column: 6,
                to_cell: 1,
            }, // 61 // moved king
            Action::TableauToFoundation {
                from_column: 6,
                to_pile: 0,
            }, // 62
            Action::FreecellToFoundation {
                from_cell: 2,
                to_pile: 0,
            }, // 63
            Action::TableauToFoundation {
                from_column: 7,
                to_pile: 3,
            }, // 64
            Action::TableauToFoundation {
                from_column: 2,
                to_pile: 2,
            }, // 65
            Action::TableauToFoundation {
                from_column: 7,
                to_pile: 0,
            }, // 66
            Action::TableauToFoundation {
                from_column: 2,
                to_pile: 1,
            }, // 67
            Action::TableauToFoundation {
                from_column: 0,
                to_pile: 3,
            }, // 68
            Action::TableauToFoundation {
                from_column: 2,
                to_pile: 2,
            }, // 69
            Action::FreecellToFoundation {
                from_cell: 0,
                to_pile: 1,
            }, // 70
            Action::TableauToFoundation {
                from_column: 5,
                to_pile: 3,
            }, // 71
            Action::TableauToFoundation {
                from_column: 0,
                to_pile: 0,
            }, // 72
            Action::TableauToFoundation {
                from_column: 7,
                to_pile: 2,
            }, // 73
            Action::TableauToFoundation {
                from_column: 5,
                to_pile: 1,
            }, // 74
            Action::TableauToFoundation {
                from_column: 4,
                to_pile: 2,
            }, // 75
            Action::TableauToFoundation {
                from_column: 3,
                to_pile: 0,
            }, // 76
            Action::TableauToFoundation {
                from_column: 0,
                to_pile: 3,
            }, // 77
            Action::FreecellToFoundation {
                from_cell: 3,
                to_pile: 0,
            }, // 78
            Action::TableauToFoundation {
                from_column: 5,
                to_pile: 2,
            }, // 79
            Action::TableauToFoundation {
                from_column: 4,
                to_pile: 1,
            }, // 80
            Action::TableauToFoundation {
                from_column: 1,
                to_pile: 3,
            }, // 81
            Action::FreecellToFoundation {
                from_cell: 1,
                to_pile: 2,
            }, // 82
            Action::TableauToFoundation {
                from_column: 5,
                to_pile: 1,
            }, // 83
            Action::TableauToFoundation {
                from_column: 4,
                to_pile: 3,
            }, // 84
            Action::TableauToFoundation {
                from_column: 1,
                to_pile: 0,
            }, // 85
        ],
        _ => panic!("No solution for this seed"),
    }
}
