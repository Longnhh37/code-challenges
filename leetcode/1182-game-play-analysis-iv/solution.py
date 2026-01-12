import pandas as pd

def gameplay_analysis(activity: pd.DataFrame) -> pd.DataFrame:
    total_player = activity['player_id'].nunique()

    first_login = activity.loc[
        activity.groupby('player_id')['event_date'].idxmin()
    ][['player_id', 'event_date']]

    df = activity.merge(
        first_login,
        on='player_id',
        how='inner',
        suffixes=('', '_first')
    )

    logged_in_again = (
        (df['event_date'] == df['event_date_first'] + pd.Timedelta(days=1))
    ).sum()

    fraction = round(logged_in_again / total_player, 2)

    return pd.DataFrame({
        'fraction': [fraction]
    })
