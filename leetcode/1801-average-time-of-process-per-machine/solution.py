import pandas as pd

def get_average_time(activity: pd.DataFrame) -> pd.DataFrame:
    
    start_df = activity[activity['activity_type'] == 'start']
    end_df = activity[activity['activity_type'] == 'end']

    merged = pd.merge(
        start_df,
        end_df,
        on=(['machine_id', 'process_id']),
        how='inner',
        suffixes = ('_start', '_end') 
    )

    merged = merged.assign(
        time_running = merged['timestamp_end'] - merged['timestamp_start']
    )

    result = (
        merged
        .groupby('machine_id', as_index=False)
        .agg(processing_time=('time_running', 'mean'))
    )

    result = result.round(3)
    
    return result
