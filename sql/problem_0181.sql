--  181. Employees Earning More Than Their Managers: https://leetcode.com/problems/employees-earning-more-than-their-managers

SELECT e1.name AS Employee

FROM Employee AS e1
RIGHT JOIN Employee AS e2 ON e2.id = e1.managerId

WHERE e1.salary > e2.salary;

-- Best postgreSQL runtime solution:

SELECT e.name AS Employee

FROM Employee e, Employee m

WHERE e.managerId = m.id AND e.salary > m.salary