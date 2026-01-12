import pandas as pd

def consecutive_numbers(logs: pd.DataFrame) -> pd.DataFrame:
    logs = logs.sort_values('id')
    
    logs['prev_1'] = (logs['num'].shift(1))

    logs['prev_2'] = (logs['num'].shift(2))

    df = logs[
        (logs['num'] == logs['prev_1']) & (logs['num'] == logs['prev_2'])
    ][['num']].drop_duplicates().rename(columns={'num': 'ConsecutiveNums'})
    
    return df
