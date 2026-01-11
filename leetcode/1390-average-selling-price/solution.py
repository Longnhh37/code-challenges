import pandas as pd

def average_selling_price(prices: pd.DataFrame, units_sold: pd.DataFrame) -> pd.DataFrame:
    
    # 1. Sort DataFrame for merge_asof
    prices.sort_values('start_date', inplace=True)
    units_sold.sort_values('purchase_date', inplace=True)

    #2. Merge units_sold with latest price before purchase_date
    if not units_sold.empty:
        df = pd.merge_asof(
            units_sold, prices,
            by='product_id',
            left_on='purchase_date',
            right_on='start_date'
        )
        #3. Keep purchase between [start_date, end_date]
        df = df[df['purchase_date'] <= df['end_date']].copy()
    else:
        # If units_sold has 0 record -> return blank DataFrame
        df = pd.DataFrame(columns=['product_id', 'units', \
            'price', 'purchase_date', 'start_date', 'end_date'])

    #4. Revenue
    df['units'] = df['units'].fillna(0)
    df['revenue'] = df['price'].fillna(0) * df['units']

    #5. Aggregate
    result = df.groupby('product_id', as_index=False).agg(
        total_revenue=('revenue', 'sum'),
        total_units=('units', 'sum')
    )

    #6. average_price, avoid dividing by 0 if units_sold is blank
    result['average_price'] = (
        (result['total_revenue'] / result['total_units'])
        .fillna(0)
        .round(2)
    )

    #7. Return all product_id, even if units_sold is blank
    all_products = pd.DataFrame({'product_id': prices['product_id'].unique()})
    result = all_products.merge(
        result[['product_id', 'average_price']],
        on='product_id',
        how='left'
    )
    result['average_price'] = result['average_price'].fillna(0)

    #8 Return
    return result
