-- Write your PostgreSQL query statement below
select department, employee, salary from ( 
    select d.name as department, e.name as employee, salary,
        dense_rank() over(partition by departmentID order by salary DESC) as top_salary
    from employee e
    join department d on e.departmentId = d.id
)
where top_salary <= 3

