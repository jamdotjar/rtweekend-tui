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
    MatName,
    // Render
    Height,
    Width,
    ImgName,
    Samples,
    Bounces,
    CamX,
    CamY,
    CamZ,
    LookX,
    LookY,
    LookZ,
    Fov,
    FocusDist,
    Aperture,
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
    pub image_name_input: String,
    pub image_height: String,
    pub image_width: String,
    pub samples: String,
    pub bounces: String,
    pub camx: String,
    pub camy: String,
    pub camz: String,
    pub lookx: String,
    pub looky: String,
    pub lookz: String,
    pub fov: String,
    pub focus_dist: String,
    pub aperture: String,
    pub render_progress: f64,
    pub selected_object: Option<usize>,
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
            image_height: String::from("600"),
            image_width: String::from("338"),
            image_name_input: String::from("image"),
            samples: String::from("50"),
            bounces: String::from("15"),
            camx: String::from("-1.0"),
            camy: String::from("0.0"),
            camz: String::from("0.0"),
            lookx: String::from("0.0"),
            looky: String::from("0.0"),
            lookz: String::from("0.0"),
            fov: String::from("45.0"),
            focus_dist: String::from("1.0"),
            aperture: String::from("0.0"),
            render_progress: 0.0,
            selected_object: None,
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
        if str::len(&self.mat_color_input) != 6 {
            return Color::new(1., 0., 1.);
        }
        let r = u8::from_str_radix(&self.mat_color_input[0..2], 16).unwrap_or_else(|_| 255);
        let g = u8::from_str_radix(&self.mat_color_input[2..4], 16).unwrap_or_else(|_| 0);
        let b = u8::from_str_radix(&self.mat_color_input[4..6], 16).unwrap_or_else(|_| 255);

        Color::new(r as f64 / 255.0, g as f64 / 255.0, b as f64 / 255.0)
    }

    pub fn change_editing(&mut self, forwards: bool) {
        if let Some(edit_mode) = &self.current_edit {
            self.current_edit = match (edit_mode, forwards) {
                (CurrentlyEditing::Size, true) => Some(CurrentlyEditing::PositionX),
                (CurrentlyEditing::PositionX, true) => Some(CurrentlyEditing::PositionY),
                (CurrentlyEditing::PositionY, true) => Some(CurrentlyEditing::PositionZ),
                (CurrentlyEditing::PositionZ, true) => Some(CurrentlyEditing::Material),
                (CurrentlyEditing::Material, true) => Some(CurrentlyEditing::Size),

                (CurrentlyEditing::MatType, true) => Some(CurrentlyEditing::MatColor),
                (CurrentlyEditing::MatColor, true) => match self.mat_type_input {
                    Some(MaterialType::Dielectric) | Some(MaterialType::Metal) => {
                        Some(CurrentlyEditing::MatProperty)
                    }
                    _ => Some(CurrentlyEditing::MatName),
                },
                (CurrentlyEditing::MatProperty, true) => Some(CurrentlyEditing::MatName),
                (CurrentlyEditing::MatName, true) => Some(CurrentlyEditing::MatType),

                (CurrentlyEditing::Width, true) => Some(CurrentlyEditing::Height),
                (CurrentlyEditing::Height, true) => Some(CurrentlyEditing::ImgName),
                (CurrentlyEditing::ImgName, true) => Some(CurrentlyEditing::Samples),
                (CurrentlyEditing::Samples, true) => Some(CurrentlyEditing::Bounces),
                (CurrentlyEditing::Bounces, true) => Some(CurrentlyEditing::CamX),
                (CurrentlyEditing::CamX, true) => Some(CurrentlyEditing::CamY),
                (CurrentlyEditing::CamY, true) => Some(CurrentlyEditing::CamZ),
                (CurrentlyEditing::CamZ, true) => Some(CurrentlyEditing::LookX),
                (CurrentlyEditing::LookX, true) => Some(CurrentlyEditing::LookY),
                (CurrentlyEditing::LookY, true) => Some(CurrentlyEditing::LookZ),
                (CurrentlyEditing::LookZ, true) => Some(CurrentlyEditing::Fov),
                (CurrentlyEditing::Fov, true) => Some(CurrentlyEditing::FocusDist),
                (CurrentlyEditing::FocusDist, true) => Some(CurrentlyEditing::Aperture),
                (CurrentlyEditing::Aperture, true) => Some(CurrentlyEditing::Width),

                (CurrentlyEditing::Size, false) => Some(CurrentlyEditing::Material),
                (CurrentlyEditing::PositionX, false) => Some(CurrentlyEditing::Size),
                (CurrentlyEditing::PositionY, false) => Some(CurrentlyEditing::PositionX),
                (CurrentlyEditing::PositionZ, false) => Some(CurrentlyEditing::PositionY),
                (CurrentlyEditing::Material, false) => Some(CurrentlyEditing::PositionZ),

                (CurrentlyEditing::MatType, false) => Some(CurrentlyEditing::MatName),
                (CurrentlyEditing::MatColor, false) => Some(CurrentlyEditing::MatType),
                (CurrentlyEditing::MatProperty, false) => Some(CurrentlyEditing::MatColor),
                (CurrentlyEditing::MatName, false) => match self.mat_type_input {
                    Some(MaterialType::Dielectric) | Some(MaterialType::Metal) => {
                        Some(CurrentlyEditing::MatProperty)
                    }
                    _ => Some(CurrentlyEditing::MatColor),
                },

                (CurrentlyEditing::Width, false) => Some(CurrentlyEditing::Aperture),
                (CurrentlyEditing::Height, false) => Some(CurrentlyEditing::Width),
                (CurrentlyEditing::ImgName, false) => Some(CurrentlyEditing::Height),
                (CurrentlyEditing::Samples, false) => Some(CurrentlyEditing::ImgName),
                (CurrentlyEditing::Bounces, false) => Some(CurrentlyEditing::Samples),
                (CurrentlyEditing::CamX, false) => Some(CurrentlyEditing::Bounces),
                (CurrentlyEditing::CamY, false) => Some(CurrentlyEditing::CamX),
                (CurrentlyEditing::CamZ, false) => Some(CurrentlyEditing::CamY),
                (CurrentlyEditing::LookX, false) => Some(CurrentlyEditing::CamZ),
                (CurrentlyEditing::LookY, false) => Some(CurrentlyEditing::LookX),
                (CurrentlyEditing::LookZ, false) => Some(CurrentlyEditing::LookY),
                (CurrentlyEditing::Fov, false) => Some(CurrentlyEditing::LookZ),
                (CurrentlyEditing::FocusDist, false) => Some(CurrentlyEditing::Fov),
                (CurrentlyEditing::Aperture, false) => Some(CurrentlyEditing::FocusDist),

                _ => None,
            }
        } else {
            self.current_edit = match self.current_screen {
                CurrentScreen::MaterialEditor => Some(CurrentlyEditing::MatColor),
                _ => Some(CurrentlyEditing::Size),
            }
        }
    }
}
