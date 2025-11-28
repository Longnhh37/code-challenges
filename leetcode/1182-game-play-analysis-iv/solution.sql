-- Write your PostgreSQL query statement below
with datediff as (
    select  
        player_id, event_date,
        (lead(event_date, 1) over(
            partition by player_id 
            order by event_date)
            - event_date)
        as consecutive_day
    from activity

)
select 
    round(
        count(*)*1.0 
        /
        (select count(distinct player_id) from activity)
    , 2) as fraction
from datediff
where consecutive_day = 1
    and (player_id, event_date) in (select player_id, min(event_date) from activity group by player_id)
