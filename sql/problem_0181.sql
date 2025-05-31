--  181. Employees Earning More Than Their Managers: https://leetcode.com/problems/employees-earning-more-than-their-managers

CREATE TABLE IF NOT EXISTS Employee (
    id          INT PRIMARY KEY,
    name        VARCHAR(4) NOT NULL,
    salary      INT NOT NULL,
    managerId   INT
);

-- ## Solutions

CREATE OR REPLACE VIEW Solution AS
    SELECT e1.name AS Employee

    FROM Employee AS e1
    INNER JOIN Employee AS e2 ON e2.id = e1.managerId

    WHERE e1.salary > e2.salary
;

-- Best postgreSQL runtime solution:

CREATE OR REPLACE VIEW OldSchoolSolution AS
    SELECT e.name AS Employee

    FROM Employee e, Employee m
    WHERE e.managerId = m.id AND e.salary > m.salary
;

-- Example 1:

INSERT INTO Employee (id, name, salary, managerId)
VALUES
(1, 'Joe',  70000, 3),
(2, 'Henr', 80000, 4),
(3, 'Sam',  60000, Null),
(4, 'Max',  90000, Null)
ON CONFLICT (id) DO NOTHING;

SELECT * FROM Solution;
SELECT * FROM OldSchoolSolution;

-- Clean db
DROP VIEW IF EXISTS Solution;
DROP VIEW IF EXISTS OldSchoolSolution;

DROP TABLE IF EXISTS Employee;