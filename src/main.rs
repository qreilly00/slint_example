slint::include_modules!();

fn main() {
    use slint::Model;

    let main_window = ChecklistApp::new().unwrap();

    //println!("{}", main_window.container().len_root_Width);

    let mut vec_checklist: Vec<ChecklistInfo> = main_window.get_arr_checklist().iter().collect();
    vec_checklist.push(ChecklistInfo::clone(&ChecklistInfo {
        str_title: "Checklist 1".into(),
    }));

    vec_checklist.push(ChecklistInfo::clone(&ChecklistInfo {
        str_title: "Checklist 2".into(),
    }));

    let checklist_model = std::rc::Rc::new(slint::VecModel::from(vec_checklist));
    main_window.set_arr_checklist(checklist_model.into());

    main_window.run().unwrap();
}
