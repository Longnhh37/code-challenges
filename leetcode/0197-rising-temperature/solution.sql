-- Write your PostgreSQL query statement below
with prev_temp as (
    select *,
        lag(recordDate, 1) over(order by recordDate) as prev_date,
        lag(temperature, 1) over(order by recordDate) as previous_temperature
    from weather
)
select id 
from prev_temp
where temperature > previous_temperature and recordDate - prev_date = 1

