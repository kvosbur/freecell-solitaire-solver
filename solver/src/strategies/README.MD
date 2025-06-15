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