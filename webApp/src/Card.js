import React from 'react';

const Card = ({ 
  card, 
  onClick, 
  selected, 
  style, 
  tabIndex,
  onKeyDown,
  'aria-label': ariaLabel 
}) => {
  if (!card) return null;

  const handleKeyDown = (e) => {
    if (e.key === 'Enter' || e.key === ' ') {
      e.preventDefault();
      onClick && onClick();
    }
    onKeyDown && onKeyDown(e);
  };

  return (
    <div
      className={`card ${card.color} ${selected ? 'selected' : ''}`}
      data-card-id={card.id}
      data-card-rank={card.rank}
      data-card-suit={card.suit}
      onClick={onClick}
      onKeyDown={handleKeyDown}
      tabIndex={tabIndex}
      style={style}
      role="button"
      aria-label={ariaLabel || `${card.rank} of ${card.suit === '♥' ? 'Hearts' : card.suit === '♦' ? 'Diamonds' : card.suit === '♣' ? 'Clubs' : 'Spades'}`}
    >
      <div className="card-top">
        {card.rank}{card.suit}
      </div>
      <div className="card-bottom">
        {card.rank}{card.suit}
      </div>
    </div>
  );
};

export default Card;
