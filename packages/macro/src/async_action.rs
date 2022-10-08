use crate::action_parser::{ActionParser, ActionToken, ActionType};
use syn::ItemImpl;

pub struct AsyncActionParser {
    action_parser: ActionParser,
}

impl AsyncActionParser {
    pub fn new(action_creator_impl: ItemImpl) -> Self {
        Self {
            action_parser: ActionParser::new(ActionType::AsyncAction, action_creator_impl),
        }
    }

    pub fn parse(&self) -> ActionToken {
        self.action_parser.parse()
    }
}
