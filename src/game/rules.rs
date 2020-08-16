use super::board::Board;
use super::r#move::Move;
use core::fmt::Debug;

#[derive(PartialEq)]
pub enum RuleType {
    CONDITION,
    CONSEQUENCE
}

pub trait Rule {
    fn valid(&self, b: &Board, m: &Move) -> bool;
    fn r#type(&self) -> RuleType;
}

impl Debug for dyn Rule {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Rule")
    }
}

pub struct BaseRule {}

impl Rule for BaseRule {
    fn valid(&self, b: &Board, m: &Move) -> bool {
        !b.is_occupied(m)
    }

    fn r#type(&self) -> RuleType { RuleType::CONDITION }
}