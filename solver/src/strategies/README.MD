# Current means of evaluation
Iteratively walk back from a solution and see if the solution can be found after 10 minutes.
How good it does is the amount of moves it can walk back and still solve in that time

# Strategies

## strat1
Most basic dfs with nothing extra done besides killing trees that get too long based on exact length check
Evaluation moves: 24

## strat2
Add naive previous state caching. Still need too long of path check otherwise call stack is exceeded.
Evaluation moves: 28

## strat3
strat2 + minimizes clones by using single game state and undoing after recursive call
Limit hit not in execution time, but memory. It used up 40 gb which is the limit I have set for it and then it just went down to a standstill going no where.
Evaluation moves: 28

## strat4
strat3 + reduce state in hash set by packing state before saving. Still hit limit of ram at a higher move count
Evaluation moves: 32

## strat5
strat4 except set a cap on cache size so that ram isn't overrun. Allowed program to run for a long time, but still didn't get any further
Evaluation moves: 32

## strat6
Based on strat5 but with a fundamentally different approach to state tracking. Instead of maintaining a large cache of all previously visited states, only tracks states currently in the search path (ancestors) to prevent infinite cycles. This allows the algorithm to revisit states from different branches that might now be reachable with fewer moves or different context, potentially finding solutions that were missed by more aggressive pruning strategies.
Evaluation moves: 32

## strat7
Hybrid approach combining the best of strat5 and strat6. Uses both ancestor tracking for cycle detection (preventing infinite loops in the current path) and an LRU cache for efficient pruning of previously visited states. Additionally implements a canonical state representation that sorts tableau columns, freecells, and foundations to increase cache hit rates by recognizing isomorphic game states that differ only in ordering.
Evaluation moves: 46

## strat8
Enhanced version of strat7 with tableau column preference heuristic. Builds on all the optimizations from strat7 (hybrid ancestor tracking + LRU cache, canonical state representation) but adds intelligent move ordering that prioritizes moves from the same tableau column as the previous move. This encourages working within the same tableau column to create longer winning sequences more sooner, leading to significantly improved performance by sorting the search space through better heuristic guidance.
Evaluation moves: 85