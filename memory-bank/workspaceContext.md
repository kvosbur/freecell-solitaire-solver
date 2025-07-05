# Workspace Context: Component Relationships and Dependencies

## Component Dependency Graph

```
game-engine (library)
    ↑
    ├── solver (binary) 
    └── appAutomation (standalone)
```

## Component Relationships

### game-engine (Foundation Library)
- **Role**: Core shared library providing FreeCell game logic
- **Dependencies**: None (pure Rust library)
- **Consumers**: solver
- **Key Exports**: GameState, Card, Move, game rules, validation logic
- **Status**: Foundational implementation complete


### solver (Algorithm Application)
- **Role**: High-performance, multi-strategy, automated solution finder for FreeCell deals.
- **Dependencies**: game-engine
- **Integration Points**: Uses GameState for state validation, Move generation, and leverages a sophisticated strategy pattern for solving.
- **Current Status**: Advanced implementation with 5 solving strategies, a CLI, and a benchmarking harness.
- **Development Priority**: High (Optimization and Integration).

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
- solver and appAutomation: Solution format compatibility needed
- All components: Consistent error handling and data structures

### Testing Strategy
- **Unit Tests**: Each component tests its own functionality
- **Integration Tests**: Test component interactions through game-engine
- **Workspace Tests**: Ensure all components build and work together

## Current Development Focus

### Active Component: game-engine/
- Preparing for v0.2.0 API Refinement
- Building a pure, focused game engine library
- Priority: Complete the breaking changes for v0.2.0

### Next Steps
1. Complete game-engine v0.2.0 API Refinement
2. Begin solver algorithm development
3. Design solution format for appAutomation integration
4. Implement mobile automation system

## Workspace Health Indicators
- ✅ All components compile successfully
- ✅ Shared library pattern working correctly
- ✅ Clear dependency relationships established
- 🔄 Integration testing needed as components mature
- ⏳ Cross-component communication patterns to be established
