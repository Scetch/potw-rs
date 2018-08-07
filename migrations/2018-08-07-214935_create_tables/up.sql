CREATE TABLE oauth (
    gid TEXT PRIMARY KEY NOT NULL,
    uid INTEGER NOT NULL,
    FOREIGN KEY(uid) REFERENCES user(uid)
);

CREATE TABLE user (
    id INTEGER PRIMARY KEY NOT NULL,
    sid TEXT NOT NULL,
    admin BOOLEAN NOT NULL
);

CREATE TABLE problem (
    id INTEGER PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    description TEXT NOT NULL
);

CREATE TABLE solution (
    id INTEGER PRIMARY KEY NOT NULL,
    pid INTEGER NOT NULL,
    uid INTEGER NOT NULL,
    language INTEGER NOT NULL,
    code TEXT NOT NULL,
    FOREIGN KEY(pid) REFERENCES problem(id),
    FOREIGN KEY(uid) REFERENCES user(id),
    FOREIGN KEY(language) REFERENCES language(id)
);

CREATE TABLE language (
    id INTEGER PRIMARY KEY NOT NULL,
    name TEXT NOT NULL
);
