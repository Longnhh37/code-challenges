import pandas as pd

def price_at_given_date(products: pd.DataFrame) -> pd.DataFrame:
    date = pd.Timestamp("2019-08-16")
    filtered_price = (
        products
        .query('change_date <= @date')
    )

    filtered_price = filtered_price.loc[
        filtered_price.groupby('product_id')['change_date'].idxmax()
    ]

    list_product = products[['product_id']].drop_duplicates()

    result = list_product.merge(
        filtered_price,
        on='product_id',
        how='left'
    ).fillna(10)

    result = result[['product_id', 'new_price']].rename(columns={'new_price': 'price'})

    return result
