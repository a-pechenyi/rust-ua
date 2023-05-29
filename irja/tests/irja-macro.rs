#[test]
fn irja_macro() {
    irja::irja! {
        фн вітаю() {
            println!("Вітаю вас, друзі!");
        }
    };

    вітаю();
}
