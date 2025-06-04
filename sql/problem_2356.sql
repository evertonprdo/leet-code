-- 2356. Number of Unique Subjects Taught by Each Teacher: https://leetcode.com/problems/number-of-unique-subjects-taught-by-each-teacher

CREATE TABLE IF NOT EXISTS Teacher (
    teacher_id INT,
    subject_id INT, 
    dept_id INT
);

-- # Solutions

CREATE OR REPLACE VIEW Solution AS
    SELECT 
        teacher_id,
        COUNT(DISTINCT subject_id) AS cnt
    
    FROM Teacher
    GROUP BY teacher_id
;

-- Example 01

INSERT INTO Teacher (teacher_id, subject_id, dept_id) 
VALUES 
    ('1', '2', '3'),
    ('1', '2', '4'),
    ('1', '3', '3'),
    ('2', '1', '1'),
    ('2', '2', '1'),
    ('2', '3', '1'),
    ('2', '4', '1')
;

SELECT * FROM Solution;

-- Clean db

DROP VIEW IF EXISTS Solution;
DROP TABLE IF EXISTS Teacher;
