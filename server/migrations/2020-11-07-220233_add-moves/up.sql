CREATE TABLE GAME_MOVES( id serial PRIMARY KEY,
                         game_id INT REFERENCES GAMES(id) NOT NULL,
                         src SMALLINT [3] NOT NULL,
                         dest SMALLINT [3] NOT NULL,
                         user_id UUID REFERENCES USERS(id) NOT NULL,
                         figure SMALLINT NOT NULL);