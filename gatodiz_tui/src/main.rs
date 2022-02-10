use cursive::{Cursive, CursiveExt};
use cursive::views::{TextView, Dialog, ListView, EditView, Checkbox};
use cursive::event::Key;
use cursive::traits::Nameable;

struct CatsayOptions<'a> {
    message: &'a str,
    dead: bool,
}

fn input_step(siv: &mut Cursive) {
    siv.add_layer(
        Dialog::new()
            .title("Por favor, preencha o formulario para o gato!")
            // Configurando o título
            .content(
                ListView::new()
                    .child(
                        "Mensagem:",
                        EditView::new().with_name("message")
                    )
                    .child(
                        "Morto?",
                        Checkbox::new().with_name("dead")
                    )
            )
            .button("OK", |s| {
                let message = s.call_on_name(
                    "message", 
                    |t: &mut EditView| t.get_content()
                ).unwrap();
                let is_dead = s.call_on_name(
                    "dead", 
                    |t: &mut Checkbox| t.is_checked()
                ).unwrap();
                let options = CatsayOptions {
                    message: &message, 
                    dead: is_dead,
                };
                result_step(s, &options)
            }),
        );
}

fn result_step(siv: &mut Cursive, options: &CatsayOptions) {
    let eye = if options.dead { "x" } else { "o" };
    let cat_text = format!(
        "{msg}
        \\
         \\
             /\\_/\\
            ( {eye} {eye} )
            =( I )=", msg=options.message, eye=eye);
    siv.pop_layer();
    siv.add_layer(
        Dialog::around(TextView::new(cat_text))
            .title("O gato diz...")
            .button("Ok", |s| s.quit()),
    );
}

fn main() {
    let mut siv = Cursive::default();
    input_step(&mut siv);
    // Observar a Key::Esc e sair
    siv.add_global_callback(Key::Esc, |s| s.quit());
    siv.run();
}
