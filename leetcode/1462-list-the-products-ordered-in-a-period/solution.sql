-- Write your PostgreSQL query statement below
select product_name, sum(unit) as unit
from products p
join orders o on p.product_id = o.product_id
where order_date between '2020-02-01'::date and '2020-02-29'::date
group by product_name
having sum(unit) >= 100
