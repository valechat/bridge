use std::collections::HashMap;
use vale_bridge::{Event, EventHandler, Plugin, PluginFunctionArgs};
use rhai::packages::Package;
use rhai::{def_package, Dynamic, ImmutableString};

def_package!(rhai:CustomPackage:"My custom package stuff", module, {
    module.set_native_fn("print", |s: ImmutableString| {
        println!("{:?}", s);
        Ok(())
    });

    module.set_native_fn("test", |_: &mut Test| {
        Ok("this is cool")
    });
});

#[derive(Clone, Debug)]
struct Test {}

impl Into<PluginFunctionArgs> for Test {
    fn into(self) -> PluginFunctionArgs {
        PluginFunctionArgs::new(HashMap::from([
            ("msg".to_string(), Dynamic::from(Test {}))
        ]))
    }
}

fn main() {
    let custom_pkg = CustomPackage::new();

    let mut plugin = Plugin::try_from(r#"
        fn init() {
            let test = "cool";
        }

        fn on_message_create(msg) {
            print(msg.test());
        }
    "#).unwrap();

    plugin.engine.register_global_module(custom_pkg.as_shared_module());

    let event_name = "message_create".to_string();
    let event = Event::new(
        &event_name,
        Test {}
    );

    plugin.handle_event(&event)
}