import pandas as pd

def list_products(products: pd.DataFrame, orders: pd.DataFrame) -> pd.DataFrame:
    df = (
        orders
        .loc[orders['order_date'].dt.to_period('M') == '2020-02']
        .groupby('product_id', as_index=False)['unit'].sum()
        .query('unit >= 100')
        .merge(products)[['product_name', 'unit']]
    )

    return df
