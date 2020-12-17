[![GitHub Workflow Status](https://img.shields.io/github/workflow/status/Penta-game/pentagame/Rust?style=for-the-badge) ](https://github.com/Penta-Game/pentagame/actions?query=workflow%3ARust) [![GitHub](https://img.shields.io/github/license/Penta-game/pentagame?style=for-the-badge)](https://github.com/Penta-Game/pentagame/blob/master/LICENSE)

![Plattform](https://img.shields.io/badge/Plattform-Linux-green?style=for-the-badge&logo=linux)

# Pentagame - Online

Pentagame online client & server

## Setup

You need [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html), [yarn](https://yarnpkg.com/getting-started/install) and the [diesel_cli](https://lib.rs/crates/diesel_cli) (only feature: `postgres` is required) as well as [GNU make](https://www.gnu.org/software/make/). Only bash is supported at the moment.

Start the setup process with cloning the github repository and entering the project folder.

You can then start the initial setup with: `make setup`
Now you need to configure your database credentials (see .github/ci/ci.toml as sample) in the file pentagame.toml in the project route:

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

To compile again you only need to run: `make serve` in `server/`

> Serve invokes a compilation & Packaging of logic

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

## Licensing

The pentagame brand and board design is owned by [Jan Suchanek](https://pentagame.org) and available under [CC-BY-NC](https://creativecommons.org/licenses/by-nc/3.0/de/). For any inqueries reagrding this brand or the board design please refer to him. 

The code and game implementation as well as architecture is licensed under [GPL v3.0-or-newer @ Cobalt](https://cobalt.rocks).
