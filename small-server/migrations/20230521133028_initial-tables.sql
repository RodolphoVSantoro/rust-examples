CREATE TABLE FRUIT (ID bigserial,
                    FRUIT_NAME "char"[], COLOR "char"[], FRUIT_WEIGHT INTEGER, PRIMARY KEY(ID));


CREATE TABLE PERSON (ID bigserial,
                     PERSON_NAME "char"[], age INTEGER, EMAIL "char"[],PRIMARY KEY(ID));


CREATE TABLE FAVORITE_SALAD (ID bigserial,
                             ID_CREATOR bigserial,
                             ID_FRUIT bigserial,
                             PRIMARY KEY(ID),
                             FOREIGN KEY (ID_CREATOR) REFERENCES PERSON(ID),
                             FOREIGN KEY (ID_FRUIT) REFERENCES FRUIT(ID));