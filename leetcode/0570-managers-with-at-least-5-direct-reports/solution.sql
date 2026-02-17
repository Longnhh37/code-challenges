# Write your MySQL query statement below
SELECT
    name
FROM Employee t1
WHERE id IN(

    SELECT
        managerId

    FROM Employee
    GROUP BY 1
    HAVING COUNT(*) >= 5)
