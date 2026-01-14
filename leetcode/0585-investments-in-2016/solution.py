import pandas as pd

def find_investments(insurance: pd.DataFrame) -> pd.DataFrame:
    insurance = insurance.copy()

    insurance['tiv_cnt'] = insurance.groupby('tiv_2015')['tiv_2015'].transform('count')
    insurance['loc_cnt'] = insurance.groupby(['lat', 'lon'])['lat'].transform('count')

    total = insurance.loc[
        (insurance['tiv_cnt'] > 1) & (insurance['loc_cnt'] == 1),
        'tiv_2016'
    ].sum()

    return pd.DataFrame({'tiv_2016': [round(total, 2)]})

