-- Your SQL goes here

CREATE TABLE santas (
    id INT GENERATED ALWAYS AS IDENTITY,
    group_id INT NOT NULL REFERENCES groups(id) ON DELETE CASCADE,
    santa_id INT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    recipient_id INT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    PRIMARY KEY(id),
    CONSTRAINT unique_set_santas UNIQUE(group_id, santa_id, recipient_id)
);
