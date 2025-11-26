-- Write your PostgreSQL query statement below
select name from Customer
where referee_id != 2 OR referee_id isnull
