
with amount_by_day as (
    select visited_on, sum(amount) as total_amount
    from customer
    group by visited_on
), 
moving_average as (
    SELECT
        visited_on,
        sum(total_amount) over(
            order by visited_on
            rows between 6  preceding and current row
            ) as amount,
        row_number() over(order by visited_on) as rnk
    FROM amount_by_day
)

select visited_on, amount, round(amount/7.0, 2) as average_amount
from moving_average
where rnk >= 7



