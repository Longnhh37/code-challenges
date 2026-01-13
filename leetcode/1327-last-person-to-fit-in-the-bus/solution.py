import pandas as pd

def last_passenger(queue: pd.DataFrame) -> pd.DataFrame:
    # Manipulation
    queue = queue.sort_values('turn')
    queue['cum_weight'] = queue['weight'].cumsum()
    queue = queue.query('cum_weight <= 1000')

    # Filter
    result = queue[['person_name']].tail(1)

    return result
