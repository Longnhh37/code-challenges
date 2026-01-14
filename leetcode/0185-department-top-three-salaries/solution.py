import pandas as pd

def top_three_salaries(employee: pd.DataFrame, department: pd.DataFrame) -> pd.DataFrame:
    employee['salary_rank'] = (
        employee
        .groupby('departmentId')['salary']
        .rank(method='dense', ascending=False)
    )
    
    df = (
        employee
        .loc[employee['salary_rank'] <= 3]
        .merge(department, left_on='departmentId', right_on='id', suffixes=['_employee', '_department'])
        .rename(columns={'name_department': 'Department', 'name_employee': 'Employee'})
        .iloc[:, [1,2,6]]
    )
    return df
