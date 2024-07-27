mod department;
use std::collections::{hash_map::Values, HashMap};
pub use department::Department;

struct Company {
    name: String,
    departments: HashMap<String, Department>,
}

struct EmployeeIterator {
    departments_iter: Values<String, Department>,
}

impl Company {
    fn new(name: String) -> Company {
        Company {
            name,
            departments: HashMap::new()
        }
    }

    fn new_with_capacity(name: String, departments_capacity: usize) -> Company {
        Company {
            name,
            departments: HashMap::with_capacity(departments_capacity)
        }
    }

    fn get_name(&self) -> &String {
        &self.name
    }

    fn add_department(&mut self, department: Department) -> bool {
        
        let department_name = department.get_name().clone();
        if self.departments.contains_key(&department_name) {
            return false;
        }
        
        self.departments.insert(department_name, department);
        true
    }

    fn get_departments(&self) -> impl Iterator<Item = &Department> {
        return self.departments.values();
    }

    fn get_employees_count(&self) -> i32 {
        let mut employees_count = 0;
        for department in self.get_departments() {
            employees_count += department.get_employees_count();
        }

        return employees_count;
    }

    fn get_employees(&self) -> impl Iterator<Item = &String> {
        
    }
}