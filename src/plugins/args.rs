use std::collections::HashMap;
use rhai::Dynamic;

pub struct PluginFunctionArgs {
    args: HashMap<String, Dynamic>,
}

impl PluginFunctionArgs {
    pub fn new(args: HashMap<String, Dynamic>) -> Self {
        Self { args }
    }

    pub fn bind_for(&self, arg_names: &Vec<&str>) -> Vec<Dynamic> {
        arg_names
            .iter()
            .map(|arg| {
                let arg = self.args.get(*arg);
                match arg {
                    Some(arg) => arg.clone(),
                    None => Dynamic::from(())
                }
            })
            .collect()
    }
}