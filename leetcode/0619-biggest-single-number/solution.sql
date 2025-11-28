-- Write your PostgreSQL query statement below
select case when true then (
    select num 
    from MyNumbers
    group by num
    having count(*) = 1
    order by num DESC
    limit 1
) else null end as num
