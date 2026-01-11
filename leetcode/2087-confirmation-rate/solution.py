import pandas as pd

def confirmation_rate(signups: pd.DataFrame, confirmations: pd.DataFrame) -> pd.DataFrame:
    rate = (
        confirmations
        .assign(is_confirmed=confirmations['action'] == 'confirmed')
        .groupby('user_id', as_index=False)['is_confirmed']
        .mean()
        .round(2)
        .rename(columns={'is_confirmed': 'confirmation_rate'})
    )

    result = signups[['user_id']].merge(
        rate, 
        on='user_id',
        how='left'
    ).fillna(0)

    return result

