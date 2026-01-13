import pandas as pd

def count_salary_categories(accounts: pd.DataFrame) -> pd.DataFrame:
    range1 = 20000
    range2 = 50000

    accounts['category'] = np.select(
        [
            (accounts['income'] >= 0) & (accounts['income'] < range1),
            (accounts['income'] >= range1) & (accounts['income'] <= range2),
            (accounts['income'] > range2),
        ],
        ['Low Salary', 'Average Salary', 'High Salary'],
        default=np.nan
    )

    category_count = (
        accounts
        .groupby('category', as_index=False)['category'].size()
    )

    category = pd.DataFrame({'category': ['Low Salary', 'Average Salary', 'High Salary']})
    result = category.merge(category_count, on='category', how='left')
    result = result.fillna(0).rename(columns={'size': 'accounts_count'})

    return result
