use std::collections::BTreeSet;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SchoolInputError {
    #[error("Grade should be between 1 and 12")]
    OutOfBound,

    #[error("This student is already added to this roster")]
    AlreadyAdded,
}

#[derive(Default)]
pub struct School {
    grades: [BTreeSet<String>; 12],
}

impl School {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, grade: usize, student: &str) -> Result<(), SchoolInputError> {
        let set = self.get_grade_mut(grade)?;

        if !set.insert(student.into()) {
            return Err(SchoolInputError::AlreadyAdded);
        }

        Ok(())
    }

    pub fn grades(&self) -> Vec<u32> {
        self.grades
            .iter()
            .enumerate()
            .filter(|(_, set)| !set.is_empty())
            .map(|(i, _)| (i + 1) as u32)
            .collect()
    }

    pub fn grade(&self, grade: usize) -> Result<Vec<String>, SchoolInputError> {
        let set = self.get_grade(grade)?;

        Ok(set.iter().cloned().collect())
    }

    // helpers
    fn get_grade(&self, grade: usize) -> Result<&BTreeSet<String>, SchoolInputError> {
        self.grades
            .get(grade - 1)
            .ok_or(SchoolInputError::OutOfBound)
    }

    fn get_grade_mut(&mut self, grade: usize) -> Result<&mut BTreeSet<String>, SchoolInputError> {
        self.grades
            .get_mut(grade - 1)
            .ok_or(SchoolInputError::OutOfBound)
    }
}

fn main() {}
