# Workspace Context: Component Relationships and Dependencies

## Component Dependency Graph

```
game-engine (library)
    ↑
    ├── game (binary)
    ├── solver (binary) 
    └── appAutomation (standalone)
```

## Component Relationships

### game-engine (Foundation Library)
- **Role**: Core shared library providing FreeCell game logic
- **Dependencies**: None (pure Rust library)
- **Consumers**: game, solver
- **Key Exports**: GameState, Card, Move, game rules, validation logic
- **Status**: Foundational implementation complete

### game (Interactive Application)
- **Role**: User-facing FreeCell game interface
- **Dependencies**: game-engine
- **Integration Points**: Uses GameState for game logic, Move validation
- **Current Status**: Basic structure in place, needs UI implementation
- **Development Priority**: High (current focus)

### solver (Algorithm Application)
- **Role**: Automated solution finder for FreeCell deals
- **Dependencies**: game-engine
- **Integration Points**: Uses GameState for state validation, Move generation
- **Current Status**: Skeleton created, algorithm implementation pending
- **Development Priority**: Medium (after game completion)

### appAutomation (Standalone System)
- **Role**: Mobile app interaction and automation
- **Dependencies**: External (mobile apps), potentially solver output
- **Integration Points**: Consumes solution sequences, interfaces with mobile apps
- **Current Status**: Directory exists, implementation pending
- **Development Priority**: Low (final integration phase)

## Shared Patterns and Standards

### Rust Workspace Configuration
- **Edition**: 2021 across all components
- **Build System**: Cargo workspace with shared target directory
- **Dependency Management**: Centralized in workspace root when possible

### Code Organization
- **Library Pattern**: game-engine as pure library (no main.rs)
- **Binary Pattern**: Applications have main.rs and focused functionality
- **Module Structure**: Clear separation between game logic and application logic

### Development Workflow
- **Build**: `cargo build --workspace` for all components
- **Test**: `cargo test --workspace` for comprehensive testing
- **Run**: `cargo run --bin <component>` for individual applications

## Integration Considerations

### Game Engine API Stability
- Changes to game-engine affect all consumers
- API changes require coordination across components
- Maintain backward compatibility when possible

### Cross-Component Communication
- game and solver: Both consume game-engine API
- solver and appAutomation: Solution format compatibility needed
- All components: Consistent error handling and data structures

### Testing Strategy
- **Unit Tests**: Each component tests its own functionality
- **Integration Tests**: Test component interactions through game-engine
- **Workspace Tests**: Ensure all components build and work together

## Current Development Focus

### Active Component: game/
- Implementing interactive FreeCell gameplay
- Building on established game-engine foundation
- Priority: Complete basic gameplay loop

### Next Steps
1. Complete game application implementation
2. Begin solver algorithm development
3. Design solution format for appAutomation integration
4. Implement mobile automation system

## Workspace Health Indicators
- ✅ All components compile successfully
- ✅ Shared library pattern working correctly
- ✅ Clear dependency relationships established
- 🔄 Integration testing needed as components mature
- ⏳ Cross-component communication patterns to be established
