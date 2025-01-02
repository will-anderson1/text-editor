use tauri::menu::{MenuItem, Menu};
#[tauri::command]
fn my_custom_command() {
  println!("I was invoked from JavaScript!")
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .menu(|handle| {
            let mut menu = Menu::default(&handle).unwrap();
            // menu.get("File").unwrap().add_item(MenuItem::with_id(handle, "preferences-item", "Preferences", true, Some("CommandOrControl+,")));
            // if let Some(file_menu) = menu.get("File") {
            //     if let MenuEntry::Submenu(submenu) = file_menu {
            //         // Append the Open File item to the File submenu
            //         submenu.menu.add_item(MenuItem::with_id(handle, "preferences-item", "Preferences", true, Some("CommandOrControl+,")));
            //     }
            // }
            let _ = menu.get("18")
                .unwrap()
                .as_submenu()
                .unwrap()
                .append(
                    &MenuItem::with_id(handle, "open-file", "Open", true, Some("CommandOrControl+N")).unwrap()
                );
            
            // print the names of the menu items
            menu.items().into_iter().for_each(|item| {
                // println!("{}", item.id());
                item.into_iter().for_each(|subitem| {
                    println!("{}", subitem.id().0);
                });
            });
            // menu.append(
            //     &Submenu::with_items(
            //         handle,
            //         "Edit",
            //         true,
            //         &[
            //             &MenuItem::with_id(handle, "preferences-item", "Preferences", true, Some("CommandOrControl+,"))
            //                 .unwrap(),
            //         ],
            //     ).unwrap()
            // );

            Ok(menu)
        })
        .setup(|app| {
            app.on_menu_event(|_, event| {
                println!("{:?}", event)
            });

            Ok(())
        })
        // .setup(|app| {
        //     app.on_menu_event(move |app, event| {
        //         if event.id() == check.id() {
        //             println!("`check` triggered, do something! is checked? {}", check.is_checked().unwrap());
        //         } else if event.id() == "toggle" {
        //             println!("toggle triggered!");
        //         }
        //     });
        //     Ok(())
        // })
        .invoke_handler(tauri::generate_handler![my_custom_command])
        .run(tauri::generate_context!())
        .expect("error while running tauri application")
}