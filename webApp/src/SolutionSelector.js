import React, { useState, useEffect } from 'react';

const SolutionSelector = ({ onSelectSolution, currentSeed }) => {
  const [solutions, setSolutions] = useState([]);
  const [selectedSolution, setSelectedSolution] = useState('');
  
  useEffect(() => {
    // In production, this would fetch from an API
    // For now, hardcode available solutions based on files in results folder
    const availableSolutions = Array.from({ length: 100 }, (_, i) => ({
      id: i + 1,
      filename: `${i + 1}.json`,
      label: `Game #${i + 1}`
    }));
    
    setSolutions(availableSolutions);
  }, []);
  
  const handleChange = (e) => {
    const value = e.target.value;
    setSelectedSolution(value);
    if (value) {
      onSelectSolution(value);
    }
  };
  
  return (
    <div className="solution-selector">
      <label htmlFor="solution-select">Load Solution: </label>
      <select 
        id="solution-select"
        value={selectedSolution} 
        onChange={handleChange}
        className="solution-select"
      >
        <option value="">Select a game...</option>
        {solutions.map(sol => (
          <option 
            key={sol.id} 
            value={sol.filename}
            className={currentSeed === sol.id ? 'current-game' : ''}
          >
            {sol.label} {currentSeed === sol.id ? '(current)' : ''}
          </option>
        ))}
      </select>
    </div>
  );
};

export default SolutionSelector;