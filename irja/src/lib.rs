use proc_macro::Ident;
use proc_macro::{TokenStream, TokenTree};
use quote::quote;

static KEYWORDS: phf::Map<&'static str, &'static str> = phf::phf_map! {
    "як" => "as",
    "розбити" => "break",
    "конст" => "const",
    "продовжуй" => "continue",
    "крейт" => "crate",
    "інакше" => "else",
    "перелік" => "enum",
    "зовнішн" => "extern",
    "брехня" => "false",
    "фн" => "fn",
    "для" => "for",
    "якщо" => "if",
    "викон" => "impl",
    "у" => "in",
    "нехай" => "let",
    "петля" => "loop",
    "супостав" => "match",
    "мод" => "mod",
    "переміщує" => "move",
    "змін" => "mut",
    "пуб" => "pub",
    "пос" => "ref",
    "поверни" => "return",
    "сама" => "self",
    "Сама" => "Self",
    "статичне" => "static",
    "структ" => "struct",
    "супер" => "super",
    "вміння" => "trait",
    "істина" => "true",
    "тип" => "type",
    "небезпечно" => "unsafe",
    "ужив" => "use",
    "де" => "where",
    "допоки" => "while",
    "асинх" => "async",
    "зачекай" => "await",
    "дин" => "dyn",
    "абстрактне" => "abstract",
    "стань" => "become",
    "скриня" => "box",
    "роби" => "do",
    "остаточний" => "final",
    "макро" => "macro",
    "перевизначено" => "override",
    "попередній" => "priv",
    "дайтип" => "typeof",
    "безрозмір" => "unsized",
    "віртуал" => "virtual",
    "пропусти" => "yield",
};

///Next goes proc_macro that modifies rust syntax by changeing fn keyword to фн
#[proc_macro]
pub fn irja(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    irja_impl(input)
}

fn irja_impl(input: TokenStream) -> TokenStream {
    let intput2 = proc_macro2::TokenStream::from(input.clone());
    dbg!(intput2);
    input
        .into_iter()
        .map(|token| match &token {
            TokenTree::Ident(ident) => {
                let identstr = ident.to_string();
                if let Some(&replacement) = KEYWORDS.get(&identstr[..]) {
                    println!("replacing {} with {}", identstr, replacement);
                    TokenTree::Ident(Ident::new(replacement, ident.span()))
                } else {
                    TokenTree::Ident(Ident::new(identstr.as_str(), ident.span()))
                }
            }
            TokenTree::Group(group) => TokenTree::Group(proc_macro::Group::new(
                group.delimiter(),
                irja_impl(group.stream()),
            )),
            _ => token,
        })
        .collect()
}

#[cfg(test)]
mod test {
    #[test]
    fn test_irjast_works() {
        crate::irja! {
            фн вітаю() -> String {
                println!("Вітаю, Світе!");
            }
        }
    }
}
