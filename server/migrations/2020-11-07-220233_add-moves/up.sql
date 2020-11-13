CREATE TABLE MOVES( fnode SMALLINT NOT NULL,
                    ncounter SMALLINT NOT NULL,
                    snode SMALLINT NOT NULL,
                    id serial PRIMARY KEY );
CREATE TABLE GAME_MOVES( id serial PRIMARY KEY,
                         move_id INT REFERENCES MOVES(id) NOT NULL,
                         game_id INT REFERENCES GAMES(id) NOT NULL );