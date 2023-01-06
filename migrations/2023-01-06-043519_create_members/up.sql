-- Your SQL goes here

CREATE TABLE members (
    id INT GENERATED ALWAYS AS IDENTITY,
    user_id INT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    group_id INT NOT NULL REFERENCES groups(id) ON DELETE CASCADE,
    is_admin BOOLEAN NOT NULL DEFAULT FALSE,
    PRIMARY KEY(id),
    CONSTRAINT unique_set_members UNIQUE(user_id, group_id)
);
