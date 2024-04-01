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

macro_rules! format_rule {
    ($vector:expr, $final_name:expr, $value:expr) => {{
        if $value.is_some() {
            $vector.push(format!("{}:{}", $final_name, $value.unwrap()));
        }
    }};
}

pub fn format_for_command(ruleset: &WorkspaceRuleset) -> String {
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
