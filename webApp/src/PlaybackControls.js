import React from 'react';

const PlaybackControls = ({ 
  isPlaying, 
  onPlay, 
  onPause, 
  onStepForward,
  onStepBackward,
  onSpeedChange,
  currentMove,
  totalMoves,
  speed
}) => {
  return (
    <div className="playback-controls">
      <button 
        onClick={onStepBackward}
        disabled={currentMove === 0}
        className="btn btn-secondary"
        title="Step backward"
      >
        ⏮
      </button>
      
      {isPlaying ? (
        <button onClick={onPause} className="btn btn-primary" title="Pause playback">
          ⏸ Pause
        </button>
      ) : (
        <button 
          onClick={onPlay} 
          className="btn btn-primary" 
          title="Play solution"
          disabled={currentMove >= totalMoves}
        >
          ▶ Play
        </button>
      )}
      
      <button 
        onClick={onStepForward}
        disabled={currentMove >= totalMoves}
        className="btn btn-secondary"
        title="Step forward"
      >
        ⏭
      </button>
      
      <div className="speed-control">
        <label htmlFor="speed-slider">Speed: </label>
        <div className="speed-presets">
          <button 
            className="speed-preset" 
            onClick={() => onSpeedChange(100)}
            title="T-Rex Speed - Very Fast"
          >
            🦖
          </button>
          <button 
            className="speed-preset" 
            onClick={() => onSpeedChange(500)}
            title="Dino Trot - Normal"
          >
            🦕
          </button>
          <button 
            className="speed-preset" 
            onClick={() => onSpeedChange(2000)}
            title="Dino Egg Hatch - Slow"
          >
            🥚
          </button>
          <button 
            className="speed-preset" 
            onClick={() => onSpeedChange(5000)}
            title="Fossil Study - Very Slow"
          >
            🦴
          </button>
        </div>
        <input
          id="speed-slider"
          type="range"
          min="100"
          max="10000"
          step="100"
          value={speed}
          onChange={(e) => onSpeedChange(Number(e.target.value))}
        />
        <span className="speed-value">
          {speed < 1000 ? `${speed}ms` : `${(speed/1000).toFixed(1)}s`}
        </span>
      </div>
      
      <div className="progress">
        <span className="move-counter">
          Move {currentMove} / {totalMoves}
        </span>
        <div className="progress-bar">
          <div 
            className="progress-fill" 
            style={{ width: `${totalMoves > 0 ? (currentMove / totalMoves) * 100 : 0}%` }}
          />
        </div>
      </div>
    </div>
  );
};

export default PlaybackControls;