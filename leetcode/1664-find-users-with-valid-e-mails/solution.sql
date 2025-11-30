-- Write your PostgreSQL query statement below
select * from users
where mail ~ '^[a-zA-Z][A-Za-z0-9_.-]*@leetcode\.com$'
