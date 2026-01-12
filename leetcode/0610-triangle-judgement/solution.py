import pandas as pd

def triangle_judgement(triangle: pd.DataFrame) -> pd.DataFrame:
    triangle['check1'] = + triangle['x'] + triangle['y'] - triangle['z']
    triangle['check2'] = + triangle['x'] - triangle['y'] + triangle['z']
    triangle['check3'] = - triangle['x'] + triangle['y'] + triangle['z']
    triangle['triangle'] = (triangle['check1'] > 0) & (triangle['check2'] > 0) & (triangle['check3'] > 0)

    triangle['triangle'] = triangle['triangle'].map({True: 'Yes', False: 'No'})

    return triangle[['x', 'y', 'z', 'triangle']]



