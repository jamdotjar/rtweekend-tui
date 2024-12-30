#![allow(unused_imports)]
use std::{
    collections::{binary_heap, HashMap},
    rc::Rc,
};

use color_eyre::owo_colors::OwoColorize;
use rtwlib::{
    color::Color,
    hittable::HittableList,
    material::{Dielectric, Lambertian, Material, Metal, Normal},
    sphere::Sphere,
    vec3::{Point3, Vec3},
};

pub enum CurrentScreen {
    Main,
    Editor,
    MaterialEditor,
    ColorEditor,
    MaterialPicker,
    Confirmation,
    Render,
}

pub enum CurrentlyEditing {
    Size,
    PositionX,
    PositionY,
    PositionZ,
    Material,
    MatType,
    MatColor,
    MatProperty,
    ColorR,
    ColorG,
    ColorB,
}
#[derive(Clone)]
pub enum MaterialType {
    Lambertian,
    Metal,
    Dielectric,
    Normal,
}

impl std::fmt::Display for MaterialType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MaterialType::Lambertian => write!(f, "Diffuse"),
            MaterialType::Metal => write!(f, "Metal"),
            MaterialType::Normal => write!(f, "Debug"),
            MaterialType::Dielectric => write!(f, "Glass"),
        }
    }
}

pub struct App {
    pub current_screen: CurrentScreen,
    pub current_edit: Option<CurrentlyEditing>,
    pub world: HittableList,
    pub materials: Vec<(String, Rc<dyn Material>)>,
    pub material_input: usize,
    pub size_input: String,
    pub position_input_x: String,
    pub position_input_y: String,
    pub position_input_z: String,
    pub mat_type_input: Option<MaterialType>,
    pub mat_color_input: String,
    pub mat_other_input: String,
    pub mat_name_input: String,
}

impl App {
    pub fn new() -> App {
        App {
            current_screen: CurrentScreen::Main,
            current_edit: None,
            world: HittableList {
                objects: Vec::new(),
            },
            materials: vec![(
                "Diffuse 1".to_string(),
                Rc::new(Lambertian::new(Color::from(0.8))),
            )],
            material_input: 0,
            size_input: String::from("1.0"),
            position_input_x: String::from("0.0"),
            position_input_y: String::from("0.0"),
            position_input_z: String::from("0.0"),
            mat_type_input: None,
            mat_color_input: String::from("fa4e4e"),
            mat_other_input: String::from("0.0"),
            mat_name_input: String::from("Material"),
        }
    }
    pub fn save_material(&mut self) -> Result<(), String> {
        let other: f64 = self
            .mat_other_input
            .parse()
            .map_err(|_| "Invalid other value")?;
        let color: Color = self.get_color();
        let mat: Rc<dyn Material> = match &self.mat_type_input {
            Some(x) => match x {
                MaterialType::Lambertian => Rc::new(Lambertian::new(color)),
                MaterialType::Metal => Rc::new(Metal::new(color, other)),
                MaterialType::Normal => Rc::new(Normal::new()),
                MaterialType::Dielectric => Rc::new(Dielectric::new(other)),
            },
            None => return Err(String::from("No material type provided")),
        };
        self.materials.push((self.mat_name_input.clone(), mat));
        self.mat_color_input = String::from("fa4e4e");
        self.mat_type_input = None;
        self.mat_other_input = String::from("1.0");
        Ok(())
    }
    pub fn save_object(&mut self) -> Result<(), String> {
        let mat: Rc<dyn Material> = self
            .materials
            .get(self.material_input)
            .ok_or("Invalid material input")?
            .clone()
            .1;

        let size: f64 = self.size_input.parse().map_err(|_| "Invalid size input")?;

        let pos_x: f64 = self
            .position_input_x
            .parse()
            .map_err(|_| "Invalid position input x")?;

        let pos_y: f64 = self
            .position_input_y
            .parse()
            .map_err(|_| "Invalid position input y")?;

        let pos_z: f64 = self
            .position_input_z
            .parse()
            .map_err(|_| "Invalid position input z")?;

        let position = Point3::new(pos_x, pos_y, pos_z);

        let sphere = Sphere::new(position, size, mat.clone());
        self.world.add(sphere);

        self.material_input = 0;
        self.size_input = String::from("1.0");
        self.position_input_x = String::from("0.0");
        self.position_input_y = String::from("0.0");
        self.position_input_z = String::from("0.0");

        Ok(())
    }

    pub fn change_editing(&mut self) {
        if let Some(edit_mode) = &self.current_edit {
            self.current_edit = match edit_mode {
                CurrentlyEditing::Size => Some(CurrentlyEditing::PositionX),
                CurrentlyEditing::PositionX => Some(CurrentlyEditing::PositionY),
                CurrentlyEditing::PositionY => Some(CurrentlyEditing::PositionZ),
                CurrentlyEditing::PositionZ => Some(CurrentlyEditing::Material),
                CurrentlyEditing::Material => Some(CurrentlyEditing::Size),

                CurrentlyEditing::MatType => Some(CurrentlyEditing::MatColor),
                CurrentlyEditing::MatColor => Some(CurrentlyEditing::MatProperty),
                CurrentlyEditing::MatProperty => Some(CurrentlyEditing::MatType),

                CurrentlyEditing::ColorR => Some(CurrentlyEditing::ColorG),
                CurrentlyEditing::ColorG => Some(CurrentlyEditing::ColorB),
                CurrentlyEditing::ColorB => Some(CurrentlyEditing::ColorR),
            }
        } else {
            self.current_edit = match self.current_screen {
                CurrentScreen::MaterialEditor => Some(CurrentlyEditing::MatColor),
                _ => Some(CurrentlyEditing::Size),
            }
        }
    }

    pub fn display(&self) -> Result<(), ()> {
        let world = &self.world.as_simple_vec();
        for object in world {
            println!("{}", object)
        }
        Ok(())
    }
    pub fn cycle_mat_type(&mut self) {
        if let Some(material_type) = &self.mat_type_input {
            self.mat_type_input = match material_type {
                MaterialType::Lambertian => Some(MaterialType::Metal),
                MaterialType::Metal => Some(MaterialType::Dielectric),
                MaterialType::Dielectric => Some(MaterialType::Normal),
                MaterialType::Normal => Some(MaterialType::Lambertian),
            }
        }
    }
    pub fn get_color(&self) -> Color {
        let r = u8::from_str_radix(&self.mat_color_input[0..2], 16).unwrap_or(255);
        let g = u8::from_str_radix(&self.mat_color_input[2..4], 16).unwrap_or(0);
        let b = u8::from_str_radix(&self.mat_color_input[4..6], 16).unwrap_or(255);

        Color::new(r as f64 / 255.0, g as f64 / 255.0, b as f64 / 255.0)
    }
}
