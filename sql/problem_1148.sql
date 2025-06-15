-- 1148. Article Views I: https://leetcode.com/problems/article-views-i

CREATE TABLE IF NOT EXISTS Views (
    article_id  INT,
    author_id   INT,
    viewer_id   INT,
    view_date   DATE
);

-- # Solution

CREATE OR REPLACE VIEW Solution AS
    SELECT DISTINCT
        author_id AS id

    FROM Views WHERE author_id = viewer_id
    ORDER BY author_id ASC
;

-- Example

TRUNCATE TABLE Views;
INSERT INTO Views (article_id, author_id, viewer_id, view_date)
VALUES
    ('1', '3', '5', '2019-08-01'),
    ('1', '3', '6', '2019-08-02'),
    ('2', '7', '7', '2019-08-01'),
    ('2', '7', '6', '2019-08-02'),
    ('4', '7', '1', '2019-07-22'),
    ('3', '4', '4', '2019-07-21'),
    ('3', '4', '4', '2019-07-21')
;

SELECT * FROM Solution;

-- Clean db

DROP VIEW IF EXISTS Solution;
DROP TABLE IF EXISTS Views;