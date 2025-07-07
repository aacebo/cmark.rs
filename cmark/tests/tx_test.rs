use std::{cell::RefCell, rc::Rc};

use cmark::{token::Token, tx::Tx};

#[test]
pub fn should_rollback() {
    let token = Rc::new(RefCell::new(Token::default()));
    let tx = Tx::new(token.clone());

    assert_eq!(token.borrow().kind, 0);
    token.borrow_mut().kind = 10;
    assert_eq!(token.borrow().kind, 10);
    tx.rollback();
    assert_eq!(token.borrow().kind, 0);
}
