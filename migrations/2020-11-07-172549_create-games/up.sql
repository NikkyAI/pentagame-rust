CREATE TABLE GAMES( id serial PRIMARY KEY,
                    name TEXT NOT NULL,
                    description TEXT,
                    host_id uuid REFERENCES USERS(id) NOT NULL );
CREATE TABLE USER_GAMES( id serial PRIMARY KEY,
                         player_id uuid REFERENCES USERS(id) NOT NULL,
                         game_id INT REFERENCES GAMES(id) NOT NULL );