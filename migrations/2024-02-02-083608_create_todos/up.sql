CREATE TABLE todos
(
    id        SERIAL PRIMARY KEY,

    title     VARCHAR(255) NOT NULL,
    completed BOOLEAN      NOT NULL DEFAULT FALSE
);
