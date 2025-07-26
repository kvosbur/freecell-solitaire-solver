# 🦕 Freecell Solitaire Webapp

A dinosaur-themed Freecell solitaire game built with React!

## Features

- 🦴 Classic Freecell gameplay with dinosaur branding
- 🎯 Game seed input for playing specific games
- ↩️ Undo functionality
- 📊 Statistics tracking (games played, win rate, best moves)
- 🏆 Solved seeds tracking
- ♿ Accessibility features (screen reader support, keyboard navigation)
- 🎨 Simple, clean design

## How to Play

1. **Objective**: Move all cards to the foundation piles (top right), building up from Ace to King by suit.

2. **Game Areas**:
   - **Free Cells** (top left): Temporary storage for individual cards
   - **Foundations** (top right): Build up suits from A to K
   - **Tableau** (bottom): Eight columns where you build down in alternating colors

3. **Rules**:
   - Move cards between tableau columns in descending order with alternating colors
   - Only Kings can be placed on empty tableau columns
   - Free cells can hold one card each
   - Foundation piles must be built up by suit starting with Ace

4. **Controls**:
   - Click cards to select/move them
   - Use keyboard navigation (Tab to move focus, Enter/Space to select)
   - Press Ctrl+U to undo moves
   - Press Escape to deselect cards

## Running the App

1. Install dependencies:
```bash
npm install
```

2. Start the development server:
```bash
npm start
```

3. Open http://localhost:3000 in your browser

## Game Controls

- **New Game**: Start a new random game or enter a specific seed
- **Undo**: Revert the last move (shows number of available undos)
- **Seed Input**: Enter a specific game number to play that exact layout

## Accessibility

The game includes:
- Full keyboard navigation support
- Screen reader announcements for game state
- High contrast focus indicators
- Descriptive ARIA labels for all game elements

## Technical Details

- Built with React 18
- Uses Microsoft's FreeCell shuffle algorithm for authentic game generation
- Stores game statistics and solved seeds in localStorage
- Responsive design (optimized for desktop)

## Dinosaur Theme

The app features a prehistoric theme with:
- 🦕 Dinosaur emojis throughout the interface
- Earthy color scheme reminiscent of fossil hunting
- "Fossil Stats" and "Conquered Seeds" terminology
- Victory messages with excavation themes

Enjoy your prehistoric card game adventure!
