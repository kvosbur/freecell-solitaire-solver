// Card utilities
export const SUITS = {
  CLUBS: '♣',
  DIAMONDS: '♦',
  HEARTS: '♥',
  SPADES: '♠'
};

export const RANKS = ['A', '2', '3', '4', '5', '6', '7', '8', '9', '10', 'J', 'Q', 'K'];

export const createDeck = () => {
  const deck = [];
  const suits = Object.values(SUITS);
  
  for (let i = 0; i < RANKS.length; i++) {
    for (let suit of suits) {
      deck.push({
        suit,
        rank: RANKS[i],
        value: i + 1,
        color: (suit === SUITS.HEARTS || suit === SUITS.DIAMONDS) ? 'red' : 'black',
        id: `${RANKS[i]}_${suit}`
      });
    }
  }
  
  return deck;
};

// Seeded random number generator (Microsoft's FreeCell algorithm)
export const createSeededRandom = (seed) => {
  let state = seed;
  
  return () => {
    state = (state * 214013 + 2531011) % 0x80000000;
    return Math.trunc(state / 0x10000);
  };
};

export const shuffleDeck = (seed) => {
  const deck = createDeck();
  const random = createSeededRandom(seed);
  
  // Microsoft FreeCell shuffle algorithm
  for (let i = deck.length - 1; i > 0; i--) {
    const j = random() % (i + 1);
    [deck[i], deck[j]] = [deck[j], deck[i]];
  }
  
  return deck;
};

export const dealCards = (deck) => {
  const tableau = [[], [], [], [], [], [], [], []];
  deck = deck.reverse(); // Reverse deck to deal from the end
  for (let card = 0; card < deck.length; card++) {
    tableau[card % 8].push(deck[card]);
  }
  
  return {
    tableau,
    freecells: [null, null, null, null],
    foundations: [[], [], [], []] // Hearts, Diamonds, Clubs, Spades
  };
};

// Game logic utilities
export const canMoveToFoundation = (card, foundation) => {
  if (!card) return false;
  
  if (foundation.length === 0) {
    return card.value === 1; // Only Ace can start a foundation
  }
  
  const topCard = foundation[foundation.length - 1];
  return card.suit === topCard.suit && card.value === topCard.value + 1;
};

export const canMoveToTableau = (card, tableau) => {
  if (!card) return false;
  
  if (tableau.length === 0) {
    return true; // Any card can go on empty tableau
  }
  
  const topCard = tableau[tableau.length - 1];
  return card.color !== topCard.color && card.value === topCard.value - 1;
};

export const canMoveToFreecell = (freecells) => {
  return freecells.some(cell => cell === null);
};

export const isGameWon = (foundations) => {
  return foundations.every(foundation => foundation.length === 13);
};

export const getMovableSequence = (tableau, startIndex) => {
  if (tableau.length === 0 || startIndex >= tableau.length) {
    return [];
  }
  
  const sequence = [tableau[startIndex]];
  
  for (let i = startIndex + 1; i < tableau.length; i++) {
    const currentCard = tableau[i];
    const previousCard = tableau[i - 1];
    
    if (currentCard.color !== previousCard.color && 
        currentCard.value === previousCard.value - 1) {
      sequence.push(currentCard);
    } else {
      break;
    }
  }
  
  return sequence;
};

export const getMaxMovableSequenceLength = (freecells, emptyTableaus) => {
  const emptyFreecells = freecells.filter(cell => cell === null).length;
  return (emptyFreecells + 1) * Math.pow(2, emptyTableaus);
};

// Local storage utilities
export const getSolvedSeeds = () => {
  const stored = localStorage.getItem('freecell-solved-seeds');
  return stored ? JSON.parse(stored) : [];
};

export const addSolvedSeed = (seed) => {
  const solvedSeeds = getSolvedSeeds();
  if (!solvedSeeds.includes(seed)) {
    solvedSeeds.push(seed);
    localStorage.setItem('freecell-solved-seeds', JSON.stringify(solvedSeeds));
  }
  return solvedSeeds;
};

export const getGameStats = () => {
  const stored = localStorage.getItem('freecell-stats');
  return stored ? JSON.parse(stored) : {
    gamesPlayed: 0,
    gamesWon: 0,
    totalMoves: 0,
    bestMoveCount: null
  };
};

export const updateGameStats = (won, moveCount) => {
  const stats = getGameStats();
  stats.gamesPlayed++;
  stats.totalMoves += moveCount;
  
  if (won) {
    stats.gamesWon++;
    if (stats.bestMoveCount === null || moveCount < stats.bestMoveCount) {
      stats.bestMoveCount = moveCount;
    }
  }
  
  localStorage.setItem('freecell-stats', JSON.stringify(stats));
  return stats;
};
