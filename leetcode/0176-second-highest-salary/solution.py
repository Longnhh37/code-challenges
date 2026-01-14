import pandas as pd

def second_highest_salary(employee: pd.DataFrame) -> pd.DataFrame:
    unique_salaries = employee['salary'].drop_duplicates().sort_values(ascending=False).reset_index(drop=True)
    second_salary = unique_salaries[1] if len(unique_salaries) > 1 else None

    return pd.DataFrame({'SecondHighestSalary': [second_salary]})

