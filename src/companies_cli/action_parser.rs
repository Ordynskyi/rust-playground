enum Action {
    AddCompany(String),
    AddEmployee()
}

struct AddEmployeeCommand {
    company: String,
    department: String
}

fn parse_action(action: &str) {

}