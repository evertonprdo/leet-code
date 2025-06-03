-- 183. Customers Who Never Order: https://leetcode.com/problems/customers-who-never-order

CREATE TABLE IF NOT EXISTS Customers (
    id INT PRIMARY KEY,
    name VARCHAR(255)
);

CREATE TABLE IF NOT EXISTS Orders (
    id INT PRIMARY KEY,
    customerId INT REFERENCES Customers(id)
);

-- # Solutions

CREATE OR REPLACE VIEW Solution AS
    SELECT 
        c.name AS Customers

    FROM Customers AS c
    LEFT JOIN Orders AS o
    ON c.id = o.customerId

    WHERE o.customerId IS NULL
;

CREATE OR REPLACE VIEW NotInSolution AS
    SELECT 
        name AS Customers
    
    FROM customers
    WHERE id NOT IN (
        SELECT customerId
        FROM orders
    )
;

-- Example 01

INSERT INTO Customers (id, name) 
VALUES
    ('1', 'Joe'),
    ('2', 'Henry'),
    ('3', 'Sam'),
    ('4', 'Max')
;

INSERT INTO Orders (id, customerId) 
VALUES 
    ('1', '3'),
    ('2', '1')
;

SELECT * FROM Solution;
SELECT * FROM NotInSolution;

-- Clean db

DROP VIEW IF EXISTS Solution;
DROP VIEW IF EXISTS NotInSolution;

DROP TABLE IF EXISTS Orders;
DROP TABLE IF EXISTS Customers;