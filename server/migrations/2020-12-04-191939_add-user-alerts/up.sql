CREATE TABLE ALERTS( id serial PRIMARY KEY,
                     user_id uuid REFERENCES USERS(id) NOT NULL,
                     header_type SMALLINT NOT NULL,
                     message TEXT NOT NULL );