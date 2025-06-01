-- 1757. Recyclable and Low Fat Products: https://leetcode.com/problems/recyclable-and-low-fat-products

CREATE TYPE yes_no AS ENUM ('Y', 'N');

CREATE TABLE IF NOT EXISTS Products (
    product_id INT PRIMARY KEY,
    low_fats yes_no,
    recyclable yes_no
);

-- ## Solutions

CREATE OR REPLACE VIEW Solution AS
    SELECT product_id 
    FROM Products
    WHERE low_fats = 'Y' AND recyclable = 'Y'
;

-- Example 01

INSERT INTO Products (product_id, low_fats, recyclable) 
VALUES 
    (0, 'Y', 'N'),
    (1, 'Y', 'Y'),
    (2, 'N', 'Y'),
    (3, 'Y', 'Y'),
    (4, 'N', 'N')
;

SELECT * FROM Solution;

-- Clean db

DROP VIEW IF EXISTS Solution;
DROP TABLE IF EXISTS Products;

DROP TYPE IF EXISTS yes_no;