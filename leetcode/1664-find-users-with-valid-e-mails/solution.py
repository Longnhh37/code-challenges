import pandas as pd

def valid_emails(users: pd.DataFrame) -> pd.DataFrame:
    df = users[
        users['mail'].str.contains(r'^[a-zA-Z]+[a-zA-Z0-9_.-]*@leetcode\.com$', regex=True)
    ]

    return df
