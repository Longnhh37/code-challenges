-- Write your PostgreSQL query statement below
select activity_date as day, count(distinct user_id) as active_users
from activity
where activity_date between '2019-07-27'::date - interval '29 days' and '2019-07-27'
group by day
