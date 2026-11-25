//! Conservative audit of the repository's ghost-only configuration boundary.
use proc_macro2::{Delimiter, TokenStream, TokenTree};

const GUARD: &str = "anchorbook_verification";
fn ident(token: Option<&TokenTree>, expected: &str) -> bool {
    matches!(token, Some(TokenTree::Ident(name)) if name == expected)
}
fn ghost_functions(tokens: TokenStream) -> bool {
    let items: Vec<_> = tokens.into_iter().collect();
    let mut index = 0;
    while index < items.len() {
        if ident(items.get(index), "pub") {
            index += 1;
            if matches!(items.get(index), Some(TokenTree::Group(g)) if g.delimiter() == Delimiter::Parenthesis)
            {
                index += 1;
            }
        }
        if ident(items.get(index), "open") || ident(items.get(index), "closed") {
            index += 1;
        }
        if !ident(items.get(index), "spec") || !ident(items.get(index + 1), "fn") {
            return false;
        }
        index += 2;
        // A spec function's body is ghost material; Rust/Verus checks its syntax.
        while index < items.len()
            && !matches!(&items[index], TokenTree::Group(g) if g.delimiter() == Delimiter::Brace)
        {
            index += 1;
        }
        if index == items.len() {
            return false;
        }
        index += 1;
    }
    true
}

pub fn ghost_only(source: &str) -> bool {
    fn walk(stream: TokenStream) -> bool {
        let tokens: Vec<_> = stream.into_iter().collect();
        let mut index = 0;
        while index < tokens.len() {
            if matches!(&tokens[index], TokenTree::Punct(p) if p.as_char() == '#') {
                let attribute_index = index
                    + 1
                    + usize::from(
                        matches!(tokens.get(index + 1), Some(TokenTree::Punct(p)) if p.as_char() == '!'),
                    );
                if let Some(TokenTree::Group(attribute)) = tokens.get(attribute_index) {
                    let attr: Vec<_> = attribute.stream().into_iter().collect();
                    if let Some(TokenTree::Group(arguments)) = attr.get(1) {
                        let args: Vec<_> = arguments.stream().into_iter().collect();
                        if ident(args.first(), GUARD) && ident(attr.first(), "cfg_attr") {
                            let permitted = ident(args.get(2), "verus_spec")
                                || ident(args.get(2), "verus_verify")
                                || (ident(args.get(2), "feature")
                                    && args
                                        .get(3)
                                        .is_some_and(|t| t.to_string() == "(proc_macro_hygiene)"));
                            if !permitted || args.len() > 4 {
                                return false;
                            }
                            index = attribute_index + 1;
                            continue;
                        }
                        if ident(args.first(), GUARD)
                            && args.len() == 1
                            && ident(attr.first(), "cfg")
                        {
                            let next = attribute_index + 1;
                            if ident(tokens.get(next), "use") {
                                let end = tokens[next..]
                                    .iter()
                                    .position(
                                        |t| matches!(t, TokenTree::Punct(p) if p.as_char() == ';'),
                                    )
                                    .map(|n| next + n);
                                let Some(end) = end else {
                                    return false;
                                };
                                let import: TokenStream =
                                    tokens[next..=end].iter().cloned().collect();
                                if import.to_string().replace(' ', "") != "usevstd::prelude::*;" {
                                    return false;
                                }
                                index = end + 1;
                                continue;
                            }
                            let is_spec = ident(tokens.get(next), "verus");
                            if !is_spec && !ident(tokens.get(next), "proof") {
                                return false;
                            }
                            if !matches!(tokens.get(next + 1), Some(TokenTree::Punct(p)) if p.as_char() == '!')
                            {
                                return false;
                            }
                            let Some(TokenTree::Group(body)) = tokens.get(next + 2) else {
                                return false;
                            };
                            if body.delimiter() != Delimiter::Brace
                                || (is_spec && !ghost_functions(body.stream()))
                            {
                                return false;
                            }
                            index = next + 3;
                            continue;
                        }
                    }
                }
            }
            match &tokens[index] {
                TokenTree::Ident(name)
                    if [
                        GUARD,
                        "verus_only",
                        "verus_keep_ghost",
                        "verus_keep_ghost_body",
                    ]
                    .iter()
                    .any(|guard| name == guard) =>
                {
                    return false;
                }
                TokenTree::Group(group) if !walk(group.stream()) => return false,
                _ => {}
            }
            index += 1;
        }
        true
    }
    source.parse().is_ok_and(walk)
}
