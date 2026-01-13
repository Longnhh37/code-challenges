import pandas as pd

def find_employees(employees: pd.DataFrame) -> pd.DataFrame:
    pd = employees[~employees['manager_id'].isin(employees['employee_id'])]
    pd = pd[pd['manager_id'].notna()]
    pd = pd[pd['salary'] < 30000][['employee_id']].sort_values('employee_id')

    return pd
