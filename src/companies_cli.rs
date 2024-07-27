mod action_parser;
mod company;

pub fn invoke_action(action: &str) {
    self::action_parser::parse_action(action);
}