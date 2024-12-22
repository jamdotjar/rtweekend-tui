use std::{collections::HashMap, rc::Rc};

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
    Confirmation,
    Render,
}

pub enum CurrentlyEditing {
    Size,
    Position,
    Material,
    MatType,
    MatColor,
    MatProperty,
}
pub enum MaterialType {
    Lambertian,
    Metal,
    Dielectric,
    Normal,
}

pub struct App {
    pub current_screen: CurrentScreen,
    pub current_edit: Option<CurrentlyEditing>,
    pub world: HittableList,
    pub materials: Vec<Rc<dyn Material>>,
    pub material_input: usize,
    pub size_input: f64,
    pub position_input: Point3,
    pub mat_type_input: Option<MaterialType>,
    pub mat_color_input: Color,
    pub mat_other_input: f64,
}

impl App {
    pub fn new() -> App {
        App {
            current_screen: CurrentScreen::Main,
            current_edit: None,
            world: HittableList {
                objects: Vec::new(),
            },
            materials: vec![Rc::new(Lambertian::new(Color::from(0.8)))],
            material_input: 0,
            size_input: 1.,
            position_input: Point3::from(0.),
            mat_type_input: None,
            mat_color_input: Color::from(0.8),
            mat_other_input: 1.,
        }
    }
    pub fn save_material(&mut self) {
        let mat: Rc<dyn Material> = match &self.mat_type_input {
            Some(x) => match x {
                MaterialType::Lambertian => Rc::new(Lambertian::new(self.mat_color_input)),
                MaterialType::Metal => {
                    Rc::new(Metal::new(self.mat_color_input, self.mat_other_input))
                }
                MaterialType::Normal => Rc::new(Normal::new()),
                MaterialType::Dielectric => Rc::new(Dielectric::new(self.mat_other_input)),
            },
            None => return,
        };
        self.materials.push(mat);
        self.mat_color_input = Color::from(0.8);
        self.mat_type_input = None;
        self.mat_other_input = 1.;
    }

    pub fn save_object(&mut self) {
        let mat: Rc<dyn Material> = self.materials[self.material_input].clone();
        let sphere = Sphere::new(self.position_input, self.size_input, mat.clone());
        self.world.add(sphere);
        self.material_input = 0;
        self.position_input = Vec3::from(0.);
        self.size_input = 1.;
    }

    pub fn change_editing(&mut self) {
          if let Some(edit_mode) = &self.current_edit {
            self.current_edit = match edit_mode {
                CurrentlyEditing::Size => Some(CurrentlyEditing::Position),
                CurrentlyEditing::Position => Some(CurrentlyEditing::Material),
                CurrentlyEditing::Material => Some(CurrentlyEditing::Size),

                CurrentlyEditing::MatType => Some(CurrentlyEditing::MatColor),
                CurrentlyEditing::MatColor => Some(CurrentlyEditing::MatProperty),
                CurrentlyEditing::MatProperty => Some(CurrentlyEditing::MatType),
            }
        }
        else {
            self.current_edit = match self.current_screen {
                CurrentScreen::MaterialEditor=> Some(CurrentlyEditing::MatColor),
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
}
