# Pentagame - Online

Pentagame online client

## States

Game Players are stored with an order. Player 1-5 = pid. This is order is based around the `rank` attribute of the UserGame

- 0 (not running): Waiting for players to join
- 1-5 (pid): Waiting for move of {pid}
- 6-10 (pid-5): Waiting for {pid} to set stopper
- 11-16 (10 + winner amount) (finished): ranking is changed so that winners are at the top. Winner amount is the used for evaluating based on ranking.

## Database

The game state system is done with a recalculation based on te last found moves. To keep search cost low all moves of a game are removed on finish.

## State evaluation:

### Game Creation

- Waiting for players until started by host

### Game Running

1. Waiting for player {pid}
2. Validating move
3. Validating finished

   1. Winner(s) & players are kept in memory
   2. Remove all gamemoves
   3. Change Ranking to reflect winner(s)
   4. set game state to 11-16

4. Adding Move & GameMove & Change State
5. Sending signal
