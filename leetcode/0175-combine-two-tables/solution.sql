-- Write your PostgreSQL query statement below
SELECT p.firstName, p.lastName, a.city, a.state
FROM Person p
FULL JOIN Address a
ON p.personId = a.personID
WHERE p.firstName IS NOT NULL AND p.lastName IS NOT NULL
