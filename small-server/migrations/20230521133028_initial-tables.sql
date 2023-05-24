CREATE TABLE FRUIT (ID bigserial,
                    FRUIT_NAME VARCHAR(100) NOT NULL,
                                            COLOR_RED smallint NOT NULL,
                                                               COLOR_GREEN smallint NOT NULL,
                                                                                    COLOR_BLUE smallint NOT NULL,
                                                                                                        FRUIT_WEIGHT INTEGER NOT NULL,
                                                                                                                             PRIMARY KEY(ID));


CREATE TABLE PERSON (ID bigserial,
                     PERSON_NAME VARCHAR(100) NOT NULL,
                                              AGE INTEGER NOT NULL,
                                                          EMAIL VARCHAR(100) NOT NULL,
                                                                             PRIMARY KEY(ID));


CREATE TABLE FRUIT_SALAD (ID bigserial,
                          SALAD_NAME VARCHAR(100) NOT NULL,
                                                  ID_CREATOR bigserial,
                                                  PRIMARY KEY(ID),
                          FOREIGN KEY(ID_CREATOR) REFERENCES PERSON(ID));


CREATE TABLE SALAD_INGREDIENTS (ID bigserial,
                                ID_SALAD bigserial NOT NULL,
                                                   ID_FRUIT bigserial,
                                                   PRIMARY KEY(ID),
                                FOREIGN KEY (ID_SALAD) REFERENCES FRUIT_SALAD(ID),
                                FOREIGN KEY (ID_FRUIT) REFERENCES FRUIT(ID));