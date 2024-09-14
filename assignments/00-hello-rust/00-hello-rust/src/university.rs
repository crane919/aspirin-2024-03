// use core::num;

#[derive(PartialEq, Clone, Copy, Debug)]
enum ClassYear {
    Senior,
    Junior,
    Sophomore,
    FirstYear,
}

struct Student {
    name: &'static str,
    class_year: ClassYear,
    gpa: f32,
}

const OLIN_STUDENTS: [Student; 8] = [
    Student {
        name: "Alice",
        class_year: ClassYear::Senior,
        gpa: 3.9,
    },
    Student {
        name: "Foo",
        class_year: ClassYear::Sophomore,
        gpa: 2.3,
    },
    Student {
        name: "Bar",
        class_year: ClassYear::Junior,
        gpa: 3.9,
    },
    Student {
        name: "Ralph",
        class_year: ClassYear::Senior,
        gpa: 3.1,
    },
    Student {
        name: "Ayush",
        class_year: ClassYear::Senior,
        gpa: 0.0,
    },
    Student {
        name: "Anna",
        class_year: ClassYear::FirstYear,
        gpa: 4.0,
    },
    Student {
        name: "Hannah",
        class_year: ClassYear::FirstYear,
        gpa: 4.0,
    },
    Student {
        name: "Lorin",
        class_year: ClassYear::Junior,
        gpa: 3.6,
    },
];

/// Calculate average gpa of students. First years don't count.
fn get_average_gpa() -> f32 {
    let mut total_gpa: f32 = 0.0;
    let mut num_students: f32 = 0.0;
    for student in OLIN_STUDENTS.iter() {
        if student.class_year != ClassYear::FirstYear {
            total_gpa += student.gpa;
            num_students += 1.0;
        }
    }
    total_gpa / num_students
}

/// Gets the number of students in selected class that have a gpa above the average
fn get_num_excel_students_for_class(class_year: ClassYear) -> u32 {
    let avg_gpa = get_average_gpa();
    let mut num_students: u32 = 0;
    for student in OLIN_STUDENTS.iter() {
        if student.gpa > avg_gpa && student.class_year == class_year {
            num_students += 1;
        }
    }
    num_students
}

/// Gets the class with the most excelling students
fn get_best_class() -> ClassYear {
    // Calculate the number of best students
    let senior_students = get_num_excel_students_for_class(ClassYear::Senior);
    let junior_students = get_num_excel_students_for_class(ClassYear::Junior);
    let sophmore_students = get_num_excel_students_for_class(ClassYear::Sophomore);

    // Compares the number of best students
    if sophmore_students > junior_students && sophmore_students > senior_students {
        ClassYear::Sophomore
    } else if junior_students > senior_students {
        ClassYear::Junior
    } else {
        ClassYear::Senior
    }
}

// Do not modify below here
#[cfg(test)]
mod tests {
    use float_cmp::approx_eq;

    use crate::university::{
        get_average_gpa, get_best_class, get_num_excel_students_for_class, ClassYear,
    };

    #[test]
    fn test_get_average_gpa() {
        assert!(approx_eq!(f32, get_average_gpa(), 2.8))
    }

    #[test]
    fn test_get_num_excel_students_for_class() {
        assert_eq!(get_num_excel_students_for_class(ClassYear::Sophomore), 0);
        assert_eq!(get_num_excel_students_for_class(ClassYear::Junior), 2);
        assert_eq!(get_num_excel_students_for_class(ClassYear::Senior), 2);
    }

    #[test]
    fn test_get_best_class() {
        assert_eq!(get_best_class(), ClassYear::Senior);
    }
}
