import pandas as pd

def monthly_transactions(transactions: pd.DataFrame) -> pd.DataFrame:
    transactions['month'] = transactions['trans_date'].dt.strftime('%Y-%m')
    result = (
        transactions
        .assign(
            approved_flag=(transactions['state'] == 'approved').astype(int),
            approved_amount=transactions['amount'].where(transactions['state'] == 'approved', 0)
        )
        .groupby(['month', 'country'], as_index=False, dropna=False).agg(
            trans_count=('state', 'size'),
            approved_count=('approved_flag', 'sum'),
            trans_total_amount=('amount', 'sum'),
            approved_total_amount=('approved_amount','sum')
        )
    )

    return result
