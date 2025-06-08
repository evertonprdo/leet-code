-- 1795. Rearrange Products Table: https://leetcode.com/problems/rearrange-products-table

CREATE TABLE IF NOT EXISTS Sales (
    sale_id int,
    product_id int,
    year int,
    quantity int,
    price int
);

CREATE TABLE IF NOT EXISTS Product (
    product_id int,
    product_name varchar(10)
);

-- # Solutions

CREATE OR REPLACE VIEW Solution AS
    SELECT
        p.product_name,
        s.year,
        s.price

    FROM Sales AS s
    LEFT JOIN Product AS p
    ON s.product_id = p.product_id
;

-- Example

TRUNCATE TABLE Sales;
INSERT INTO Sales (sale_id, product_id, year, quantity, price)
VALUES 
    ('1', '100', '2008', '10', '5000'),
    ('2', '100', '2009', '12', '5000'),
    ('7', '200', '2011', '15', '9000')
;

TRUNCATE TABLE Product;
INSERT INTO Product (product_id, product_name) 
VALUES 
    ('100', 'Nokia'),
    ('200', 'Apple'),
    ('300', 'Samsung')
;

SELECT * FROM Solution;

-- Clean db

DROP VIEW IF EXISTS Solution;

DROP TABLE IF EXISTS Sales;
DROP TABLE IF EXISTS Product;
