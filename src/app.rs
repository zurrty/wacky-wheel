use std::f32::consts::PI;

use crate::wheel::*;
use macroquad::prelude::*;

const STOP_THRESHOLD: f32 = 0.00005;
const TWOPI: f32 = PI * 2.0;
const ARC_SEGMENTS: usize = 16;
/// how fast the wheel slows down. lower = faster.
const FRICTION: f32 = 0.98;
#[derive(Debug)]
pub struct App {
    pub wheel: Wheel,
    pub font: Font,
    pub pallete: Vec<Color>,
    time: f64,


    pub is_spinning: bool,
    velocity: f32,
    angle: f32,
}

impl App {
    pub fn new(wheel: Wheel, font: Font) -> Self {
        let pallete = wheel.get_pallete();
        Self {
            wheel: wheel,
            font,
            pallete,
            time: get_time(),
            is_spinning: false,
            velocity: 0.0,
            angle: 0.0,
        }
    }
    pub fn get_index(&self) -> usize {
        let choices = &self.wheel.choices;
        let len = choices.len();
        // convert len to f32
        let len_f = len as f32;

        // scale the angle by
        let angle_scaled = len_f * self.angle / TWOPI;
        ((len_f - 1.0) - (angle_scaled % len_f)).ceil() as usize % len
    }
    pub fn delta_time(&self) -> f32 {
        (get_time() - self.time) as f32
    }
    pub fn start_spin(&mut self) {
        self.time = get_time();
        let rand_speed: f32 = rand::gen_range(0.4, 1.5);
        self.angle = 0.0;
        self.velocity = rand_speed;
        self.is_spinning = true;
    }
    pub fn update_wheel(&mut self) {
        if self.is_spinning {
            self.angle += self.velocity * self.delta_time();
            self.velocity *= FRICTION;
        }
        if self.velocity < STOP_THRESHOLD {
            self.is_spinning = false;
            self.velocity = 0.0;
        }
    }
    pub fn draw_wheel(&self) {
        let choices = &self.wheel.choices;
        let origin = vec2(screen_width() / 2.0, screen_height() / 2.0);
        let radius = get_wheel_radius();
        let length = choices.len();
        let arclen = TWOPI / length as f32;

        // dont show less than 8 segments
        // let num_segments = usize::max(8, length);

        for i in 0..length {
            let offset = i as f32 * arclen;
            let choice = choices.get(i % length).unwrap();
            let segment_color = self.pallete.get(i % self.pallete.len()).cloned().unwrap_or(RED);
            let angle1 = self.angle + offset - PI / 2.0;
            let angle2 = angle1 + arclen;
            self.draw_segment(&choice.name, origin, radius, angle1, angle2, segment_color);
        }

        let arrow_pos = Vec2::new(origin.x, 24.0);
        draw_triangle(
            arrow_pos - vec2(20.0, 0.0),
            arrow_pos + vec2(20.0, 0.0),
            arrow_pos + vec2(0.0, 32.0),
            WHITE,
        );
        draw_triangle_lines(
            arrow_pos - vec2(20.0, 0.0),
            arrow_pos + vec2(20.0, 0.0),
            arrow_pos + vec2(0.0, 32.0),
            4.0,
            BLACK,
        );
    }
    fn draw_segment(
        &self,
        text: &str,
        origin: Vec2,
        radius: f32,
        angle1: f32,
        angle2: f32,
        color: Color,
    ) {
        let angle_diff = angle2 - angle1;
        let segment_angle = angle_diff / ARC_SEGMENTS as f32;

        for i in 0..ARC_SEGMENTS {
            let a1 = angle1 + i as f32 * segment_angle;
            let v1 = Vec2::from_angle(a1) * radius;

            let a2 = angle1 + (i + 1) as f32 * segment_angle;
            let v2 = Vec2::from_angle(a2) * radius;
            draw_triangle(origin, v1 + origin, v2 + origin, color)
        }
        let text_scale = radius / 400.0;
        let dimensions = measure_text(text, Some(&self.font), 24, text_scale);
        let height = dimensions.height;
        let text_angle = angle1 + angle_diff / 2.0;
        let text_pos =
            origin + vec2(radius / 8.0, height / 2.0).rotate(Vec2::from_angle(text_angle));
        
        let text_color = if (color.r + color.g + color.b) / 3.0 < 0.5 {
            WHITE
        } else { BLACK };

        draw_text_ex(
            text,
            text_pos.x,
            text_pos.y,
            TextParams {
                font: Some(&self.font),
                font_size: 24,
                font_scale: text_scale,
                color: text_color,
                rotation: text_angle,
                ..Default::default()
            },
        );

        let (x,y) = (origin + Vec2::from_angle(angle1) * radius).into();
        draw_line(origin.x, origin.y, x, y, 4.0, BLACK);
        let (x,y) = (origin + Vec2::from_angle(angle2) * radius).into();
        draw_line(origin.x, origin.y, x, y, 4.0, BLACK);
    }
}

fn get_wheel_radius() -> f32 {
    let smallest_axis = f32::min(screen_width(), screen_height());
    return (smallest_axis / 2.0) - 32.0;
}
