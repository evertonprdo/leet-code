-- 1741. Find Total Time Spent by Each Employee: https://leetcode.com/problems/find-total-time-spent-by-each-employee

CREATE TABLE IF NOT EXISTS Employees (
    emp_id INT,
    event_day DATE,
    in_time INT,
    out_time INT
);

CREATE OR REPLACE VIEW Solution AS
    SELECT
        event_day AS day,
        emp_id,
        SUM(out_time - in_time) AS total_time
    
    FROM Employees
    GROUP BY day, emp_id ORDER BY event_day, emp_id ASC
;

-- Example 01

TRUNCATE TABLE Employees;
INSERT INTO Employees (emp_id, event_day, in_time, out_time) 
VALUES
    ('1', '2020-11-28', '4', '32'),
    ('1', '2020-11-28', '55', '200'),
    ('1', '2020-12-3', '1', '42'),
    ('2', '2020-11-28', '3', '33'),
    ('2', '2020-12-9', '47', '74')
;

SELECT * FROM Solution;

-- Clean db

DROP VIEW IF EXISTS Solution;
DROP TABLE IF EXISTS Employees;