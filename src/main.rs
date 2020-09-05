use cursive::theme::ColorStyle;
use cursive::views::TextView;

fn main() {
    let mut siv = cursive::default();

    siv.set_theme(theme: theme::Theme)

    siv.add_global_callback('q', |s| s.quit());

    siv.add_layer(TextView::new("Hello, world!"));

    siv.run();
}
