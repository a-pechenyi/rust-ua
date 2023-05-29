#![feature(ip_in_core)]
#![feature(error_in_core)]
use async_trait::async_trait;
use std::collections::HashMap;
use syn::{parse_str, spanned::Spanned};

// mod syn_ua {

//     #[derive(Debug)]
//     pub struct File {
//         inner: syn::File,
//     }

//     pub struct Fn;

//     impl syn::parse::Parse for Fn {
//         fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {

//             input.parse::<syn::Ident>()?;
//             input.parse::<syn::Token![fn]>()?;
//             input.parse::<syn::Ident>()?;
//             input.parse::<syn::Token![(]>()?;
//             input.parse::<syn::Token![)]>()?;
//             input.parse::<syn::Token![->]>()?;
//             input.parse::<syn::Type>()?;
//             input.parse::<syn::Token![;]>()?;
//             Ok(Fn)
//         }
//     }

//     impl syn::parse::Parse for File {
//         fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
//             input.peek(syn::Token![fn]);
//         }
//     }
// }

// fn main() {
//     let t: HashMap<&'static str, &'static str> = HashMap::from([("фн", "fn")]);
//     const CODE: &str = "фн main() {}";
//     // let f: syn::File = parse_str("").unwrap();

//     let tokens: TokenStream = CODE.parse().unwrap();
//     let transformedTokens: TokenStream = tokens
//         .into_iter()
//         .map(|token| match &token {
//             TokenTree::Ident(ident) => {
//                 let ident = ident.to_string();
//                 if let Some(&replacement) = t.get(&ident[..]) {
//                     println!("replacing {} with {}", ident, replacement);
//                     let new_span = ident.span().located_at(ident.span());
//                     TokenTree::Ident(Ident::new(replacement, new_span))
//                 } else {
//                     TokenTree::Ident(Ident::new(ident.as_str(), ident.span()))
//                 }
//             }
//             _ => token,
//         })
//         .collect();

//     println!("{}", transformedTokens.to_string());
// }

/// New main

fn main() {
    func_from_irja();
}

irja::irja! {
    фн func_from_irja() {
        фн внутрішня_функція() {
            println!("Вітаю із середини");
        }

        println!("Вітаю, Світе!");
        внутрішня_функція();
    }
}

mod try_hyper {
    use std::{future::Future, net::TcpStream, os::unix::net::SocketAddr};

    use hyper::body::Buf;

