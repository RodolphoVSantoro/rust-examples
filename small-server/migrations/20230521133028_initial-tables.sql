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


CREATE TABLE FAVORITE_SALADS (ID_SALAD bigserial,
                              ID_CREATOR bigserial,
                              ID_FRUIT bigserial,
                              PRIMARY KEY(ID_SALAD),
                              FOREIGN KEY (ID_CREATOR) REFERENCES PERSON(ID),
                              FOREIGN KEY (ID_FRUIT) REFERENCES FRUIT(ID));