import pandas as pd

def most_friends(request_accepted: pd.DataFrame) -> pd.DataFrame:
    if request_accepted.empty:
        return pd.DataFrame(columns=['id', 'num'])
    
    df = pd.concat([
        request_accepted[['requester_id']].rename(columns={'requester_id': 'id'}),
        request_accepted[['accepter_id']].rename(columns={'accepter_id': 'id'})
    ], ignore_index=True)
    df = (df.groupby('id', as_index=False).size().rename(columns={'size': 'num'}))
    
    return df.loc[[df['num'].idxmax()]]

