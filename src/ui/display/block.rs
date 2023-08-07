use crate::app::app::App;

// block binds when a popup is shown
pub fn block_binds(app: &mut App) -> bool {
    if app.show_nav
        || app.show_fzf
        || app.show_help
        || app.show_popup
        || app.show_bookmark
        || app.show_ops_menu
    {
        return true;
    }

    false
}
