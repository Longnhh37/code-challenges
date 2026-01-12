import pandas as pd

def count_employees(employees: pd.DataFrame) -> pd.DataFrame:
    manager = (
        employees
        .groupby('reports_to', as_index=False)
        .agg(
            reports_count=('employee_id', 'count'),
            average_age=('age', 'mean')
        )
    )
    manager['average_age'] = (manager['average_age'] + 1e-9).round(0)

    df = employees.merge(
        manager,
        left_on='employee_id',
        right_on='reports_to'
    ).sort_values('employee_id')

    return df[['employee_id', 'name', 'reports_count', 'average_age']]

