-- Write your PostgreSQL query statement below
select distinct num as ConsecutiveNums from (
    select 
        lag(num, 1) over(order by id) as prev_num,
        num,
        lead(num, 1) over(order by id) as next_num
    from logs
)
where prev_num = num and num = next_num
