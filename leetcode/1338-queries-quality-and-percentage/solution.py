import pandas as pd
from decimal import Decimal, ROUND_HALF_UP

def sql_round(x, n=2):
    if pd.isna(x):
        return 0.0
    return float(
        Decimal(str(x)).quantize(
            Decimal('1.' + '0'*n),
            rounding=ROUND_HALF_UP
        )
    )

def queries_stats(queries: pd.DataFrame) -> pd.DataFrame:
    # 1. Poor query (ignore NULL rating)
    queries['poor_query'] = queries['rating'].lt(3).where(queries['rating'].notna())

    # 2. Quality
    queries['quality'] = queries['rating'] / queries['position']

    # 3. Aggregate
    result = (
        queries
        .groupby('query_name', as_index=False)
        .agg(
            quality=('quality', 'mean'),
            poor_query_percentage=('poor_query', 'mean')
        )
    )

    # 4. SQL-style rounding
    result['quality'] = result['quality'].apply(sql_round)
    result['poor_query_percentage'] = result['poor_query_percentage'].apply(
        lambda x: sql_round(x * 100)
    )

    return result[['query_name', 'quality', 'poor_query_percentage']]

