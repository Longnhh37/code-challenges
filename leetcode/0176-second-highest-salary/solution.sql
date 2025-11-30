select COALESCE((
    select distinct salary 
    from employee
    order by salary DESC
    offset 1
    limit 1
), null) as SecondHighestSalary
