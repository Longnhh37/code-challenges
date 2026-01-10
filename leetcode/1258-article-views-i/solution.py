import pandas as pd

def article_views(views: pd.DataFrame) -> pd.DataFrame:
    result = views[
        (views['author_id'] == views['viewer_id'])
    ] \
        .rename(columns={'author_id': 'id'})

    return result[['id']].drop_duplicates().sort_values(by='id') 
