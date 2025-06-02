-- 182. Duplicate Emails: https://leetcode.com/problems/duplicate-emails

CREATE TABLE IF NOT EXISTS Person (
    id INT PRIMARY KEY,
    email VARCHAR(255)
);

-- # Solutions
-- Hammer Solution (Where when you only have a hammer all problems are nails);
CREATE OR REPLACE VIEW Solution AS
    SELECT DISTINCT p1.email
    
    FROM Person AS p1
    INNER JOIN Person AS p2 ON p1.email = p2.email
    
    WHERE p1.id != p2.id
;

CREATE OR REPLACE VIEW OtherSolution AS
    SELECT email
    FROM (
        SELECT COUNT(id) AS amount, email
        FROM Person
        GROUP BY email
    )
    WHERE amount > 1
;

-- The best PostgreSQL runtime solution
CREATE OR REPLACE VIEW ExpectedSolution AS
    SELECT email
    FROM Person
    GROUP BY email
    HAVING COUNT(email) > 1
;

-- Example 01

INSERT INTO Person (id, email) 
VALUES
    ('1', 'a@b.com'),
    ('2', 'c@d.com'),
    ('3', 'a@b.com')
;

SELECT * FROM Solution;
SELECT * FROM ExpectedSolution;
SELECT * FROM OtherSolution;

-- Clean db

DROP VIEW IF EXISTS Solution;
DROP VIEW IF EXISTS ExpectedSolution;
DROP VIEW IF EXISTS OtherSolution;

DROP TABLE IF EXISTS Person;