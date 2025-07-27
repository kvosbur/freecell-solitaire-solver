import React, { useState, useEffect, useCallback } from 'react';
import Card from './Card';
import PlaybackControls from './PlaybackControls';
import SolutionSelector from './SolutionSelector';
import { PlaybackController } from './playbackController';
import {
  shuffleDeck,
  dealCards,
  canMoveToFoundation,
  canMoveToTableau,
  canMoveToFreecell,
  isGameWon,
  getMovableSequence,
  getMaxMovableSequenceLength,
  getSolvedSeeds,
  addSolvedSeed,
  getGameStats,
  updateGameStats,
  SUITS
} from './gameUtils';
import './index.css';

const App = () => {
  const [gameState, setGameState] = useState(null);
  const [selectedCard, setSelectedCard] = useState(null);
  const [selectedLocation, setSelectedLocation] = useState(null);
  const [gameHistory, setGameHistory] = useState([]);
  const [currentSeed, setCurrentSeed] = useState(null);
  const [seedInput, setSeedInput] = useState('');
  const [moveCount, setMoveCount] = useState(0);
  const [gameWon, setGameWon] = useState(false);
  const [solvedSeeds, setSolvedSeeds] = useState([]);
  const [stats, setStats] = useState(null);
  const [draggedCard, setDraggedCard] = useState(null);
  
  // Playback state
  const [playbackController] = useState(() => new PlaybackController());
  const [isPlaybackMode, setIsPlaybackMode] = useState(false);
  const [isPlaying, setIsPlaying] = useState(false);
  const [currentPlaybackMove, setCurrentPlaybackMove] = useState(0);
  const [playbackSpeed, setPlaybackSpeed] = useState(500);
  const [totalMoves, setTotalMoves] = useState(0);

  useEffect(() => {
    setSolvedSeeds(getSolvedSeeds());
    setStats(getGameStats());
    startNewGame();
  }, []);

  const startNewGame = useCallback((seed = null) => {
    const gameSeed = seed || Math.floor(Math.random() * 1000000);
    const deck = shuffleDeck(gameSeed);
    const newGameState = dealCards(deck);
    
    setGameState(newGameState);
    setCurrentSeed(gameSeed);
    setSelectedCard(null);
    setSelectedLocation(null);
    setGameHistory([]);
    setMoveCount(0);
    setGameWon(false);
    setSeedInput(gameSeed.toString());
  }, []);

  const saveGameState = useCallback(() => {
    if (gameState) {
      setGameHistory(prev => [...prev, {
        gameState: JSON.parse(JSON.stringify(gameState)),
        moveCount
      }]);
    }
  }, [gameState, moveCount]);

  const undoMove = useCallback(() => {
    if (gameHistory.length > 0) {
      const previousState = gameHistory[gameHistory.length - 1];
      setGameState(previousState.gameState);
      setMoveCount(previousState.moveCount);
      setGameHistory(prev => prev.slice(0, -1));
      setSelectedCard(null);
      setSelectedLocation(null);
    }
  }, [gameHistory]);

  const makeMove = useCallback((fromLocation, toLocation, cards) => {
    if (!gameState) return false;

      saveGameState();
    
    const newGameState = JSON.parse(JSON.stringify(gameState));
    
    // Remove cards from source
    if (fromLocation.type === 'tableau') {
      newGameState.tableau[fromLocation.index] = 
        newGameState.tableau[fromLocation.index].slice(0, -cards.length);
    } else if (fromLocation.type === 'freecell') {
      newGameState.freecells[fromLocation.index] = null;
    }
    
    // Add cards to destination
    if (toLocation.type === 'tableau') {
      newGameState.tableau[toLocation.index].push(...cards);
    } else if (toLocation.type === 'foundation') {
      newGameState.foundations[toLocation.index].push(...cards);
    } else if (toLocation.type === 'freecell') {
      newGameState.freecells[toLocation.index] = cards[0];
    }
    
    setGameState(newGameState);
    setMoveCount(prev => prev + 1);
    
    // Check for win condition
    if (isGameWon(newGameState.foundations)) {
      setGameWon(true);
      const newSolvedSeeds = addSolvedSeed(currentSeed);
      setSolvedSeeds(newSolvedSeeds);
      const newStats = updateGameStats(true, moveCount + 1);
      setStats(newStats);
    }
    
    return true;
  }, [gameState, saveGameState, moveCount, currentSeed]);

  const handleCardClick = useCallback((card, location) => {
    if (!gameState || gameWon) return;

    // If clicking the same card, deselect
    if (selectedCard && selectedCard.id === card.id) {
      setSelectedCard(null);
      setSelectedLocation(null);
      return;
    }

    // If no card selected, select this card
    if (!selectedCard) {
      // Only allow selecting from tableau top cards or freecells
      if (location.type === 'tableau') {
        const tableau = gameState.tableau[location.index];
        if (tableau.length > 0 && tableau[tableau.length - 1].id === card.id) {
          setSelectedCard(card);
          setSelectedLocation(location);
        }
      } else if (location.type === 'freecell') {
        setSelectedCard(card);
        setSelectedLocation(location);
      }
      return;
    }

    // Try to move selected card(s) to this location
    if (selectedLocation.type === 'tableau') {
      const sourceTableau = gameState.tableau[selectedLocation.index];
      const cardIndex = sourceTableau.findIndex(c => c.id === selectedCard.id);
      const sequence = getMovableSequence(sourceTableau, cardIndex);
      
      // Check if we can move this sequence
      const emptyFreecells = gameState.freecells.filter(cell => cell === null).length;
      const emptyTableaus = gameState.tableau.filter(col => col.length === 0).length;
      const maxMovable = getMaxMovableSequenceLength(gameState.freecells, emptyTableaus);
      
      if (sequence.length > maxMovable) {
        setSelectedCard(null);
        setSelectedLocation(null);
        return;
      }
      
      // Check if move is valid
      let canMove = false;
      if (location.type === 'tableau') {
        canMove = canMoveToTableau(sequence[0], gameState.tableau[location.index]);
      } else if (location.type === 'foundation' && sequence.length === 1) {
        canMove = canMoveToFoundation(sequence[0], gameState.foundations[location.index]);
      }
      
      if (canMove) {
        makeMove(selectedLocation, location, sequence);
      }
    } else if (selectedLocation.type === 'freecell') {
      let canMove = false;
      if (location.type === 'tableau') {
        canMove = canMoveToTableau(selectedCard, gameState.tableau[location.index]);
      } else if (location.type === 'foundation') {
        canMove = canMoveToFoundation(selectedCard, gameState.foundations[location.index]);
      }
      
      if (canMove) {
        makeMove(selectedLocation, location, [selectedCard]);
      }
    }
    
    setSelectedCard(null);
    setSelectedLocation(null);
  }, [gameState, selectedCard, selectedLocation, makeMove, gameWon]);

  const handleSlotClick = useCallback((location) => {
    if (!selectedCard || !gameState || gameWon) return;

    let canMove = false;
    let cardsToMove = [];

    if (selectedLocation.type === 'tableau') {
      const sourceTableau = gameState.tableau[selectedLocation.index];
      const cardIndex = sourceTableau.findIndex(c => c.id === selectedCard.id);
      cardsToMove = getMovableSequence(sourceTableau, cardIndex);
    } else {
      cardsToMove = [selectedCard];
    }

    if (location.type === 'freecell' && cardsToMove.length === 1) {
      canMove = canMoveToFreecell(gameState.freecells);
    } else if (location.type === 'tableau') {
      canMove = canMoveToTableau(cardsToMove[0], gameState.tableau[location.index]);
    } else if (location.type === 'foundation' && cardsToMove.length === 1) {
      canMove = canMoveToFoundation(cardsToMove[0], gameState.foundations[location.index]);
    }

    if (canMove) {
      makeMove(selectedLocation, location, cardsToMove);
    }
    
    setSelectedCard(null);
    setSelectedLocation(null);
  }, [selectedCard, selectedLocation, gameState, makeMove, gameWon]);

  const handleNewGameClick = () => {
    const seed = seedInput.trim();
    if (seed && !isNaN(seed)) {
      startNewGame(parseInt(seed));
    } else {
      startNewGame();
    }
  };

  // Playback methods
  const loadSolution = useCallback(async (solutionFile) => {
    try {
      console.log('Loading solution:', solutionFile);
      const response = await fetch(`/results/${solutionFile}`);
      
      if (!response.ok) {
        throw new Error(`HTTP error! status: ${response.status}`);
      }
      
      const solutionData = await response.json();
      console.log('Solution data loaded:', solutionData);
      
      // Reset playback state
      setIsPlaybackMode(true);
      setIsPlaying(false);
      setCurrentPlaybackMove(0);
      
      // Start new game with the solution's seed
      startNewGame(solutionData.seed);
      
      // Wait a bit for the game state to update
      setTimeout(async () => {
        // Load solution into controller
        await playbackController.loadSolution(solutionData);
        setTotalMoves(solutionData.solution_moves.length);
        console.log('Playback controller loaded with', solutionData.solution_moves.length, 'moves');
      }, 100);
      
    } catch (error) {
      console.error('Failed to load solution:', error);
      console.error('Error details:', error.message);
      alert(`Failed to load solution: ${error.message}`);
    }
  }, [playbackController, startNewGame]);

  const handlePlay = useCallback(async () => {
    setIsPlaying(true);
    
    // Monitor progress
    const progressInterval = setInterval(() => {
      setCurrentPlaybackMove(playbackController.currentMoveIndex);
    }, 100);
    
    await playbackController.play();
    
    clearInterval(progressInterval);
    setIsPlaying(false);
    setCurrentPlaybackMove(playbackController.currentMoveIndex);
  }, [playbackController]);

  const handlePause = useCallback(() => {
    playbackController.pause();
    setIsPlaying(false);
  }, [playbackController]);

  const handleStepForward = useCallback(async () => {
    await playbackController.stepForward();
    setCurrentPlaybackMove(playbackController.currentMoveIndex);
  }, [playbackController]);

  const handleStepBackward = useCallback(() => {
    if (currentPlaybackMove > 0 && gameHistory.length > 0) {
      // Use the existing undo functionality
      undoMove();
      
      // Update playback controller position
      playbackController.currentMoveIndex = currentPlaybackMove - 1;
      setCurrentPlaybackMove(currentPlaybackMove - 1);
      
      console.log(`Stepped backward to move ${currentPlaybackMove - 1}`);
    } else {
      console.log('Cannot step backward - at beginning of solution');
    }
  }, [currentPlaybackMove, gameHistory.length, undoMove, playbackController]);

  const handleSpeedChange = useCallback((speed) => {
    setPlaybackSpeed(speed);
    playbackController.setSpeed(speed);
  }, [playbackController]);

  const exitPlaybackMode = useCallback(() => {
    setIsPlaybackMode(false);
    setIsPlaying(false);
    playbackController.reset();
  }, [playbackController]);

  const handleKeyDown = useCallback((e) => {
    if (e.key === 'Escape') {
      setSelectedCard(null);
      setSelectedLocation(null);
    } else if (e.key === 'u' && e.ctrlKey) {
      e.preventDefault();
      undoMove();
    }
  }, [undoMove]);

  useEffect(() => {
    document.addEventListener('keydown', handleKeyDown);
    return () => document.removeEventListener('keydown', handleKeyDown);
  }, [handleKeyDown]);

  if (!gameState) {
    return <div className="app">Loading...</div>;
  }

  const winRate = stats ? (stats.gamesPlayed > 0 ? Math.round((stats.gamesWon / stats.gamesPlayed) * 100) : 0) : 0;
  const avgMoves = stats ? (stats.gamesPlayed > 0 ? Math.round(stats.totalMoves / stats.gamesPlayed) : 0) : 0;

  return (
    <div className="app">
      <header className="header">
        <h1>🦕 Freecell Solitaire</h1>
        <p className="subtitle">A prehistoric card game adventure!</p>
      </header>

      <div className="controls">
        <input
          type="number"
          value={seedInput}
          onChange={(e) => setSeedInput(e.target.value)}
          placeholder="Game seed"
          className="seed-input"
          aria-label="Game seed number"
        />
        <button onClick={handleNewGameClick} className="btn btn-primary">
          New Game
        </button>
        <button 
          onClick={undoMove} 
          className="btn btn-secondary"
          disabled={gameHistory.length === 0}
          aria-label={`Undo last move. ${gameHistory.length} moves available to undo`}
        >
          Undo ({gameHistory.length})
        </button>
        <SolutionSelector 
          onSelectSolution={loadSolution}
          currentSeed={currentSeed}
        />
        {isPlaybackMode && (
          <button 
            onClick={exitPlaybackMode} 
            className="btn btn-secondary"
            title="Exit playback mode"
          >
            Exit Playback
          </button>
        )}
      </div>

      {isPlaybackMode && (
        <PlaybackControls
          isPlaying={isPlaying}
          onPlay={handlePlay}
          onPause={handlePause}
          onStepForward={handleStepForward}
          onStepBackward={handleStepBackward}
          onSpeedChange={handleSpeedChange}
          currentMove={currentPlaybackMove}
          totalMoves={totalMoves}
          speed={playbackSpeed}
        />
      )}

      <div className="main-content">
        <div className="stats-sidebar">
          <h3>🦴 Fossil Stats</h3>
          <div className="stats-grid">
            <div className="stat-item">
              <strong>Current Seed:</strong> {currentSeed}
            </div>
            <div className="stat-item">
              <strong>Moves:</strong> {moveCount}
            </div>
            <div className="stat-item">
              <strong>Games Played:</strong> {stats?.gamesPlayed || 0}
            </div>
            <div className="stat-item">
              <strong>Win Rate:</strong> {winRate}%
            </div>
            <div className="stat-item">
              <strong>Avg Moves:</strong> {avgMoves}
            </div>
            <div className="stat-item">
              <strong>Best Game:</strong> {stats?.bestMoveCount || 'N/A'} moves
            </div>
          </div>
          
          {solvedSeeds.length > 0 && (
            <div className="solved-seeds">
              <h4>🏆 Conquered Seeds ({solvedSeeds.length})</h4>
              <div className="seed-list">
                {solvedSeeds.slice(-20).map(seed => (
                  <span key={seed} className="seed-tag">{seed}</span>
                ))}
                {solvedSeeds.length > 20 && <span className="seed-tag">...</span>}
              </div>
            </div>
          )}
        </div>

        <div className={`game-board ${isPlaybackMode && isPlaying ? 'playback-active' : ''}`} role="main" aria-label="Freecell game board">
        <div className="top-row">
          <div className="foundations" role="region" aria-label="Foundation piles">
            <span className="sr-only">Foundation piles where cards are built up by suit from Ace to King</span>
            {gameState.foundations.map((foundation, index) => {
              const suitNames = ['Hearts', 'Diamonds', 'Clubs', 'Spades'];
              const topCard = foundation.length > 0 ? foundation[foundation.length - 1] : null;
              return (
                <div
                  key={`foundation-${index}`}
                  className="card-slot foundation-slot"
                  onClick={() => handleSlotClick({ type: 'foundation', index })}
                  tabIndex={-1}
                  onKeyDown={(e) => {
                    if (e.key === 'Enter' || e.key === ' ') {
                      e.preventDefault();
                      handleSlotClick({ type: 'foundation', index });
                    }
                  }}
                  aria-label={topCard ? 
                    `${suitNames[index]} foundation: ${topCard.rank} of ${suitNames[index]}` : 
                    `Empty ${suitNames[index]} foundation`}
                >
                  {topCard && (
                    <Card
                      card={topCard}
                      onClick={() => handleCardClick(topCard, { type: 'foundation', index })}
                      selected={selectedCard && selectedCard.id === topCard.id}
                      tabIndex={-1}
                    />
                  )}
                </div>
              );
            })}
          </div>

          <div className="freecells" role="region" aria-label="Free cells">
            <span className="sr-only">Free cells for temporary card storage</span>
            {gameState.freecells.map((card, index) => (
              <div
                key={`freecell-${index}`}
                className="card-slot"
                onClick={() => handleSlotClick({ type: 'freecell', index })}
                tabIndex={0}
                onKeyDown={(e) => {
                  if (e.key === 'Enter' || e.key === ' ') {
                    e.preventDefault();
                    handleSlotClick({ type: 'freecell', index });
                  }
                }}
                aria-label={card ? `Free cell ${index + 1}: ${card.rank} of ${card.suit}` : `Empty free cell ${index + 1}`}
              >
                {card && (
                  <Card
                    card={card}
                    onClick={() => handleCardClick(card, { type: 'freecell', index })}
                    selected={selectedCard && selectedCard.id === card.id}
                    tabIndex={0}
                  />
                )}
              </div>
            ))}
          </div>
        </div>

        <div className="tableau" role="region" aria-label="Tableau columns">
          <span className="sr-only">Eight tableau columns where cards are built down in alternating colors</span>
          {gameState.tableau.map((column, colIndex) => (
            <div key={`tableau-${colIndex}`} className="tableau-column">
              {column.length === 0 ? (
                <div
                  className="card-slot"
                  onClick={() => handleSlotClick({ type: 'tableau', index: colIndex })}
                  tabIndex={0}
                  onKeyDown={(e) => {
                    if (e.key === 'Enter' || e.key === ' ') {
                      e.preventDefault();
                      handleSlotClick({ type: 'tableau', index: colIndex });
                    }
                  }}
                  aria-label={`Empty tableau column ${colIndex + 1}`}
                />
              ) : (
                column.map((card, cardIndex) => (
                  <Card
                    key={card.id}
                    card={card}
                    onClick={() => handleCardClick(card, { type: 'tableau', index: colIndex })}
                    selected={selectedCard && selectedCard.id === card.id}
                    style={{ top: `${cardIndex * -80}px`, zIndex: column.length + cardIndex }}
                    tabIndex={cardIndex === column.length - 1 ? 0 : -1}
                    aria-label={`${card.rank} of ${card.suit} in tableau column ${colIndex + 1}, position ${cardIndex + 1}`}
                  />
                ))
              )}
              {column.length > 0 && (
                <div
                  className="card-slot"
                  onClick={() => handleSlotClick({ type: 'tableau', index: colIndex })}
                  style={{ top: `${column.length * -80}px`, opacity: 0 }}
                  tabIndex={-1}
                  aria-hidden="true"
                />
              )}
            </div>
          ))}
        </div>
        </div>
      </div>

      {gameWon && (
        <div className="victory-message" role="dialog" aria-live="assertive">
          <h2>🎉 Victory! 🦕</h2>
          <p>You've excavated all the cards!</p>
          <p>Game #{currentSeed} completed in {moveCount} moves</p>
          <button 
            onClick={() => setGameWon(false)} 
            className="btn btn-primary"
            style={{ marginTop: '15px' }}
          >
            Continue
          </button>
        </div>
      )}

      <div className="sr-only" aria-live="polite">
        {selectedCard && `Selected: ${selectedCard.rank} of ${selectedCard.suit}`}
      </div>
    </div>
  );
};

export default App;
