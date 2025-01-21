use druid::{AppLauncher, Color, Data, Lens, Widget, WidgetExt, WindowDesc};
use druid::widget::{Button, Flex, Label, List, TextBox};
use std::sync::Arc;

#[derive(Clone, Data, Lens)]
struct TodoState {
    tasks: Arc<Vec<Task>>,
    new_task: String,
    new_status: String,
}

#[derive(Clone, Data, Lens)]
struct Task {
    id: u64,
    description: String,
    status: String,
}

fn build_ui() -> impl Widget<TodoState> {
    let input = TextBox::new()
        .with_placeholder("Ajouter une tâche...")
        .lens(TodoState::new_task)
        .fix_width(300.0);

    let status_input = TextBox::new()
        .with_placeholder("Statut (ex: À faire)")
        .lens(TodoState::new_status)
        .fix_width(200.0);

    let add_task_button = Button::new("Ajouter").on_click(|_ctx, data: &mut TodoState, _env| {
        if !data.new_task.is_empty() && !data.new_status.is_empty() {
            let new_id = data.tasks.iter().map(|task| task.id).max().unwrap_or(0) + 1;
            let mut tasks = Arc::make_mut(&mut data.tasks);
            tasks.push(Task {
                id: new_id,
                description: data.new_task.clone(),
                status: data.new_status.clone(),
            });
            data.new_task.clear();
            data.new_status.clear();
        }
    });

    let task_list = List::new(|| {
        Flex::row()
            .with_child(Label::new(|item: &Task, _env: &_| item.description.clone()).fix_width(200.0))
            .with_child(Label::new(|item: &Task, _env: &_| format!("Statut: {}", item.status)).fix_width(100.0))
            .with_child(Button::new("Supprimer").on_click(|_ctx, task: &mut Task, _env| {
                println!("Supprimer tâche : {:?}", task.id);
            }))
    })
        .lens(TodoState::tasks);

    Flex::column()
        .with_child(input)
        .with_child(status_input)
        .with_child(add_task_button)
        .with_flex_child(task_list, 1.0)
        .padding(20.0)
        .background(Color::rgb8(10, 140, 90))
}

fn main() {
    let main_window = WindowDesc::new(build_ui())
        .title("Todo List")
        .window_size((500.0, 600.0));
    let initial_state = TodoState {
        tasks: Arc::new(Vec::new()),
        new_task: String::new(),
        new_status: String::new(),
    };
    AppLauncher::with_window(main_window)
        .launch(initial_state)
        .expect("Erreur au lancement de l'application");
}
