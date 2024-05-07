use std::{cell::RefCell, collections::HashMap, process, rc::Rc};

use hyprland::{
    ctl,
    data::{Monitors, Workspace, WorkspaceRules, WorkspaceRuleset},
    event_listener::EventListener,
    keyword::Keyword,
    shared::{HyprData, HyprDataActive, WorkspaceType},
};
use single_instance::SingleInstance;

mod utils;

struct State {
    initial_rules: WorkspaceRules,
    toggle_cache: HashMap<i32, bool>,
}

impl State {
    fn new() -> Self {
        Self {
            initial_rules: WorkspaceRules::get().unwrap(),
            toggle_cache: HashMap::new(),
        }
    }

    fn update_window_decorations(&mut self, workspace: &Workspace) {
        if workspace.windows == 0 {
            return;
        }

        let new_state = workspace.fullscreen || workspace.windows == 1;
        if new_state
            == self
                .toggle_cache
                .get(&workspace.id)
                .copied()
                .unwrap_or(false)
        {
            println!("Returning for workspace {}", workspace.name);
            return;
        }

        if new_state {
            let ruleset = utils::get_ruleset_from_workspace(&self.initial_rules, workspace);

            Keyword::set(
                "workspace",
                format!(
                    "{},{}",
                    workspace.id,
                    utils::format_for_command(&WorkspaceRuleset {
                        gaps_in: Some(vec![0, 0, 0, 0]),
                        gaps_out: Some(vec![0, 0, 0, 0]),
                        border: Some(false),
                        rounding: Some(false),
                        ..ruleset
                    })
                ),
            )
            .unwrap();
        } else {
            let ruleset = utils::get_ruleset_from_workspace(&self.initial_rules, workspace);
            println!(
                "Sending `{}`",
                format!("{},{}", workspace.id, utils::format_for_command(&ruleset))
            );

            Keyword::set(
                "workspace",
                format!("{},{}", workspace.id, utils::format_for_command(&ruleset)),
            )
            .unwrap();
        }

        self.toggle_cache.insert(workspace.id, new_state);
    }

    #[inline]
    fn update_active_workspaces(&mut self) {
        Monitors::get()
            .unwrap()
            .into_iter()
            .filter_map(|m| utils::get_workspace(&m.active_workspace.name))
            .for_each(|w| self.update_window_decorations(&w));
    }

    #[inline]
    fn update_active_workspace(&mut self) {
        self.update_window_decorations(&Workspace::get_active().unwrap());
    }

    #[inline]
    fn reset(&mut self) {
        self.toggle_cache.clear();
        self.update_active_workspaces();
    }
}

// https://github.com/rust-lang/rfcs/issues/2407#issuecomment-385291238
macro_rules! enclose {
    ( ($( $x:ident ),*) $y:expr ) => {
        {
            $(let $x = $x.clone();)*
            $y
        }
    };
}

fn main() {
    let instance = SingleInstance::new(env!("CARGO_PKG_NAME")).unwrap();
    if !instance.is_single() {
        eprintln!(
            "There is already another instance running! Close it before trying to run a new one"
        );
        process::exit(1);
    }

    // To reset changes by a potenetial previous instance
    ctl::reload::call().unwrap();

    let state = Rc::new(RefCell::new(State::new()));
    state.borrow_mut().update_active_workspaces();

    let mut listener = EventListener::new();

    listener.add_config_reload_handler(enclose! { (state) move |_| state.borrow_mut().reset() });
    listener.add_window_close_handler(
        enclose! { (state) move |_| state.borrow_mut().update_active_workspaces() },
    );
    listener.add_window_moved_handler(
        enclose! { (state) move |_| state.borrow_mut().update_active_workspaces() },
    );
    listener.add_fullscreen_state_change_handler(
        enclose! { (state) move |_| state.borrow_mut().update_active_workspace() },
    );
    // listener.add_float_state_handler(enclose! { (workspace_rules) move |_| update_active_workspaces(&workspace_rules)});
    listener.add_active_monitor_change_handler(enclose! { (state) move |e| {
        if let WorkspaceType::Regular(name) = e.workspace {
            if let Some(workspace) = utils::get_workspace(&name) {
                state.borrow_mut().update_window_decorations(&workspace);
            }
        }
    } });
    listener.add_window_open_handler(enclose! { (state) move |e| {
        if !e.workspace_name.starts_with("special:") {
            if let Some(workspace) = utils::get_workspace(&e.workspace_name) {
                state.borrow_mut().update_window_decorations(&workspace);
            }
        }
    } });
    listener.add_workspace_change_handler(enclose! { (state) move |t| {
        if let WorkspaceType::Regular(name) = t {
            if let Some(workspace) = utils::get_workspace(&name) {
                state.borrow_mut().update_window_decorations(&workspace);
            }
        }
    } });

    listener.start_listener().unwrap();
}
