use ruff_text_size::TextRange;

use crate::lexer::{Lexer, LexerContext, LexicalError, TokenValue};
use crate::{Tok, TokenKind};

#[derive(Debug)]
pub(crate) struct TokenSource<'src> {
    lexer: Lexer<'src>,
    tokens: Vec<(TokenKind, TextRange)>,
    errors: Vec<LexicalError>,
}

impl<'src> TokenSource<'src> {
    pub(crate) fn new(lexer: Lexer<'src>) -> Self {
        Self {
            lexer,
            tokens: vec![],
            errors: vec![],
        }
    }

    /// Returns the kind of the current token.
    pub(crate) fn current_kind(&self) -> TokenKind {
        self.lexer.current_kind()
    }

    /// Returns the range of the current token.
    pub(crate) fn current_range(&self) -> TextRange {
        self.lexer.current_range()
    }

    pub(crate) fn take_value(&mut self) -> TokenValue {
        self.lexer.take_value()
    }

    /// Returns the next token kind and its range without consuming it.
    pub(crate) fn peek(&self) -> Option<(TokenKind, TextRange)> {
        let checkpoint = self.lexer.checkpoint();
        let next = loop {
            let next = self.lexer.next_token_with_context(LexerContext::Peeking);
            match next.as_ref() {
                Ok((Tok::EndOfFile, _)) => return None,
                Ok((token, _)) if is_trivia(token) => continue,
                result => {
                    break (match result {
                        Ok((token, range)) => (TokenKind::from_token(&token), *range),
                        Err(error) => (TokenKind::Unknown, error.location()),
                    })
                }
            }
        };
        self.lexer.rewind(checkpoint);
        Some(next)
    }

    #[inline]
    pub(crate) fn next_token(&mut self) -> Option<(TokenKind, TextRange)> {
        loop {
            let next = self.lexer.next()?;

            match next {
                Ok((token, range)) => {
                    let kind = TokenKind::from_token(&token);
                    self.tokens.push((kind, range));
                    if is_trivia(&token) {
                        continue;
                    }
                    break Some((kind, range));
                }
                Err(error) => {
                    let location = error.location();
                    self.errors.push(error);
                    break Some((TokenKind::Unknown, location));
                }
            }
        }
    }

    pub(crate) fn finish(self) -> (Vec<(TokenKind, TextRange)>, Vec<LexicalError>) {
        assert!(self.peek().is_none(), "TokenSource was not fully consumed");

        (self.tokens, self.errors)
    }
}

const fn is_trivia(token: &Tok) -> bool {
    matches!(token, Tok::Comment(_) | Tok::NonLogicalNewline)
}
