# gameRules.md

**Purpose:**  
Defines the terminology, rules, and core components of FreeCell as implemented in this project. Serves as a reference for developers and contributors.

---

## FreeCell Game Components & Terminology

### Tableau (Main Playing Area)
- 8 columns of cards dealt face-up at the start of the game.
- Columns 1-4 start with 7 cards each; columns 5-8 start with 6 cards each.
- Only the bottom (exposed) card in each column can be moved.
- Cards can be built down in alternating colors (red on black, black on red).

### Free Cells (Temporary Storage)
- 4 empty spaces in the upper-left corner of the play area.
- Each free cell can hold exactly one card.
- Any exposed card can be moved to an empty free cell.
- Cards in free cells can be moved to tableau columns or foundations.

### Foundations (Goal Piles)
- 4 piles in the upper-right corner, one for each suit (♠, ♥, ♦, ♣).
- Foundations are built up from Ace to King in the same suit.
- Cards can only be moved to foundations, not from them.
- The game is won when all cards are in the foundations.

### Exposed Card
- The bottom card in a tableau column.
- Any card in a free cell.
- The top card of a foundation pile.

### Valid Moves
- **Tableau to Tableau:** Move an exposed card to another tableau column if it is one rank lower and of the opposite color.
- **Tableau to Free Cell:** Move any exposed card to an empty free cell.
- **Free Cell to Tableau:** Move a card from a free cell to a tableau column if it is one rank lower and of the opposite color.
- **To Foundation:** Move an Ace to an empty foundation, or the next rank in the same suit to its foundation pile.
- **To Empty Tableau:** Any exposed card can be moved to an empty tableau column.

### Sequences
- Multiple cards can be moved together as a sequence if they form a valid descending sequence in alternating colors.
- The maximum number of cards that can be moved as a sequence is limited by the number of empty free cells and empty tableau columns (see "power moves" in FreeCell strategy).

### Win Condition
- The game is won when all 52 cards are moved to the foundations, sorted by suit from Ace to King.

### Microsoft FreeCell Compatibility
- The initial deal for a given game number matches the original Microsoft FreeCell algorithm, ensuring identical shuffles and gameplay for the same game number.

---

**See also:**  
- [projectbrief.md](./projectbrief.md) for project goals  
- [systemPatterns.md](./systemPatterns.md) for architecture  
- [activeContext.md](./activeContext.md) for current work focus
