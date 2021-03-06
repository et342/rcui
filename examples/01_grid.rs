use rcui::*;

fn text_cell(s: &str) -> Box<Text> {
    Box::new(Text {
        text: s.to_string(),
        halign: HAlign::Centre,
        valign: VAlign::Centre,
    })
}

fn main() {
    Rcui::exec(Proxy::wrap(
        |origin, rcui, event| {
            if let Event::KeyStroke(key) = event {
                if *key as u8 as char == 'q' {
                    rcui.quit();
                }
            }
            origin.handle_event(rcui, event);
        },
        Column::new(vec![
            Cell::Fixed(3.0, text_cell("This is the Grid Example:")),
            Cell::Many(
                3,
                Row::wrap(vec![
                    Cell::Many(1, text_cell("hello")),
                    Cell::Many(1, text_cell("hello")),
                    Cell::One(text_cell("hello")),
                ]),
            ),
            Cell::Many(
                2,
                Row::wrap(vec![
                    Cell::One(text_cell("world")),
                    Cell::One(text_cell("world")),
                    Cell::One(text_cell("world")),
                ]),
            ),
            Cell::One(Row::wrap(vec![
                Cell::One(text_cell("foo")),
                Cell::One(text_cell("foo")),
                Cell::One(text_cell("foo")),
            ])),
            Cell::One(Row::wrap(vec![
                Cell::One(text_cell("bar")),
                Cell::One(text_cell("bar")),
                Cell::One(text_cell("bar")),
            ])),
        ]),
    ));

    println!("Quitting gracefully uwu");
}
