import pandas as pd

def immediate_food_delivery(delivery: pd.DataFrame) -> pd.DataFrame:
    df = delivery.loc[
        delivery.groupby('customer_id')['order_date'].idxmin()
    ]

    immediate_percentage = round(
            (df['order_date'] == df['customer_pref_delivery_date']).mean() * 100, 
        2)

    return pd.DataFrame({
        'immediate_percentage': [immediate_percentage]
    })
