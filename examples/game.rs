extern "C" {
    pub fn log_number(number: usize);
}

fn main() {
    let mut blue_amount = 0.0;

    trickle::set_event_handler(move || {
        blue_amount += 0.1;
        trickle::clear_screen_to_color(0.0, 0.0, blue_amount, 1.0);
    });
}