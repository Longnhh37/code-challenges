-- Write your PostgreSQL query statement below
select name, bonus
from employee e
left join bonus b on e.empID = b.empId
where bonus < 1000 or bonus is null
