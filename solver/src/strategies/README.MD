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