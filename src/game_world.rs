use crate::math::Vector2;

pub static mut SINGLETON: GameWorld = GameWorld {
    layers: vec![]
};

pub struct Transform {
    position: Vector2,
    rotation: Vector2,
    scale: Vector2
}

impl Default for Transform {
    fn default() -> Self {
        return Transform::new()
    }
}

impl Transform {
    pub fn new() -> Self {
        return Transform {
            position: Vector2::default(),
            rotation: Vector2::default(),
            scale: Vector2::default()
        }
    }

    pub fn translate(&mut self, offset: Vector2) {
        self.position += offset;
    }

    pub fn rotate(&mut self, vec: Vector2) {
        self.rotation += vec;
    }

    pub fn rotate_x(&mut self, angle: i32) {
        self.rotation.0 += angle;
    }

    pub fn rotate_y(&mut self, angle: i32) {
        self.rotation.1 += angle;
    }

    pub fn scale(&mut self, factor: i32) {
        self.scale *= factor;
    }

    pub fn scale_x(&mut self, factor: i32) {
        self.scale.0 *= factor;
    }

    pub fn scale_y(&mut self, factor: i32) {
        self.scale.1 *= factor;
    }

    pub fn resize(&mut self, value: Vector2) {
        self.scale += value;
    }

    pub fn resize_x(&mut self, value: i32) {
        self.scale.0 += value;
    }

    pub fn resize_y(&mut self, value: i32) {
        self.scale.1 += value;
    }
}

pub trait Component {
    fn get_name(&self) -> &str;
    fn start(&mut self);
    fn update(&mut self);
    fn is_renderable(&self) -> bool;
    fn render(&mut self);
}

pub enum GameObjectError <'a>{
    ComponentNotFound(&'a str)
}

pub struct GameObject {
    pub transform: Transform,
    components: Vec<Box<dyn Component>>
}

impl GameObject {
    pub fn new() -> Self {
        return GameObject {
            transform: Transform::new(),
            components: Vec::new()
        }
    }

    pub fn add_component(&mut self, component: Box<dyn Component>) {
        self.components.push(component);
    }

    pub fn remove_component<'a>(&mut self, name: &'a str) -> Result<(), GameObjectError<'a>> {
        for i in 0..self.components.len() - 1 {
            if self.components[i].get_name() == name {
                self.components.remove(i);
                return Ok(())
            }
        }
        return Err(GameObjectError::ComponentNotFound(name))
    }

    pub fn render(&mut self) {
        for i in self.components.iter_mut() {
            if i.is_renderable() {
                i.render()
            }
        }
    }

    pub fn start(&mut self) {
        for i in self.components.iter_mut() {
            i.start()
        }
    }

    pub fn update(&mut self) {
        for i in self.components.iter_mut() {
            i.update();
        }
    }
}

pub enum LayerError {
    ObjectNotFound(i32)
}

pub struct Layer {
    objects: Vec<GameObject>
}

impl Layer {
    pub fn new() -> Self {
        return Layer {
            objects: Vec::new()
        }
    }

    pub fn render(&mut self) {
        for i in self.objects.iter_mut() {
            i.render();
        }
    }

    pub fn update(&mut self) {
        for i in self.objects.iter_mut() {
            i.update();
        }
    }

    pub fn add_object(&mut self, object: GameObject) {
        self.objects.push(object);
    }

    pub fn remove_object(&mut self, id: i32) -> Result<(), LayerError> {
        match self.objects.get(id as usize) {
            Some(_) => {
                self.objects.remove(id as usize);
                return Ok(())
            },
            None => return Err(LayerError::ObjectNotFound(id))
        }
    }
}

pub struct GameWorld {
    layers: Vec<Layer>
}

impl GameWorld {
    pub fn start(&mut self) {}

    pub fn update(&mut self) {
        for i in self.layers.iter_mut() {
            i.update();
            i.render();
        }
    }
}