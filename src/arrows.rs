
use std::error::Error;
use ratatui::{
    style::Color,
    widgets::{
        canvas::{Canvas, Context, Line},
        Block, Borders,
    },
};
pub const RIGHTARROW: Arrow = Arrow {
    base_line: Line {
        x1: -5.0,
        y1: 0.0,
        x2: 3.5,
        y2: 0.0,
        color: Color::Green,
    }, // First line of the arrowhead
    left_line: Line {
        x1: 0.0,
        y1: 9.0,
        x2: 3.5,
        y2: 0.0,
        color: Color::Green,
    }, // First line of the arrowhead
    right_line: Line {
        x1: 0.0,
        y1: -9.0,
        x2: 3.5,
        y2: 0.0,
        color: Color::Green,
    }, // First line of the arrowhead
};
pub const LEFTARROW: Arrow = Arrow {
    base_line: Line {
        x1: -5.0,
        y1: 0.0,
        x2: 3.5,
        y2: 0.0,
        color: Color::Green,
    }, // First line of the arrowhead
    left_line: Line {
        x1: 0.0,
        y1: 9.0,
        x2: -5.0,
        y2: 0.0,
        color: Color::Green,
    }, // First line of the arrowhead
    right_line: Line {
        x1: 0.0,
        y1: -9.0,
        x2: -5.0,
        y2: 0.0,
        color: Color::Green,
    },
};

pub const UPARROW: Arrow = Arrow {
    base_line: Line {
        x1: 0.0,
        y1: 5.0,
        x2: 0.0,
        y2: -5.0,
        color: Color::Green,
    }, // First line of the arrowhead
    left_line: Line {
        x1: 0.0,
        y1: 9.0,
        x2: -5.0,
        y2: 0.0,
        color: Color::Green,
    }, // First line of the arrowhead
    right_line: Line {
        x1: 0.0,
        y1: 9.0,
        x2: 5.0,
        y2: 0.0,
        color: Color::Green,
    },
};
pub const DOWNARROW: Arrow = Arrow {
    base_line: Line {
        x1: 0.0,
        y1: 5.0,
        x2: 0.0,
        y2: -5.0,
        color: Color::Green,
    }, // First line of the arrowhead
    left_line: Line {
        x1: 0.0,
        y1: -9.0,
        x2: -5.0,
        y2: 0.0,
        color: Color::Green,
    }, // First line of the arrowhead
    right_line: Line {
        x1: 0.0,
        y1: -9.0,
        x2: 5.0,
        y2: 0.0,
        color: Color::Green,
    },
};
pub fn draw_arrow(
    arrow: Arrow,
) -> Result<Canvas<'static, impl Fn(&mut Context<'_>)>, Box<dyn Error>> {
    let canvas = Canvas::default()
        .paint(move |ctx| {
            ctx.draw(&arrow.base_line);
            ctx.draw(&arrow.right_line);
            ctx.draw(&arrow.left_line);
        })
        .block(Block::default().borders(Borders::ALL))
        .y_bounds([-10.0, 10.0])
        .x_bounds([-5.0, 5.0]);
    Ok(canvas)
}
#[derive(Clone)]
pub struct Arrow {
    base_line: Line,
    left_line: Line,
    right_line: Line,
}
