use std::collections::{BTreeSet, HashMap};

#[derive(Default)]
pub struct School {
    grades: [BTreeSet<String>; 12],
    student_to_grade: HashMap<String, usize>,
}

impl School {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, grade: usize, student: &str) {
        if self.student_to_grade.contains_key(student) {
            return;
        }
        
        if let Some(set) = self.grades.get_mut(grade - 1) {
            set.insert(student.into());
            self.student_to_grade.insert(student.into(), grade);
        }
    }

    pub fn grade(&self, grade: usize) -> Vec<String> {
        self.grades
            .get(grade - 1)
            .map(|set| set.iter().cloned().collect())
            .unwrap_or_default()
    }

     pub fn grades(&self) -> Vec<u32> {
        self.grades
            .iter()
            .enumerate()
            .filter(|(_, set)| !set.is_empty())
            .map(|(i, _)| (i + 1) as u32)
            .collect()
    }
}