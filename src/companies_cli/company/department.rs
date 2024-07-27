pub struct Department {
    name: String,
    employees: Vec<String>,
    employees_sorted: bool
}

impl Department {
    pub fn new(name: String) -> Department {
        Department {
            name,
            employees: Vec::new(),
            employees_sorted: true,
        }
    }

    pub fn new_with_capacity(name: String, employees_capacity: usize) -> Department {
        Department {
            name,
            employees: Vec::with_capacity(employees_capacity),
            employees_sorted: true,
        }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn add_employee(&mut self, employee: String) {
        self.employees_sorted = self.employees.last().map_or(true, |last| last < &employee);
        self.employees.push(employee);
    }

    pub fn sort_employees(&mut self) {
        if self.employees_sorted {
            return;
        }

        self.employees.sort();
        self.employees_sorted = true;
    }

    pub fn get_employees(&mut self) -> &Vec<String> {
        if !self.employees_sorted {
            self.employees.sort();
            self.employees_sorted = true;
        }

        &self.employees
    }

    pub fn get_employees_count(&self) -> i32 {
        self.employees.len() as i32
    }

    pub fn get_employees_sorted(&mut self) -> &Vec<String> {
        self.sort_employees();
        self.get_employees()
    }
}