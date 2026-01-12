import pandas as pd

def biggest_single_number(my_numbers: pd.DataFrame) -> pd.DataFrame:
    single_count = (
        my_numbers
        .groupby('num', as_index=False).size()
        .query('size == 1')[['num']]
    )

    result = (
        single_count
        .sort_values('num', ascending=False)
        .head(1)
    )
    if result.empty:
        result = pd.DataFrame({'num': [None]})

    return result


