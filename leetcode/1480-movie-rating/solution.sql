-- Write your PostgreSQL query statement below
with avg_rating as (
    select m.title, avg(rating) as avg_rating
    from movierating mr
    join movies m on mr.movie_id = m.movie_id
    where created_at between '2020-02-01'::date and '2020-02-29'::date
    group by m.title
    order by avg_rating DESC, m.title
    limit 1
)

(select name as results
from users u
join MovieRating mr on u.user_id = mr.user_id
group by name
order by count(*) DESC, name
limit 1)

union all

select title as results
from avg_rating

