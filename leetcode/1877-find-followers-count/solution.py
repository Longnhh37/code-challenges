import pandas as pd

def count_followers(followers: pd.DataFrame) -> pd.DataFrame:
    result = (
        followers
        .groupby('user_id', as_index=False)['follower_id'].size()
        .rename(columns={'size': 'followers_count'})
        )

    return result


