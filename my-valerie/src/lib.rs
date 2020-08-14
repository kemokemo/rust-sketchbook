#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

// copy from the valerie official sample.
//  https://github.com/emmanuelantony2000/valerie#add-and-subtract-one-using-a-button
use valerie::prelude::components::*;
use valerie::prelude::*;

fn ui() -> Node {
    let value = StateAtomic::new(0isize);

    div!(
        h1!("Value ", value.clone()),
        button!("Add 1")
            .on_event("click", value.clone(), move |x, _| {
                *x += 1;
            }),
        button!("Subtract 1")
            .on_event("click", value.clone(), move |x, _| {
                *x -= 1;
            })
    )
    .into()
}

#[valerie(start)]
pub fn run() {
    App::render_single(ui());
}
