-- 175. Combine Two Tables: https://leetcode.com/problems/combine-two-tables

CREATE TABLE IF NOT EXISTS Person (
    personId    INT PRIMARY KEY,
    lastName    VARCHAR(40) NOT NULL,
    firstName   VARCHAR(40) NOT NULL
);

CREATE TABLE IF NOT EXISTS Address (
    addressId   INT PRIMARY KEY,
    personId    INT NOT NULL,
    city        VARCHAR(40) NOT NULL,

    CONSTRAINT fk_person FOREIGN KEY (personId) 
        REFERENCES Person(personId)
);

-- # Solutions

CREATE OR REPLACE VIEW Solution AS
    SELECT 
        p.firstName, p.lastName, a.city, a.state

    FROM Person AS p 
    LEFT JOIN Address AS a 
        ON p.personId = a.personId
;

-- Example 1

INSERT INTO Person VALUES
(1, 'Wang', 'Allen'),
(2, 'Alice', 'Bob')
    ON CONFLICT (personId) DO NOTHING;

INSERT INTO Address VALUES
( 1, 2, 'New York City' , 'New York'  ),
( 2, 3, 'Leetcode'      , 'California');
    ON CONFLICT (addressId) DO NOTHING;

SELECT * FROM Solution;

-- Clean db

DROP VIEW IF EXISTS Solution;
DROP TABLE IF EXISTS Person;
DROP TABLE IF EXISTS Address;