    irja::irja! {
        пуб мод стд {
            пуб мод хиба {
                пуб ужив std::error::Error як Хиба;
            }
            пуб мод конверт {
                пуб ужив std::convert::Infallible як Незламний;
            }
            пуб мод сітка {
                пуб ужив std::net::SocketAddr як АдресаГнізда;
            }
        }

        пуб мод гайпер {
            пуб мод тіло {
                пуб ужив hyper::body::Bytes як Байти;
                пуб ужив hyper::body::Incoming як Входяще;
                пуб ужив hyper::body::Body як Тіло;
            }
            пуб мод сервер {
                пуб мод підкл {
                    пуб мод ппгт1 {
                        пуб ужив hyper::server::conn::http1::Builder як Будівельник;
                        пуб ужив hyper::server::conn::http1::Connection як Підключення;
                    }
                }
            }

            пуб мод послуга {
                пуб ужив hyper::service::service_fn як послуга_фн;
            }

            пуб ужив hyper::Request як Запит;
            пуб ужив hyper::Response як Відгук;

        }

        пуб мод ппгт_тіло_помічник {
            пуб ужив http_body_util::Full as Повний;
        }

        пуб мод тікіо {
            пуб мод сітка {
                пуб ужив tokio::net::TcpListener як ПкпСлухач;
            }
            пуб мод задача {
                пуб ужив tokio::task::spawn як породи;
            }
            пуб ужив tokio::main як головне;
        }

        вміння ВідгукПереклад<Тип> {
            фн новий(тіло: Тип) -> Відгук<Тип>;
        }

        викон<Тип> ВідгукПереклад<Тип> для Відгук<Тип> {
            фн новий(тіло: Тип) -> Відгук<Тип> {
                Відгук::new(тіло)
            }
        }


        вміння СкриняПереклад<Тип> {
            фн новий(тіло: Тип) -> Скриня<Тип>;
        }

        викон<Тип> СкриняПереклад<Тип> для Скриня<Тип> {
            фн новий(а: Тип) -> Скриня<Тип> {
                Скриня::new(а)
            }
        }

        вміння ПовнийПереклад<Тип> {
            фн новий(тіло: Тип) -> Повний<Тип>;
        }


        викон<Д> ПовнийПереклад<Д> для Повний<Д>
        де Д: Buf {
            фн новий(тіло: Д) -> Повний<Д> {
                Повний::new(тіло)
            }
        }

        вміння ІзПереклад<Тип> {
            фн із(a: Тип) -> Сама;
        }

        викон<ТипА, ТипБ> ІзПереклад<ТипА> для ТипБ
        де ТипБ: From<ТипА> {
            фн із(a: ТипА) -> ТипБ {
                ТипБ::from(a)
            }
        }

        асинх фн повяжи(адр: АдресаГнізда) -> std::io::Result<ПкпСлухач> {
            ПкпСлухач::bind(адр).зачекай
        }


        асинх фн приймай(слухач: &ПкпСлухач) -> std::io::Result<(tokio::net::TcpStream, core::net::SocketAddr)> {
            слухач.accept().зачекай
        }

        вміння БудівельникПереклад {
            фн новий() -> ппгт1::Будівельник;
            фн обслужити_підключення<І, П>(сама, іо: І, послуга: П) -> гайпер::сервер::підкл::ппгт1::Підключення<І, П>
            де
                П: hyper::service::HttpService<hyper::body::Incoming>,
                П::Error: Into<Box<dyn core::error::Error + Send + Sync>>,
                П::ResBody: 'static,
                <П::ResBody as гайпер::тіло::Тіло>::Error: Into<Box<dyn core::error::Error + Send + Sync>>,
                І: tokio::io::AsyncRead + tokio::io::AsyncWrite + Unpin;
        }

        викон БудівельникПереклад для ппгт1::Будівельник {
            фн новий() -> Сама {
                ппгт1::Будівельник::new()
            }

            фн обслужити_підключення<І, П>(сама, іо: І, послуга: П) -> гайпер::сервер::підкл::ппгт1::Підключення<І, П>
            де
                П: hyper::service::HttpService<hyper::body::Incoming>,
                П::Error: Into<Box<dyn core::error::Error + Send + Sync>>,
                П::ResBody: 'static,
                <П::ResBody as гайпер::тіло::Тіло>::Error: Into<Box<dyn core::error::Error + Send + Sync>>,
                І: tokio::io::AsyncRead + tokio::io::AsyncWrite + Unpin,
            {
                сама.serve_connection(іо, послуга)
            }
        }


        ужив Result як Результат;
        ужив Результат::Ok як Гаразд;
        ужив Результат::Err як Хиб;
        ужив std::boxed::Box як Скриня;
        ужив Sync as Синх;
        ужив Send as Слать;
        ужив println as друкряд;




        ужив стд::конверт::Незламний;
        ужив стд::сітка::АдресаГнізда;

        ужив гайпер::тіло::Байти;
        ужив гайпер::{Запит, Відгук};
        ужив ппгт_тіло_помічник::Повний;
        ужив гайпер::{сервер::підкл::ппгт1, послуга::послуга_фн};
        ужив тікіо::сітка::ПкпСлухач;

        асинх фн вітаю(_: Запит<гайпер::тіло::Входяще>) -> Результат<Відгук<Повний<Байти>>, Незламний> {
            Гаразд(Відгук::новий(Повний::новий(Байти::із("Вітаю, Світе!"))))
        }

        #[тікіо::головне]
        асинх фн головне() -> Результат<(), Скриня<дин стд::хиба::Хиба + Слать + Синх>> {
            нехай адр = АдресаГнізда::із(([127, 0, 0, 1], 3000));
            // ПКП - Протокол контролю передачі
            // Створюємо ПкпСлухача та пов'язуємо його з 127.0.0.1:3000
            нехай слухач: ПкпСлухач = повяжи(адр).зачекай?;

            // Ми починаємо петлю щоб постійно приймати нові підключення
            петля {
                нехай (струмок, _) = приймай(&слухач).зачекай?;

                // Породжуємо (spawn) Токіо задачу щоб обслуговувати багато підключень одночасно
                тікіо::задача::породи(асинх переміщує {
                    // Нарешті, ми зв'язуємо вхідні підключення із нашою послугою 'вітаю'
                    // ппгт - протокол передачі гіпер тексту
                    якщо нехай Хиб(хиба) = ппгт1::Будівельник::новий()
                        // `послуга_фн` перетворює нашу функцію на `Послуга`
                        .обслужити_підключення(струмок, послуга_фн(вітаю))
                        .зачекай
                    {
                        друкряд!("Помилка обслуговування підключення: {:?}", хиба);
                    }
                });
            }
        }


    }
}
