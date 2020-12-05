CREATE TABLE GAME_MOVES( id serial PRIMARY KEY,
                         user_id uuid REFERENCES USERS(id) NOT NULL,
                         game_id INT REFERENCES GAMES(id) NOT NULL,
                         umove SMALLINT [7] NOT NULL );