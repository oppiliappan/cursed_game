mod universe;

use cursive::Cursive;
use cursive::views::{ TextView, LinearLayout, Button, DummyView };
use cursive::view::Identifiable;

static WIDTH: u32 = 30;
static HEIGHT: u32 = 30;

fn main() {

    let mut main = Cursive::default();
    let myuniverse = universe::Universe::new(WIDTH, HEIGHT);

    let grid = TextView::new(format!("{}", myuniverse))
        .with_id("grid");

    let buttons = LinearLayout::vertical()
        .child(Button::new("Tick", move |s| {
            s.call_on_id("grid", |view: &mut TextView| {
                view.set_content(format!("{}", myuniverse.tick()))
            });
        }))
        .child(Button::new("Reload", |s| {
            let fresh = universe::Universe::new(WIDTH, HEIGHT);
            load_universe(s, &fresh);
        }))
        .child(DummyView)
        .child(Button::new("Quit", |s| s.quit()));

    main.add_layer(
            LinearLayout::horizontal()
            .child(grid)
            .child(buttons)
    );

    main.run();

}

fn load_universe(main: &mut Cursive, u: &universe::Universe) {
    main.call_on_id("grid", |view: &mut TextView| {
        view.set_content(format!("{}", u))
    });
}

// fn tick(main: &mut Cursive, u: &mut universe::Universe) {
//     u.tick();
//     main.call_on_id("grid", |view: &mut TextView| {
//         view.set_content(format!("{}", u));
//     });
// }
