import pandas as pd


def user_activity(activity: pd.DataFrame) -> pd.DataFrame:
    end_date = pd.Timestamp('2019-07-27')
    start_date = end_date - pd.Timedelta(days=29)

    df = (
        activity
        .loc[activity['activity_date'].between(start_date, end_date)]
        .groupby('activity_date', as_index=False)['user_id']
        .nunique()
        .rename(columns={'activity_date': 'day', 'user_id': 'active_users'})
    )

    return df
