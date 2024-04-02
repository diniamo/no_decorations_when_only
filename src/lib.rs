use hyprland::{
    data::{Workspace, WorkspaceRules, WorkspaceRuleset, Workspaces},
    shared::HyprData,
};

#[inline]
pub fn get_workspace(name: &str) -> Option<Workspace> {
    Workspaces::get()
        .unwrap()
        .into_iter()
        .find(|w| w.name == name)
}

#[inline]
pub fn get_ruleset_from_workspace<'a>(
    workspace_rules: &'a WorkspaceRules,
    workspace: &Workspace,
) -> Option<&'a WorkspaceRuleset> {
    workspace_rules
        .iter()
        .find(|r| r.workspace_string == workspace.id.to_string())
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
