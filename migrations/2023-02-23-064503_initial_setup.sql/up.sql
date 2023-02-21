CREATE TABLE user (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    username VARCHAR(50) UNIQUE NOT NULL,
    password VARCHAR(100) NOT NULL
);

-- username/password admin
INSERT INTO user VALUES (1,'admin', 'd033e22ae348aeb5660fc2140aec35850c4da997');

CREATE TABLE vet (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    name VARCHAR(100) NOT NULL
);

CREATE TABLE pet (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    name VARCHAR(100) NOT NULL,
    owner_name VARCHAR(100) NOT NULL,
    owner_phone VARCHAR(20) NOT NULL,
    age INT NOT NULL,
    pet_type INT NOT NULL,
    vet_id INT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    created_by INT NOT NULL,
    FOREIGN KEY (vet_id) REFERENCES vet(id) ON DELETE CASCADE,
    FOREIGN KEY (created_by) REFERENCES user(id)
);

CREATE TABLE visit (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    pet_id INT NOT NULL,
    vet_id INT NOT NULL,
    visit_date DATE NOT NULL,
    notes text,
    FOREIGN KEY (pet_id) REFERENCES pet(id) ON DELETE CASCADE,
    FOREIGN KEY (vet_id) REFERENCES vet(id) ON DELETE CASCADE
);


INSERT INTO vet (id, name) VALUES(1, 'James Carter');
INSERT INTO vet (id, name) VALUES(2, 'Helen Leary');
INSERT INTO vet (id, name) VALUES(3, 'Linda Douglas');
INSERT INTO vet (id, name) VALUES(4, 'Rafael Ortega');

INSERT INTO pet (id, name, owner_name, owner_phone, age, pet_type, vet_id, created_at, created_by)
VALUES(1, 'Felix', 'John Doe', '333', 3, 1, 1, '2022-01-01 9:00:00', 1);

INSERT INTO pet (id, name, owner_name, owner_phone, age, pet_type, vet_id, created_at, created_by)
VALUES(2, 'Chloe', 'Peter Falk', '333', 5, 2, 1, '2022-01-01 9:00:00', 1);

INSERT INTO pet (id, name, owner_name, owner_phone, age, pet_type, vet_id, created_at, created_by)
VALUES(3, 'Iru', 'Dr.Falken', '333', 8, 2, 3, '2022-01-01 9:00:00', 1);

INSERT INTO pet (id, name, owner_name, owner_phone, age, pet_type, vet_id, created_at, created_by)
VALUES(4, 'Willy', 'Harold Davis', '333', 10, 2, 1, '2022-01-01 9:00:00', 1);
