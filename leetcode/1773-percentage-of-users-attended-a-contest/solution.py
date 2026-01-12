import pandas as pd

def users_percentage(users: pd.DataFrame, register: pd.DataFrame) -> pd.DataFrame:
    df = (
        register
        .groupby('contest_id', as_index=False)
        .size()
    )

    df['percentage'] = (df['size'] / len(users) * 100).round(2)

    df = df.sort_values(['percentage', 'contest_id'], ascending=[False, True])

    return df[['contest_id', 'percentage']]
