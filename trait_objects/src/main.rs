use trait_objects::{Button, Screen};

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(Button {
                width: 75,
                height: 10,
                label: "Button 1".to_string(),
            }),
            Box::new(Button {
                width: 75,
                height: 10,
                label: "Button 2".to_string(),
            }),
        ]
    };
    screen.run();
}
