with process_time_table as (
    select a1.machine_id, a1.process_id, (a2.timestamp - a1.timestamp) use_time 
    from activity a1
    join activity a2 on 
        a1.machine_id = a2.machine_id
        and a1.process_id = a2.process_id
        and a1.activity_type != a2.activity_type
    where a1.activity_type = 'start'
)

select machine_id, round(avg(use_time)::numeric, 3) as processing_time
from process_time_table
group by machine_id
