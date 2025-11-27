-- Write your PostgreSQL query statement below
select 
    contest_id,
    round(count(*)*100.0/(select count(*) from users), 2) as percentage
from register
group by contest_id
order by percentage DESC, contest_id 

