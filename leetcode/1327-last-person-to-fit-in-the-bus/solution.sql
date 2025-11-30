-- Write your PostgreSQL query statement below
select person_name 
from (
    select person_name, sum(weight) over(order by turn) as load
    from queue
) 
where load <=1000
order by load DESC
limit 1
