import pandas as pd

def students_and_examinations(students: pd.DataFrame, subjects: pd.DataFrame, examinations: pd.DataFrame) -> pd.DataFrame:
    examinations_count = (
        examinations
        .groupby(['student_id', 'subject_name'], as_index=False)
        .size()
        .rename(columns={'size': 'attended_exams'})
    )

    merged = pd.merge(
        students,
        subjects,
        how='cross'
    )

    result = pd.merge(
        merged,
        examinations_count,
        on=['student_id', 'subject_name'],
        how='left'
    )
    
    result = (
        result
        .fillna({'attended_exams': 0})
        .sort_values(by=['student_id', 'subject_name'])
    )

    return result

