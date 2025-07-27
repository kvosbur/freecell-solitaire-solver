export class PlaybackController {
  constructor() {
    this.moves = [];
    this.currentMoveIndex = 0;
    this.isPlaying = false;
    this.playbackSpeed = 500; // ms between moves
    this.abortController = null;
  }

  async loadSolution(solutionData) {
    this.moves = solutionData.solution_moves;
    this.currentMoveIndex = 0;
  }

  async executeMove(move) {
    const { source, destination, card_count = 1 } = move;
    
    console.log('Executing move:', { source, destination, card_count });
    
    // Step 1: Find and click source
    const sourceElement = this.findSourceElement(source, card_count);
    if (!sourceElement) {
      console.error('Failed to find source element');
      console.error('Source:', source);
      console.error('Available tableau columns:', document.querySelectorAll('.tableau-column').length);
      console.error('Available freecells:', document.querySelectorAll('.freecells .card-slot').length);
      throw new Error(`Cannot find source element for move ${JSON.stringify(move)}`);
    }
    
    console.log('Clicking source:', sourceElement);
    console.log('Source element classes:', sourceElement.className);
    sourceElement.click();
    
    // Step 2: Wait for selection state and verify card is selected
    await this.waitForSelection();
    
    // Step 3: Wait for the user-configured delay to show the selection
    await this.delay(this.playbackSpeed);
    
    // Step 4: Find and click destination
    const destElement = this.findDestinationElement(destination);
    if (!destElement) {
      console.error('Failed to find destination element');
      console.error('Destination:', destination);
      throw new Error(`Cannot find destination element for move ${JSON.stringify(move)}`);
    }
    
    console.log('Clicking destination:', destElement);
    console.log('Destination element classes:', destElement.className);
    destElement.click();
    
    // Step 5: Wait for move to complete (short wait, just for state update)
    await this.waitForMoveCompletion();
  }

  async waitForSelection() {
    let attempts = 0;
    while (attempts < 10) {
      await this.waitForNextTick();
      const selectedCards = document.querySelectorAll('.card.selected');
      if (selectedCards.length > 0) {
        console.log('Card selected successfully');
        return;
      }
      attempts++;
    }
    console.warn('Card selection may have failed');
  }

  async waitForMoveCompletion() {
    let attempts = 0;
    while (attempts < 20) {
      await this.waitForNextTick();
      const selectedCards = document.querySelectorAll('.card.selected');
      if (selectedCards.length === 0) {
        console.log('Move completed successfully');
        return;
      }
      attempts++;
    }
    console.warn('Move completion may have failed - cards still selected');
  }

  findSourceElement(source, cardCount = 1) {
    if (source.Tableau) {
      // Find the correct card in tableau column
      const columnIndex = source.Tableau.index;
      const columns = document.querySelectorAll('.tableau-column');
      const column = columns[columnIndex];
      if (!column) {
        console.error(`Tableau column ${columnIndex} not found. Total columns: ${columns.length}`);
        return null;
      }
      
      const cards = column.querySelectorAll('.card');
      console.log(`Found ${cards.length} cards in tableau column ${columnIndex}`);
      
      // For tableau, we need the card that's cardCount from the bottom
      const targetCard = cards[cards.length - cardCount];
      if (targetCard) {
        console.log(`Selected card: ${targetCard.dataset.cardId}`);
      }
      return targetCard;
    } else if (source.Freecell) {
      // Find card in specific freecell
      const freecells = document.querySelectorAll('.freecells .card-slot');
      const freecell = freecells[source.Freecell.index];
      return freecell ? freecell.querySelector('.card') : null;
    }
    
    return null;
  }

  findDestinationElement(destination) {
    if (destination.Tableau) {
      const columnIndex = destination.Tableau.index;
      const columns = document.querySelectorAll('.tableau-column');
      const column = columns[columnIndex];
      if (!column) {
        console.error(`Tableau column ${columnIndex} not found. Total columns: ${columns.length}`);
        return null;
      }
      
      // Try to find the last card in the column to click on
      const cards = column.querySelectorAll('.card');
      
      if (cards.length > 0) {
        console.log(`Clicking on last card in column ${columnIndex}`);
        return cards[cards.length - 1]; // Click last card
      } else {
        // Empty column - click the slot
        console.log(`Column ${columnIndex} is empty, clicking slot`);
        return column.querySelector('.card-slot');
      }
    } else if (destination.Freecell) {
      // Click the freecell slot
      const freecells = document.querySelectorAll('.freecells .card-slot');
      return freecells[destination.Freecell.index];
    } else if (destination.Foundation) {
      // Click the foundation slot
      const foundations = document.querySelectorAll('.foundations .card-slot');
      return foundations[destination.Foundation.index];
    }
    
    return null;
  }

  waitForNextTick() {
    return new Promise(resolve => setTimeout(resolve, 200));
  }

  async play() {
    this.isPlaying = true;
    this.abortController = new AbortController();
    
    while (this.currentMoveIndex < this.moves.length && this.isPlaying) {
      try {
        await this.executeMove(this.moves[this.currentMoveIndex]);
        this.currentMoveIndex++;
        
        // No additional delay needed - timing is handled in executeMove
      } catch (error) {
        console.error('Playback error:', error);
        this.pause();
        break;
      }
    }
    
    this.isPlaying = false;
  }

  pause() {
    this.isPlaying = false;
    if (this.abortController) {
      this.abortController.abort();
    }
  }

  async stepForward() {
    if (this.currentMoveIndex < this.moves.length) {
      await this.executeMove(this.moves[this.currentMoveIndex]);
      this.currentMoveIndex++;
    }
  }

  async stepBackward() {
    // This would require undo functionality
    console.warn('Step backward not implemented - requires undo support');
  }

  delay(ms) {
    return new Promise(resolve => setTimeout(resolve, ms));
  }

  setSpeed(speed) {
    this.playbackSpeed = speed;
  }

  reset() {
    this.currentMoveIndex = 0;
    this.isPlaying = false;
  }
}