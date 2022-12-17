use flows::SokobanLauncher;
use tiny::flow::AppFlow;
use tiny::flows::IntroFlow;
use tiny::platform::Platform;

mod flows;
mod level;
mod sokoban;

fn main() {
    let controls = [
        ("wasd", "move"),
        ("r", "reset"),
        ("u", "undo"),
        ("q", "quit"),
    ];

    let mut platform = Platform::new();
    let mut app = AppFlow::new()
        .with_flow(IntroFlow::<SokobanLauncher>::new("Sokoban").with_controls(&controls));

    while !app.should_quit() {
        Platform::clear_display();
        app.render();

        let input_char = platform.read_char();
        let key = Platform::translate_input(input_char);

        app.handle_key(key);
        app.update();
    }
}
