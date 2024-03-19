slint::include_modules!();

fn main() {
    use slint::Model;

    let main_window = ChecklistApp::new().unwrap();

    //let mut vec_checklist: Vec<ChecklistInfo> = main_window.get_arr_checklist().iter().collect();

    let main_window_weak = main_window.as_weak().unwrap();
    main_window.on_add_checklist(move || {
        let mut vec_checklist: Vec<ChecklistInfo> = main_window_weak.get_arr_checklist().iter().collect();
        vec_checklist.push(ChecklistInfo::clone(&ChecklistInfo {str_title: "Checklist".into(),}));

        let checklist_model = std::rc::Rc::new(slint::VecModel::from(vec_checklist));
        main_window_weak.set_arr_checklist(checklist_model.into());
    });

    let main_window_weak = main_window.as_weak().unwrap();
    main_window.on_remove_all_checklist(move || {
        let mut vec_checklist: Vec<ChecklistInfo> = Vec::new();

        let checklist_model = std::rc::Rc::new(slint::VecModel::from(vec_checklist));
        main_window_weak.set_arr_checklist(checklist_model.into());
    });

    let main_window_weak = main_window.as_weak().unwrap();
    main_window.on_add_checkable_item(move || {
        let mut vec_checkable_item: Vec<CheckableItemInfo> = main_window_weak.get_arr_checkable_item().iter().collect();

        vec_checkable_item.push(CheckableItemInfo::clone(&CheckableItemInfo {}));

        let checklist_model = std::rc::Rc::new(slint::VecModel::from(vec_checkable_item));
        main_window_weak.set_arr_checkable_item(checklist_model.into());
    });

    /*main_window.invoke_add_checklist();
    main_window.invoke_add_checklist();*/

    main_window.run().unwrap();
}
