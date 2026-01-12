import pandas as pd

def find_classes(courses: pd.DataFrame) -> pd.DataFrame:
    result = courses.groupby('class', as_index=False).size()
    result = result[result['size'] >= 5]
    return result[['class']]

