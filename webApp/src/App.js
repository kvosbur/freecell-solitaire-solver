import React, { useState, useEffect, useCallback } from 'react';
import Card from './Card';
import PlaybackControls from './PlaybackControls';
import Toast from './Toast';
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
  getLastCompletedSeed,
  setLastCompletedSeed as saveLastCompletedSeed
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
  
  // Playback state
  const [playbackController] = useState(() => new PlaybackController());
  const [isPlaybackMode, setIsPlaybackMode] = useState(false);
  const [isPlaying, setIsPlaying] = useState(false);
  const [currentPlaybackMove, setCurrentPlaybackMove] = useState(0);
  const [playbackSpeed, setPlaybackSpeed] = useState(500);
  const [totalMoves, setTotalMoves] = useState(0);

  // Toast notification state
  const [toasts, setToasts] = useState([]);

  // Auto-play all games state
  const [isAutoPlayingAll, setIsAutoPlayingAll] = useState(false);
  const [autoPlayProgress, setAutoPlayProgress] = useState({ current: 0, total: 0 });
  const [autoPlayController, setAutoPlayController] = useState(null);
  const [lastCompletedSeed, setLastCompletedSeed] = useState(0);
  const [currentAutoPlaySeed, setCurrentAutoPlaySeed] = useState(null);

  // Toast helper functions
  const showToast = useCallback((message, type = 'info') => {
    const id = Date.now() + Math.random();
    const newToast = { id, message, type };
    setToasts(prev => [...prev, newToast]);
  }, []);

  const removeToast = useCallback((id) => {
    setToasts(prev => prev.filter(toast => toast.id !== id));
  }, []);

  // Security: Validate solution file names to prevent path traversal
  const validateSolutionFile = useCallback((filename) => {
    // Only allow files matching pattern: number.json
    const sanitized = filename.replace(/[^0-9.json]/g, '');
    
    if (!/^\d+\.json$/.test(sanitized)) {
      throw new Error('Invalid solution file format');
    }
    
    return sanitized;
  }, []);

  const startNewGame = useCallback((seed = null, resetPlayback = true) => {
    // Limit seeds to 1-32000 range
    const gameSeed = seed || Math.floor(Math.random() * 32000) + 1;
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
    
    // Only reset playback state when explicitly requested (manual new game)
    if (resetPlayback) {
      setIsPlaybackMode(false);
      setIsPlaying(false);
      setCurrentPlaybackMove(0);
      setTotalMoves(0);
      playbackController.reset();
    }
  }, [playbackController]);

  useEffect(() => {
    setSolvedSeeds(getSolvedSeeds());
    setStats(getGameStats());
    setLastCompletedSeed(getLastCompletedSeed());
    startNewGame();
  }, [startNewGame]);

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
      const parsedSeed = parseInt(seed);
      if (parsedSeed >= 1) {
        startNewGame(parsedSeed);
      } else {
        showToast('Please enter a seed greater than 1', 'warning');
        return;
      }
    } else {
      startNewGame();
    }
  };

  // Playback methods
  const loadSolution = useCallback(async (solutionFile) => {
    try {
      // Validate and sanitize solution file name for security
      const validatedFile = validateSolutionFile(solutionFile);
      console.log('Loading solution:', validatedFile);
      const response = await fetch(`/results/${validatedFile}`);
      
      if (!response.ok) {
        if (response.status === 404) {
          throw new Error(`Solution file not found. This game may be unsolvable or the solution hasn't been computed yet.`);
        } else {
          throw new Error(`HTTP error! status: ${response.status}`);
        }
      }
      
      const contentType = response.headers.get('content-type');
      if (!contentType || !contentType.includes('application/json')) {
        throw new Error(`Solution file not found. This game may be unsolvable or the solution hasn't been computed yet.`);
      }
      
      const solutionData = await response.json();
      console.log('Solution data loaded:', solutionData);
      
      // Validate solution data has required fields
      if (!solutionData.seed || !solutionData.solution_moves || !Array.isArray(solutionData.solution_moves)) {
        throw new Error('Invalid solution file format - missing required fields');
      }
      
      // Reset playback state
      setIsPlaybackMode(true);
      setIsPlaying(false);
      setCurrentPlaybackMove(0);
      
      // Start new game with the solution's seed (don't reset playback state)
      startNewGame(solutionData.seed, false);
      
      // Wait a bit for the game state to update
      setTimeout(async () => {
        try {
          // Load solution into controller
          await playbackController.loadSolution(solutionData);
          setTotalMoves(solutionData.solution_moves.length);
          console.log('Playback controller loaded with', solutionData.solution_moves.length, 'moves');
        } catch (controllerError) {
          console.error('Playback controller error:', controllerError);
          // Clean up on controller error
          setIsPlaybackMode(false);
          setIsPlaying(false);
          setCurrentPlaybackMove(0);
          setTotalMoves(0);
          playbackController.reset();
          showToast('Failed to load solution into playback controller', 'error');
        }
      }, 100);
      
    } catch (error) {
      console.error('Failed to load solution:', error);
      console.error('Error details:', error.message);
      
      // Clean up any partial playback state on error
      setIsPlaybackMode(false);
      setIsPlaying(false);
      setCurrentPlaybackMove(0);
      setTotalMoves(0);
      playbackController.reset();
      
      showToast(`Failed to load solution: ${error.message}`, 'error');
      
      // Re-throw the error so autoSolve can handle it
      throw error;
    }
  }, [playbackController, startNewGame, showToast, validateSolutionFile]);

  // Auto-solve function that loads solution for current seed
  const autoSolve = useCallback(async () => {
    if (!currentSeed || currentSeed < 1 || currentSeed > 32000) {
      showToast('Auto-solve is only available for seeds 1-32000', 'warning');
      return;
    }
    
    try {
      const solutionFile = `${currentSeed}.json`;
      await loadSolution(solutionFile);
    } catch (error) {
      console.error('Auto-solve failed:', error);
      const errorMessage = error.message.includes('Solution file not found') 
        ? `No solution available for seed ${currentSeed}. This game may be unsolvable or the solution hasn't been computed yet.`
        : `Failed to auto-solve seed ${currentSeed}: ${error.message}`;
      showToast(errorMessage, 'error');
    }
  }, [currentSeed, loadSolution, showToast]);

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

  // Auto-play all games functionality
  const autoPlayAll = useCallback(async () => {
    if (isAutoPlayingAll) {
      // Stop auto-play
      if (autoPlayController) {
        autoPlayController.abort();
        setAutoPlayController(null);
      }
      setIsAutoPlayingAll(false);
      showToast(`Auto-play paused at seed ${lastCompletedSeed}. Click again to resume.`, 'info');
      return;
    }

    // Start or resume auto-play
    setIsAutoPlayingAll(true);
    const controller = new AbortController();
    setAutoPlayController(controller);

    // Generate list of seeds (1-32000, skip 11982)
    const allSeeds = [];
    const startSeed = lastCompletedSeed > 0 ? lastCompletedSeed + 1 : 1;
    
    for (let seed = startSeed; seed <= 32000; seed++) {
      if (seed !== 11982) {
        allSeeds.push(seed);
      }
    }

    // Calculate progress based on where we're resuming from
    const totalGames = 31999; // Total games (32000 - 1 for skipping 11982)
    const gamesCompleted = lastCompletedSeed > 0 ? 
      (lastCompletedSeed >= 11982 ? lastCompletedSeed - 1 : lastCompletedSeed) : 0;
    
    setAutoPlayProgress({ current: gamesCompleted, total: totalGames });
    
    if (lastCompletedSeed > 0) {
      showToast(`Resuming auto-play from seed ${startSeed} (${allSeeds.length} games remaining)`, 'info');
    } else {
      showToast(`Starting auto-play of ${allSeeds.length} games`, 'info');
    }

    let successCount = 0;
    let errorCount = 0;

    for (let i = 0; i < allSeeds.length; i++) {
      if (controller.signal.aborted) {
        break;
      }

      const seed = allSeeds[i];
      setCurrentAutoPlaySeed(seed);
      
      // Update progress based on total games completed out of 31999
      const totalCompleted = gamesCompleted + i + 1;
      setAutoPlayProgress({ current: totalCompleted, total: totalGames });
      console.log(`Progress: ${totalCompleted}/${totalGames} (${((totalCompleted / totalGames) * 100).toFixed(2)}%) - Playing seed ${seed}`);

      try {
        // Check if solution file exists before trying to load
        const solutionFile = `${seed}.json`;
        const validatedFile = validateSolutionFile(solutionFile);
        const response = await fetch(`/results/${validatedFile}`);
        
        if (!response.ok) {
          console.log(`Skipping seed ${seed} - no solution file`);
          continue;
        }

        // Load the solution (this will put us in playback mode)
        await loadSolution(solutionFile);
        
        // Wait a bit for the solution to load
        await new Promise(resolve => setTimeout(resolve, 200));
        
        // Set T-Rex speed for fast playback
        handleSpeedChange(0);
        
        // Start playback and wait for completion
        await handlePlay();
        
        // Wait for playback to complete
        await new Promise((resolve) => {
          const checkCompletion = () => {
            if (controller.signal.aborted || currentPlaybackMove >= totalMoves) {
              resolve();
            } else {
              setTimeout(checkCompletion, 100);
            }
          };
          checkCompletion();
        });
        
        successCount++;
        console.log(`Completed seed ${seed} (${i + 1}/${allSeeds.length})`);
        
        // Update last completed seed for resume capability
        setLastCompletedSeed(seed);
        saveLastCompletedSeed(seed);
        
        // Brief pause between games
        await new Promise(resolve => setTimeout(resolve, 300));
        
      } catch (error) {
        if (controller.signal.aborted) {
          break;
        }
        errorCount++;
        console.error(`Failed to auto-play seed ${seed}:`, error);
      }
    }

    // Cleanup
    setIsAutoPlayingAll(false);
    setAutoPlayController(null);
    setCurrentAutoPlaySeed(null);
    setAutoPlayProgress({ current: 0, total: 0 });
    
    if (!controller.signal.aborted) {
      if (successCount + errorCount >= allSeeds.length) {
        // All games completed - reset progress
        setLastCompletedSeed(0);
        saveLastCompletedSeed(0);
        showToast(`Auto-play completed! All ${successCount} games played, ${errorCount} errors`, 'success');
      } else {
        showToast(`Auto-play completed! ${successCount} games played, ${errorCount} errors`, 'success');
      }
    }
  }, [isAutoPlayingAll, autoPlayController, lastCompletedSeed, validateSolutionFile, loadSolution, handleSpeedChange, handlePlay, currentPlaybackMove, totalMoves, showToast]);

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
          min="1"
        />
        <button onClick={handleNewGameClick} className="btn btn-primary">
          Deal Game
        </button>
        <button 
          onClick={undoMove} 
          className="btn btn-secondary"
          disabled={gameHistory.length === 0}
          aria-label={`Undo last move. ${gameHistory.length} moves available to undo`}
        >
          Undo ({gameHistory.length})
        </button>
        <button 
          onClick={autoSolve} 
          className="btn btn-secondary"
          disabled={!currentSeed || currentSeed < 1 || currentSeed > 32000 || isPlaybackMode || isAutoPlayingAll}
          title={currentSeed >= 1 && currentSeed <= 32000 ? 
            `Auto-solve game #${currentSeed}` : 
            'Auto-solve is only available for seeds 1-32000'}
        >
          Auto Solve
        </button>
        <button 
          onClick={autoPlayAll} 
          className={`btn ${isAutoPlayingAll ? 'btn-primary' : 'btn-secondary'}`}
          disabled={isPlaybackMode && !isAutoPlayingAll}
          title={isAutoPlayingAll ? 
            'Stop auto-playing all games' : 
            'Auto-play all games with solutions (1-32000, skip 11982)'}
        >
          {isAutoPlayingAll ? 'Stop Auto-Play All' : 'Auto-Play All'}
        </button>
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
              <strong>Avg Moves:</strong> {avgMoves}
            </div>
            <div className="stat-item">
              <strong>Best Game:</strong> {stats?.bestMoveCount || 'N/A'} moves
            </div>
            {isAutoPlayingAll && (
              <div className="stat-item auto-play-progress">
                <strong>Auto-Play Progress:</strong>
                <div>{autoPlayProgress.current} / 31999</div>
                {currentAutoPlaySeed && (
                  <div><strong>Current Seed:</strong> {currentAutoPlaySeed}</div>
                )}
                <div className="progress-percentage">
                  {autoPlayProgress.current > 0 ? 
                    `${((autoPlayProgress.current / 31999) * 100).toFixed(2)}%` : 
                    '0.00%'}
                </div>
                <div className="progress-bar">
                  <div 
                    className="progress-fill" 
                    style={{ 
                      width: `${autoPlayProgress.current > 0 ? (autoPlayProgress.current / 31999) * 100 : 0}%`,
                      minWidth: autoPlayProgress.current > 0 ? '2px' : '0px' // Ensure visibility
                    }}
                  />
                </div>
              </div>
            )}
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

        <div className={`game-board ${isPlaybackMode ? 'playback-mode' : ''} ${isPlaybackMode && isPlaying ? 'playback-active' : ''} ${playbackSpeed === 0 ? 'trex-speed' : ''}`} role="main" aria-label="Freecell game board">
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
            onClick={() => {
              setGameWon(false);
              startNewGame(); // Deal a new random game
            }} 
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

      {/* Toast notifications */}
      <div className="toast-container">
        {toasts.map((toast) => (
          <Toast
            key={toast.id}
            message={toast.message}
            type={toast.type}
            onClose={() => removeToast(toast.id)}
          />
        ))}
      </div>
    </div>
  );
};

export default App;
