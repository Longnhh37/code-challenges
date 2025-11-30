with p1 as (   
    select product_id, new_price as price
    from products
    where (product_id, change_date) in (
        select product_id, max(change_date)
        from products
        where change_date <= '2019-08-16'::date
        group by product_id)
), p2 as (
    select distinct product_id, 10 as price
    from products
    where product_id not in (select product_id from p1)
)

select * from p1
union 
select * from p2
