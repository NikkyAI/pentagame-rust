# Pentagame - Online

Pentagame online client & server

## Setup

You need [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html), [yarn](https://yarnpkg.com/getting-started/install) and the [diesel_cli](https://lib.rs/crates/diesel_cli) (only feature: `postgres` is required) as well as [GNU make](https://www.gnu.org/software/make/). Only bash is supported at the moment.

Start the setup process with cloning the github repository and entering the project folder.

You can then start the initial setup with: `make setup`
Now you need to configure your database credentials (see .github/ci/ci.toml as sample) in the fiole pentagame.toml in the project route:

Pentagame.toml:

```toml
[server]
ip = 'localhost'
port = 8080

[database]
user = 'pentagame-dev'
password = '<password>'
host = 'localhost'
port = 5432
database = 'pentagame-dev'

[auth]
file = 'NEW'
session = 24
```

When this is done you just need to run the database migrations and generate a new application key: `make db-setup generate`

Build and serve the application (binary is in `target/release/pentagame`): `make build serve`

And you're done.

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
