import pandas as pd

def sales_analysis(sales: pd.DataFrame) -> pd.DataFrame:
    df = sales.loc[
        sales.groupby('product_id')['year'].idxmin()
        ][['product_id', 'year']]

    result = sales.merge(
        df, 
        on=['product_id', 'year'],
    ).drop(columns='sale_id').rename(columns={'year': 'first_year'})

    return result


