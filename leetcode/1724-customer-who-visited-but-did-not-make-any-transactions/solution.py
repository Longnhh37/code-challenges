import pandas as pd

def find_customers(visits: pd.DataFrame, transactions: pd.DataFrame) -> pd.DataFrame:
    df = visits[~visits['visit_id'].isin(transactions['visit_id'])]
    
    result = (
        df.groupby('customer_id')
        .size()
        .reset_index(name='count_no_trans')
    )

    return result
