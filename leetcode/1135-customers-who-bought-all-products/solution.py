import pandas as pd

def find_customers(customer: pd.DataFrame, product: pd.DataFrame) -> pd.DataFrame:
    num_product = list(product.nunique())

    df = (
        customer
        .drop_duplicates()
        .groupby('customer_id', as_index=False)['product_key']
        .size()
        .query('size == @num_product')
    )

    return df[['customer_id']]

