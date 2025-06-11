-- 1587. Bank Account Summary II: https://leetcode.com/problems/bank-account-summary-ii

CREATE TABLE IF NOT EXISTS Users (account int, name varchar(20));
CREATE TABLE IF NOT EXISTS Transactions (trans_id int, account int, amount int, transacted_on date);

-- # Solution

CREATE OR REPLACE VIEW Solution AS
    SELECT
        u.name,
        SUM(t.amount) AS balance

    FROM Users AS u
    JOIN Transactions AS t
    ON u.account = t.account

    GROUP BY u.name HAVING SUM(t.amount) > 10000
;

-- Example 01

TRUNCATE TABLE Users;
INSERT INTO Users (account, name) 
VALUES 
    ('900001', 'Alice'),
    ('900002', 'Bob'),
    ('900003', 'Charlie');

TRUNCATE TABLE Transactions;
INSERT INTO Transactions (trans_id, account, amount, transacted_on)
VALUES
    ('1', '900001', '7000', '2020-08-01'),
    ('2', '900001', '7000', '2020-09-01'),
    ('3', '900001', '-3000', '2020-09-02'),
    ('4', '900002', '1000', '2020-09-12'),
    ('5', '900003', '6000', '2020-08-07'),
    ('6', '900003', '6000', '2020-09-07'),
    ('7', '900003', '-4000', '2020-09-11');

SELECT * FROM Solution;

-- Clean db

DROP VIEW IF EXISTS Solution;

DROP TABLE IF EXISTS Users;
DROP TABLE IF EXISTS Transactions;