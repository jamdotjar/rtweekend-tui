#![warn(clippy::pedantic)]

use std::{fs::File, io::Read};

use crate::App;
use color_eyre::Result;
use ratatui::{
    buffer::Buffer,
    layout::{Margin, Rect},
    style::{Color, Modifier, Style},
    symbols::half_block::UPPER,
    widgets::{block::BlockExt, canvas::Canvas, Block, BorderType, Borders, Widget},
    Frame,
};
use rtwlib::{
    camera::{self, Camera},
    color::linear_to_gamma,
    hittable::HittableList,
    vec3::{Point3, Vec3},
};

pub fn render_preview(frame: &mut Frame, area: Rect, app: &App, block: bool) -> Result<()> {
    let preview_block = Block::new()
        .title("Preview")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);

    let mut cam = Camera::new();
    cam.image_width = (area.width - 2).into();
    cam.image_height = (area.height * 2).into();
    cam.samples = 10;
    cam.bounces = 5;

    cam.sky = app.sky.clone();

    cam.lookfrom = Point3::new(
        app.camx.parse::<f64>()?,
        app.camy.parse::<f64>()?,
        app.camz.parse::<f64>()?,
    );

    cam.lookat = Point3::new(
        app.lookx.parse::<f64>()?,
        app.looky.parse::<f64>()?,
        app.lookz.parse::<f64>()?,
    );

    cam.vup = Point3::new(0.0, 1.0, 0.0);

    cam.vfov = app.fov.parse::<f64>()?;
    cam.focus_dist = app.focus_dist.parse::<f64>()?;
    cam.defocus_angle = app.aperture.parse::<f64>()?;
    let preview: Preview;

    if block {
        preview = Preview::new(app.world.clone())
            .block(preview_block)
            .camera(cam);
    } else {
        preview = Preview::new(app.world.clone()).camera(cam);
    }

    frame.render_widget(preview, area);

    Ok(())
}

pub struct Preview<'a> {
    cam: Camera,
    world: HittableList,
    block: Option<Block<'a>>,
}

impl<'a> Preview<'a> {
    fn new(world: HittableList) -> Self {
        Self {
            cam: Camera::default(),
            world,
            block: None,
        }
    }

    #[must_use]
    pub fn block(mut self, block: Block<'a>) -> Self {
        self.block = Some(block);
        self
    }

    pub fn camera(mut self, cam: Camera) -> Self {
        self.cam = cam;
        self
    }
}

impl<'a> Widget for Preview<'a> {
    fn render(mut self, area: Rect, buf: &mut Buffer) {
        self.cam.initialize();
        let mut lines = Vec::new();
        for y in 0..=self.cam.get_height() - 1 {
            let mut xlines = Vec::new();
            for x in 0..=self.cam.image_width - 1 {
                let mut color = Vec3::new(0., 0., 0.);
                for _ in 0..self.cam.samples {
                    //gets jittered rays per sample, averages result.
                    let r = self.cam.get_ray(x.into(), y.into());
                    color += self.cam.ray_color(r, 5, &self.world);
                    //println!("{:?}\n{:?}", r, self.cam.ray_color(r, 5, &self.world));
                }
                color = color * self.cam.get_sample_scale();

                let color_r = linear_to_gamma(color.x);
                let color_g = linear_to_gamma(color.y);
                let color_b = linear_to_gamma(color.z);

                let color_u8 = [
                    (color_r * 255.0) as u8,
                    (color_g * 255.0) as u8,
                    (color_b * 255.0) as u8,
                ];
                xlines.push(color_u8);
            }
            lines.push(xlines);
        }
        self.block.render(area, buf);
        let area = self.block.inner_if_some(area);

        for y in (0..=lines.len() as usize - 1).step_by(2) {
            for x in 0..=lines[y].len() as usize - 1 {
                if (y / 2 >= area.height.into()) | (x >= lines[y].len()) {
                    //put this back if resizing starts crashing stuff
                    break;
                }

                buf.set_string(
                    area.left() + x as u16,
                    area.top() + (y / 2) as u16,
                    "▀",
                    Style::default()
                        .fg(Color::Rgb(lines[y][x][0], lines[y][x][1], lines[y][x][2]))
                        .bg(Color::Rgb(
                            lines[y + 1][x][0],
                            lines[y + 1][x][1],
                            lines[y + 1][x][2],
                        )),
                );
            }
        }
    }
}
