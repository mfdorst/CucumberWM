use std::process::Command;
use std::thread;
use umberwm::{
    umberwm, Actions, Conf, CustomAction, DisplayBorder, EventsCallbacks, Keybind, WindowBorder,
    MOD_MASK_1, MOD_MASK_CONTROL, MOD_MASK_SHIFT,
};

fn main() {
    let meta = MOD_MASK_1;

    let display_borders = vec![
        DisplayBorder {
            left: 0,
            right: 0,
            top: 0,
            bottom: 0,
            gap: 10,
        },
        DisplayBorder {
            left: 0,
            right: 0,
            top: 0,
            bottom: 0,
            gap: 0,
        },
    ];

    let border = WindowBorder {
        width: 1,
        focus_color: 0x906cff,
        normal_color: 0x000000,
    };

    let workspaces_names = vec![(1..=9).map(|i| i.to_string()).collect()];

    let custom_actions = vec![
        (
            Keybind::new(meta, "space"),
            Box::new(|| {
                thread::spawn(move || {
                    let _ = Command::new("rofi").arg("-show").arg("run").status();
                });
            }) as CustomAction,
        ),
        (
            Keybind::new(meta | MOD_MASK_SHIFT, "Return"),
            Box::new(|| {
                thread::spawn(move || {
                    let _ = Command::new("alacritty").status();
                });
            }),
        ),
        (
            Keybind::new(meta | MOD_MASK_CONTROL, "q"),
            Box::new(|| std::process::exit(0)),
        ),
    ]
    .into_iter()
    .collect();

    let wm_actions = vec![
        (
            Keybind::new(meta | MOD_MASK_SHIFT, "c"),
            Actions::CloseWindow,
        ),
        (
            Keybind::new(meta | MOD_MASK_SHIFT, "m"),
            Actions::ChangeLayout,
        ),
        (Keybind::new(meta | MOD_MASK_SHIFT, "g"), Actions::ToggleGap),
        (
            Keybind::new(meta | MOD_MASK_CONTROL, "r"),
            Actions::SerializeAndQuit,
        ),
    ]
    .into_iter()
    .collect();

    // Won't tile windows with WM_CLASS's in this list
    let ignore_classes = vec!["xscreensaver", "Discover-overlay"]
        .into_iter()
        .map(|x| x.to_string())
        .collect();

    let float_classes = vec![
        "confirm",
        "dialog",
        "error",
        "splash",
        "toolbar",
        "screenkey",
        "audacious",
        "Download",
        "dropbox",
        "file_progress",
        "file-roller",
        "Komodo_confirm_repl",
        "Komodo_find2",
        "pidgin",
        "skype",
        "Transmission",
        "Update",
        "Xephyr",
        "obs",
        "rofi",
        "xscreensaver",
        "quickmarks",
        "discover-overlay",
        "Discover-overlay",
    ]
    .into_iter()
    .map(|x| x.to_string())
    .collect();

    let events_callbacks = EventsCallbacks {
        on_change_workspace: None,
    };

    let with_gap = true;

    umberwm(Conf {
        meta,
        display_borders,
        border,
        workspaces_names,
        custom_actions,
        wm_actions,
        ignore_classes,
        float_classes,
        events_callbacks,
        with_gap,
    })
    .run();
}
