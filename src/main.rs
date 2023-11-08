use hyprland::data::{Workspaces, Workspace};
use hyprland::event_listener::EventListenerMutable as EventListener;
use hyprland::prelude::*;
use hyprland::Result;
use serde::Serialize;
use serde_json::json;

#[derive(Debug, Serialize)]
struct WorkspaceCustom {
    pub id: i32,
    pub name: String,
    pub active: bool,
    pub windows: u16,
}

const ICONS: [&str;8] = ["󰑊 ", "󰑊 ", "󰑊 ", "󰑊 ", "󰑊 ", "󰑊 ", "󰑊 ", "󰑊 "];

fn output()  {
    let workspaces: Vec<_> = Workspaces::get().expect("Unable to get workspaces").to_vec();
    let active_workspace_id = Workspace::get_active().expect("Unable to get active workspace").id;
    let mut out_workspaces: Vec<WorkspaceCustom> = Vec::new();

    for i in 1..=ICONS.len() {
        let ws: WorkspaceCustom = WorkspaceCustom {
            id: i as i32,
            name: ICONS[i-1].to_string(),
            active: false,
            windows: 0,
        };
        out_workspaces.push(ws);
    }

    for workspace in workspaces.iter() {
        out_workspaces[workspace.id as usize - 1].windows = workspace.windows;
        if workspace.windows > 0 {
            out_workspaces[workspace.id as usize - 1].name = "󰊠 ".to_string();
        }
        if active_workspace_id == workspace.id {
            out_workspaces[workspace.id as usize - 1].name = "󰮯 ".to_string();
            out_workspaces[workspace.id as usize - 1].active = true;
        }
    }

    println!("{}", json!(out_workspaces).to_string());
}

fn main() -> Result<()> {
    output();
    let mut event_listener = EventListener::new();
    event_listener.add_workspace_change_handler(|_, _| {
        output();
    });
    event_listener.add_workspace_destroy_handler(|_, _| {
        output();
    });

    event_listener.add_window_close_handler(|_, _| {
        output();
    });
    event_listener.add_window_open_handler(|_, _| {
        output();
    });

    event_listener.start_listener()
}
