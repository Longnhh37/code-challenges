import pandas as pd

def categorize_products(activities: pd.DataFrame) -> pd.DataFrame:
    activities = activities.drop_duplicates().sort_values(['sell_date', 'product'])

    num_sold = activities.groupby('sell_date', as_index=False)\
        .size().rename(columns={'size': 'num_sold'})

    product_list = activities.groupby('sell_date', as_index=False)['product'] \
        .agg(','.join).rename(columns={'product': 'products'})   

    result = num_sold.merge(product_list)

    return result
