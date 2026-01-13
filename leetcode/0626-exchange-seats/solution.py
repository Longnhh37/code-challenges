import pandas as pd

def exchange_seats(seat: pd.DataFrame) -> pd.DataFrame:
    seat['id'] = seat['id'].apply(lambda x : x-1 if x%2 == 0 else x if (x%2 == 1) & (x == len(seat)) else x+1)
    return seat.sort_values(by='id')

"""
def exchange_seats(seat: pd.DataFrame) -> pd.DataFrame:
    if seat.empty:
        return seat
    
    seat['id_swap'] = seat['id'] + (seat['id'] % 2) * 2 - 1

    max_id = seat['id'].max()
    if max_id % 2 == 0:
        max_id -= 1

    seat.loc[seat['id'].idxmax(), 'id_swap'] = max_id

    seat_swap = seat.copy()

    df = seat.merge(
        seat_swap,
        left_on='id',
        right_on='id_swap',
        suffixes=['old', '']
    )

    df = df[['id_swap', 'student']].rename(columns={'id_swap':'id'}).sort_values('id')

    return df
"""  
