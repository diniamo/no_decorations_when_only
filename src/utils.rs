use hyprland::{
    data::{Workspace, WorkspaceRules, WorkspaceRuleset, Workspaces},
    shared::HyprData,
};

#[inline]
pub fn get_workspace(name: &str) -> Option<Workspace> {
    // HACK: this should not be able to fail
    Workspaces::get()
        .ok()
        .and_then(|ws| ws.into_iter().find(|w| w.name == name))
}

#[inline]
pub fn get_ruleset_from_workspace(
    workspace_rules: &WorkspaceRules,
    workspace: &Workspace,
) -> WorkspaceRuleset {
    let workspace_string = workspace.id.to_string();
    workspace_rules
        .iter()
        .find(|r| r.workspace_string == workspace_string)
        .cloned()
        .unwrap_or(WorkspaceRuleset {
            workspace_string,
            monitor: None,
            default: None,
            gaps_in: None,
            border: None,
            shadow: None,
            gaps_out: None,
            rounding: None,
            decorate: None,
            persistent: None,
            border_size: None,
        })
}

macro_rules! format_rules {
    ($vector:tt, [$(($final_name:expr, $value:expr)),+ $(,)?]) => {
        $(
            if $value.is_some() {
                $vector.push(format!("{}:{}", $final_name, $value.unwrap()));
            }
        )*
    };
}

pub fn format_for_command(ruleset: &WorkspaceRuleset) -> String {
    let mut vector = Vec::new();

    format_rules!(
        vector,
        [
            ("monitor", ruleset.monitor.as_ref()),
            ("default", ruleset.default),
            ("gapsin", ruleset.gaps_in.clone().map(|s| s[0])),
            ("gapsout", ruleset.gaps_out.clone().map(|s| s[0])),
            ("bordersize", ruleset.border_size),
            ("border", ruleset.border),
            ("shadow", ruleset.shadow),
            ("rounding", ruleset.rounding),
            ("decorate", ruleset.decorate),
            ("persistent", ruleset.persistent),
        ]
    );

    vector.join(",")
}
