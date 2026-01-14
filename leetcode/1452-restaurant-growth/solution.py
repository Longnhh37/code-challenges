import pandas as pd

def restaurant_growth(customer: pd.DataFrame) -> pd.DataFrame:
    customer = (
        customer
        .groupby('visited_on', as_index=False)['amount'].sum()
        .sort_values('visited_on')
    )

    customer['total_amount'] = (
        customer
        .rolling('7D', on='visited_on')
        ['amount'].sum()
    )
    
    customer['average_amount'] = (
        customer
        .rolling('7D', on='visited_on')
        ['amount'].mean().round(2)
    )

    start_date = customer['visited_on'].min() + pd.Timedelta(days=6)

    df = customer[customer['visited_on'] >= start_date] \
        [['visited_on', 'total_amount', 'average_amount']] \
        .rename(columns={'total_amount': 'amount'})
    return df


