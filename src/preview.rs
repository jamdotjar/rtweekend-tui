#![warn(clippy::pedantic)]

use std::{fs::File, io::Read};

use ratatui::{buffer::Buffer, layout::Rect, style::{Color, Modifier, Style}, symbols::half_block::UPPER, widgets::{canvas::Canvas, Block, BorderType, Borders, Widget}, Frame};
use rtwlib::{camera::{self, Camera}, color::linear_to_gamma, hittable::HittableList, vec3::{Point3, Vec3}};
use color_eyre::Result;
use crate::{ App};

pub fn render_preview(frame: &mut Frame, area: Rect, app: &App) -> Result<()> { 
    let preview_block = Block::default()
        .title("Preview")
        .borders(Borders::ALL).border_type(BorderType::Thick);
    let file = File::create(format!("temp.ppm")).unwrap();

    let mut cam = Camera::new(file);
    cam.aspect_ratio = area.width as f64 / (area.height*2) as f64;
    cam.image_width = area.width as u32;
    cam.samples = 5;
    cam.bounces = 5;

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

  
    let preview = Preview {
        cam: cam,
        world: app.world.clone()
    };

  
    
    frame.render_widget(preview, area);

    Ok(())
}


pub struct Preview {
    cam: Camera,
    world: HittableList
}

impl Widget for Preview {

    fn render(mut self, area: Rect, buf: &mut Buffer) {
        self.cam.initialize();
        let mut lines = Vec::new();
        for y in 0..=self.cam.get_height() - 1  {
            let mut xlines = Vec::new();
            for x in 0..=self.cam.image_width - 1 { 
                let mut color = Vec3::new(0.,0.,0.);
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

                let color_u8 =  [
                    (color_r * 255.0) as u8, 
                    (color_g * 255.0) as u8, 
                    (color_b * 255.0) as u8
                ];
                xlines.push(color_u8);
            }
            lines.push(xlines);
        }
        for y in (0..=self.cam.get_height() as usize - 1).step_by(2) {
            for x in 0..=self.cam.image_width  as usize - 1 {
                buf.set_string(area.left() + x as u16, area.top() + (y/2) as u16, "▀", Style::default()
                .fg(Color::Rgb(
                    lines[y][x][0],
                    lines[y][x][1],
                    lines[y][x][2],
                    ))
                .bg(Color::Rgb(
                    lines[y+1][x][0],
                    lines[y+1][x][1],
                    lines[y+1][x][2],

                    )));
            }
    
        }


    }
}

