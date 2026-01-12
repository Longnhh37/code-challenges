import pandas as pd

def find_primary_department(employee: pd.DataFrame) -> pd.DataFrame:
    employee['dep_count'] = (
        employee
        .groupby('employee_id')['employee_id']
        .transform('count')
    )

    df = employee[
        (employee['primary_flag'] == 'Y') | (employee['dep_count'] == 1)
    ]

    return df[['employee_id', 'department_id']]    
