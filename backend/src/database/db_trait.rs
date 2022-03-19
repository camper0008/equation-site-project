use crate::models::{DbEquation, DbUser};

pub trait Database {
    fn add_user(user: DbUser) -> ();
    fn get_user_from_id(id: String) -> Option<DbUser>;
    fn add_equation(equation: DbEquation) -> ();
    fn get_equation_from_id(id: String) -> Option<DbEquation>;
}
