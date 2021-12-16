use std::ops::Deref;
use rhai::{AST, Engine, EvalAltResult, Scope};

#[cfg(feature = "events")]
use crate::{Event, EventHandler, PluginFunctionArgs};

pub struct PluginConfig {}

pub struct Plugin<'a> {
    pub engine: Engine,
    ast: AST,
    scope: Scope<'a>
}

impl Plugin<'_> {
    pub fn get_event_names(&self) -> Vec<&str> {
        self.ast.iter_functions()
            .map(|f| f.name.split_at(3).1)
            .collect()
    }
}

impl TryFrom<&str> for Plugin<'_> {
    type Error = Box<EvalAltResult>;

    fn try_from(script: &str) -> Result<Self, Self::Error> {
        let engine = Engine::new_raw();
        let ast = engine.compile(script)?.clone_functions_only();

        let mut scope = Scope::new();

        engine.call_fn_raw(
            &mut scope,
            &ast,
            false,
            false,
            "init",
            None,
            [],
        )?;

        Ok(Self {
            engine,
            ast,
            scope
        })
    }
}

#[cfg(feature = "events")]
impl<E: Into<PluginFunctionArgs> + Clone> EventHandler<E> for &Plugin<'_> {
    fn handle_event(&self, event: &Event<E>) {
        let args: PluginFunctionArgs = event.deref().clone().into();
        let fn_name = format!("on_{}", event.name);

        for function in self.ast.iter_functions() {
            if function.name == fn_name {
                let bound_args = args.bind_for(&function.params);
                self.engine.call_fn_raw(
                    &mut self.scope.clone(),
                    &self.ast,
                    true,
                    false,
                    function.name,
                    None,
                    bound_args
                ).unwrap();
            }
        }
    }
}

#[cfg(feature = "events")]
impl<E: Into<PluginFunctionArgs> + Clone> EventHandler<E> for Plugin<'_> {
    fn handle_event(&self, event: &Event<E>) {
        (&self).handle_event(event)
    }
}