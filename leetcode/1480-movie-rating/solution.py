import pandas as pd

def movie_rating(movies: pd.DataFrame, users: pd.DataFrame, movie_rating: pd.DataFrame) -> pd.DataFrame:
    most_review = (
        movie_rating
        .merge(users)
        .groupby('name', as_index=False).size()
        .sort_values(['size', 'name'], ascending=[False, True])
        .head(1)
    )

    highest_rating = movie_rating.merge(movies)
    highest_rating = highest_rating[highest_rating['created_at'].dt.to_period('M') == '2020-02']
    highest_rating = (
        highest_rating
        .groupby('title', as_index=False)['rating'].mean()
        .sort_values(['rating', 'title'], ascending=[False, True])
        .head(1)
    )
    
    df = pd.concat([
        most_review['name'], 
        highest_rating['title']
    ], ignore_index=True).to_frame('results')
    return df
