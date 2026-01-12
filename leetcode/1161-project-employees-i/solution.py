import pandas as pd

def project_employees_i(project: pd.DataFrame, employee: pd.DataFrame) -> pd.DataFrame:
    df = (
        project
        .merge(employee, on='employee_id', how='left')
        .groupby('project_id', as_index=False)['experience_years']
        .mean()
        .rename(columns={'experience_years': 'average_years'})
        .round(2)
    )
    
    return df[['project_id', 'average_years']]
