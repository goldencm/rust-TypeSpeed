use super::app::{Ids, Game, Data};
use super::conrod_core;
use rand::Rng;

pub fn update(ref mut ui: conrod_core::UiCell, ids: &Ids, game : &mut Game, data : &mut Data) {
    use conrod_core::Widget;
    use conrod_core::widget::text_box;
    use conrod_core::widget::range_slider;
    use conrod_core::widget::{Button, Canvas, RangeSlider, Text, TextBox};
    use conrod_core::{color, Colorable, Labelable, Positionable, Sizeable};

    let caption = format!("Hello World!");
    
    Canvas::new()
        .color(color::WHITE)
        .pad(0.0)
        .set(ids.canvas, ui);

    if !game.end() {
        // for _click in Button::new()
        //     .top_left_with_margin_on(ids.canvas, 0.0)
        //     .label("Guess!")
        //     .w_h(100.0, 40.0)
        //     .color(color::WHITE)
        //     .press_color(color::RED)
        //     .set(ids.guess_button, ui)
        // {
            
        // }

        
        Text::new("").top_left_with_margin_on(ids.canvas, 0.0)
            .x_direction_from(ids.canvas, conrod_core::position::Direction::Backwards, 150 as f64)
            .set(ids.empty, ui);
        

        for (key, value) in game.strings.clone() {
            if value.2 > ui.win_h + 425 as f64 {
                let (word, multi, xpos) = game.strings.get(&key).unwrap();
                let to_set = (game.next_word(), rand::thread_rng().gen_range(0, 10) * game.speedMultiplier, 0 as f64);
                *game.strings.get_mut(&key).unwrap() = to_set;
                game.failed += 1;
            } else {
                match key {
                    0 => Text::new(&value.0).down_from(ids.empty, 0.0)
                            .x_direction_from(ids.empty, conrod_core::position::Direction::Forwards, value.2)
                            .font_size(10)
                            .set(ids.zero, ui),
                    1 => Text::new(&value.0).down_from(ids.zero, 0.0)
                            .x_direction_from(ids.empty, conrod_core::position::Direction::Forwards, value.2)
                            .font_size(10)
                            .set(ids.one, ui),
                    2 => Text::new(&value.0).down_from(ids.one, 0.0)
                            .x_direction_from(ids.empty, conrod_core::position::Direction::Forwards, value.2)
                            .font_size(10)
                            .set(ids.two, ui),
                    3 => Text::new(&value.0).down_from(ids.two, 0.0)
                            .x_direction_from(ids.empty, conrod_core::position::Direction::Forwards, value.2)
                            .font_size(10)
                            .set(ids.three, ui),
                    4 => Text::new(&value.0).down_from(ids.three, 0.0)
                            .x_direction_from(ids.empty, conrod_core::position::Direction::Forwards, value.2)
                            .font_size(10)
                            .set(ids.four, ui),
                    5 => Text::new(&value.0).down_from(ids.four, 0.0)
                            .x_direction_from(ids.empty, conrod_core::position::Direction::Forwards, value.2)
                            .font_size(10)
                            .set(ids.five, ui),
                    6 => Text::new(&value.0).down_from(ids.five, 0.0)
                            .x_direction_from(ids.empty, conrod_core::position::Direction::Forwards, value.2)
                            .font_size(10)
                            .set(ids.six, ui),
                    7 => Text::new(&value.0).down_from(ids.six, 0.0)
                            .x_direction_from(ids.empty, conrod_core::position::Direction::Forwards, value.2)
                            .font_size(10)
                            .set(ids.seven, ui),
                    8 => Text::new(&value.0).down_from(ids.seven, 0.0)
                            .x_direction_from(ids.empty, conrod_core::position::Direction::Forwards, value.2)
                            .font_size(10)
                            .set(ids.eight, ui),
                    _ => Text::new(&value.0).down_from(ids.eight, 0.0)
                            .x_direction_from(ids.empty, conrod_core::position::Direction::Forwards, value.2)
                            .font_size(10)
                            .set(ids.nine, ui),
    
                }
                let (word, multi, xpos) = game.strings.get(&key).unwrap();
                let to_set = (word.clone(), *multi, *xpos + (0.01 * (*multi as f64)));
                *game.strings.get_mut(&key).unwrap() = to_set;
            }
            
            
        }
        for edit in TextBox::new(&data.get_attempt())
            .down_from(ids.nine, 0.0)
            .x_direction_from(ids.empty ,conrod_core::position::Direction::Forwards, 150 as f64)
            .w_h(500.0, 20.0)
            .set(ids.textbox, ui)
        {
            match edit {
                text_box::Event::Enter => {
                    for (key, value) in game.strings.clone() {
                        
                        if data.get_attempt().eq_ignore_ascii_case(&value.0) {
                            
                            let (word, multi, xpos) = game.strings.get(&key).unwrap();
                            let to_set = (game.next_word(), rand::thread_rng().gen_range(0, 10) * game.speedMultiplier, 0 as f64);
                            *game.strings.get_mut(&key).unwrap() = to_set;
                            game.score += 1;
                            game.speedMultiplier += 1;
                            break;
                        } 
                    } 
                    data.new_attempt("");
                }
                text_box::Event::Update(text) => {
                    data.new_attempt(&text);
                }
            }
        }
        let score = "Score ".to_string() + &game.score.to_string();
        Text::new(&score)
            .right_from(ids.textbox, 0.0)
            .set(ids.info_text, ui);
        
        let failed = "| Failures ".to_string() + &game.failed.to_string();
        Text::new(&failed)
            .right_from(ids.info_text, 0.0)
            .set(ids.count_text, ui);
    
    }
}