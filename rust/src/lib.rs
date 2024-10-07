use godot::{
    classes::{Control, IControl},
    prelude::*,
    tools,
};

struct Ext;

#[gdextension]
unsafe impl ExtensionLibrary for Ext {}

#[derive(GodotClass)]
#[class(base=Control)]
pub struct MyControl {
    base: Base<Control>,
}

#[godot_api]
impl IControl for MyControl {
    fn init(base: Base<Self::Base>) -> Self {
        let mf = tools::try_load::<MyResource>("res://MyResource.tres");
        mf.unwrap().bind();
        Self { base }
    }
}

#[derive(GodotClass)]
#[class(base=Resource, init)]
pub struct MyResource;
