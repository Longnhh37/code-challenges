import pandas as pd

def find_managers(employee: pd.DataFrame) -> pd.DataFrame:


    count_employee = (
        employee[['managerId']]
        .groupby('managerId')
        .filter(lambda x: len(x) >= 5)
    )

    return employee[employee['id'].isin(count_employee['managerId'])][['name']]
