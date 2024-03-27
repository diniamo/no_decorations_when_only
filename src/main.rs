use std::{process, rc::Rc};

use single_instance::SingleInstance;
use hyprland::{ctl, data::{Workspace, WorkspaceRules, WorkspaceRuleset, Workspaces}, event_listener::EventListener, keyword::Keyword, shared::{HyprData, HyprDataActive, WorkspaceType}};

#[inline]
fn get_ruleset_from_workspace<'a>(workspace_rules: &'a WorkspaceRules, workspace: &Workspace) -> &'a WorkspaceRuleset {
    workspace_rules.iter().find(|r| r.workspace_string == workspace.name).unwrap()
}

fn update_window_decorations(workspace: &Workspace, workspace_rules: &WorkspaceRules) {
    if workspace.fullscreen || workspace.windows == 1 {
        let ruleset = get_ruleset_from_workspace(workspace_rules, workspace);

        Keyword::set("workspace", format!("{},{}", workspace.id, format_for_command(&WorkspaceRuleset {
            gaps_in: Some(vec![0, 0, 0, 0]),
            gaps_out: Some(vec![0, 0, 0, 0]),
            border: Some(false),
            rounding: Some(false),
            ..(ruleset.clone())
        }))).expect("Failed to set keyword");
    } else if workspace.windows > 1 {
        let ruleset = get_ruleset_from_workspace(workspace_rules, workspace);
        Keyword::set("workspace", format!("{},{}", workspace.id, format_for_command(ruleset))).expect("Failed to set keyword");
    }
}

macro_rules! format_rule {
    ($vector: expr, $final_name: expr, $value: expr) => {
        {
            if $value.is_some() {
                $vector.push(format!("{}:{}", $final_name, $value.unwrap()));
            }
        }
    };
}

fn format_for_command(ruleset: &WorkspaceRuleset) -> String {
    let mut vector = Vec::new();

    format_rule!(vector, "monitor", ruleset.monitor.as_ref());
    format_rule!(vector, "default", ruleset.default);
    format_rule!(vector, "gapsin", ruleset.gaps_in.clone().map(|s| s[0]));
    format_rule!(vector, "gapsout", ruleset.gaps_out.clone().map(|s| s[0]));
    format_rule!(vector, "bordersize", ruleset.border_size);
    format_rule!(vector, "border", ruleset.border);
    format_rule!(vector, "shadow", ruleset.shadow);
    format_rule!(vector, "rounding", ruleset.rounding);
    format_rule!(vector, "decorate", ruleset.decorate);
    format_rule!(vector, "persistent", ruleset.persistent);

    vector.join(",")
}

fn get_workspace(name: &str) -> Option<Workspace> {
    println!("Getting worrkspace {name}");
    Workspaces::get()
        .expect("Failed to get workspaces")
        .into_iter()
        .find(|w| { println!("Comparing {}", w.name); w.name == name })
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
        eprintln!("There is already another instance running! Close it before trying to run a new one");
        process::exit(1);
    }

    // To reset changes by a potenetial previous instance
    ctl::reload::call().expect("Failed to reload Hyprland");

    let workspace_rules = Rc::new(WorkspaceRules::get().expect("Failed to get workspacerules"));

    let mut listener = EventListener::new();

    listener.add_window_open_handler(enclose! { (workspace_rules) move |e| {
        if !e.workspace_name.starts_with("special:") {
            update_window_decorations(&get_workspace(&e.workspace_name).unwrap(), &workspace_rules)
        }
    } });
    // TODO: windows can also close on other workspaces
    listener.add_window_close_handler(enclose! { (workspace_rules) move |_| update_window_decorations(&Workspace::get_active().expect("Failed to get active workspace"), &workspace_rules) });
    listener.add_window_moved_handler(enclose! { (workspace_rules) move |e| update_window_decorations(&get_workspace(&e.workspace_name).unwrap(), &workspace_rules) });
    // TODO: windows can also close on other workspaces
    listener.add_float_state_handler(enclose ! { (workspace_rules) move |_| update_window_decorations(&Workspace::get_active().expect("Failed to get active workspace"), &workspace_rules) });
    // HACK: remove this once the other TODOs are done
    listener.add_workspace_change_handler(enclose! { (workspace_rules) move |t| {
        if let WorkspaceType::Regular(name) = t {
            update_window_decorations(&get_workspace(&name).unwrap(), &workspace_rules)
        }
    } });
    listener.add_fullscreen_state_change_handler(enclose! { (workspace_rules) move |_| update_window_decorations(&Workspace::get_active().expect("Failed to get active workspace"), &workspace_rules) });

    listener.start_listener().expect("Couldn't start listener");
}
