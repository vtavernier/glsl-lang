//! First stage lexer declaration

use std::{iter::Peekable, str::CharIndices};

use lang_util::{TextRange, TextSize};

use crate::util::LineMap;

/// Type of token for line splitting
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(clippy::upper_case_acronyms)]
#[repr(u16)]
pub enum NewlineTokenKind {
    LETTER,
    DIGIT,
    PUNCT,
    NEWLINE,
    WS,
}

/// First stage token with location
pub type NewlineToken = crate::util::TextToken<NewlineTokenKind>;

/// Basic lexer to split input lines according to the GLSL spec
///
/// This only detects \r\n sequences and classifies other characters following the types declared
/// in [NewlineTokenKind](NewlineTokenKind).
#[derive(Debug, Clone)]
pub struct NewlineSplitter<'i> {
    end: TextSize,
    chars: Peekable<CharIndices<'i>>,
    line_map: LineMap,
}

impl<'i> NewlineSplitter<'i> {
    pub fn new(input: &'i str) -> Self {
        Self {
            end: TextSize::of(input),
            chars: input.char_indices().peekable(),
            line_map: LineMap::new(),
        }
    }

    pub fn line_map(&self) -> &LineMap {
        &self.line_map
    }

    pub fn into_line_map(self) -> LineMap {
        self.line_map
    }

    fn current_pos(&mut self, start_pos: usize) -> TextRange {
        TextRange::new(
            TextSize::from(start_pos as u32),
            self.chars
                .peek()
                .map(|(pos, _)| TextSize::from(*pos as u32))
                .unwrap_or(self.end),
        )
    }
}

impl<'i> Iterator for NewlineSplitter<'i> {
    type Item = NewlineToken;

    fn next(&mut self) -> Option<Self::Item> {
        use NewlineTokenKind::*;

        let c = self.chars.next();

        // GLSL spec: Lines are relevant for compiler diagnostic messages and the
        // preprocessor.  They are terminated by carriage-return or line-feed. If both
        // are used together, it will count as only a single line termination.

        match c {
            Some((pos, ch)) if ch == '\r' || ch == '\n' => {
                // Advance to next char if it's also part of the newline
                let range = if let Some((next_pos, next_ch)) = self.chars.peek() {
                    // End boundary of the newline token
                    let end = if (*next_ch == '\r' || *next_ch == '\n') && *next_ch != ch {
                        self.chars.next();

                        // Peek to get the next char boundary
                        self.chars
                            .peek()
                            .map(|(pos, _)| TextSize::from(*pos as u32))
                            .unwrap_or(self.end)
                    } else {
                        TextSize::from(*next_pos as u32)
                    };

                    TextRange::new(TextSize::from(pos as u32), end)
                } else {
                    // No more characters
                    TextRange::new(TextSize::from(pos as u32), self.end)
                };

                self.line_map.add_line(range.end().into());
                Some(NewlineToken::new(NEWLINE, range))
            }
            Some((pos, ch)) if ch.is_ascii_alphabetic() => {
                Some(NewlineToken::new(LETTER, self.current_pos(pos)))
            }
            Some((pos, ch)) if ch.is_ascii_digit() => {
                Some(NewlineToken::new(DIGIT, self.current_pos(pos)))
            }
            Some((pos, ch)) if ch.is_ascii_whitespace() => {
                // \n and \r have been already matched
                Some(NewlineToken::new(WS, self.current_pos(pos)))
            }
            Some((pos, _)) => Some(NewlineToken::new(PUNCT, self.current_pos(pos))),
            None => None,
        }
    }
}
