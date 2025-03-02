#![windows_subsystem = "windows"]

use druid::{
    AppLauncher, Color, Command, Data, Env, Lens, Selector, Widget, WidgetExt, WindowDesc,
};
use druid::widget::{Button, Flex, Label, List, Scroll, TextBox};
use rusqlite::{params, Connection, Result};
use std::sync::Arc;

const DELETE_TASK: Selector<u64> = Selector::new("delete-task");

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

fn init_db() -> Result<Connection> {
    let conn = Connection::open("tasks.db")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS tasks (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            description TEXT NOT NULL,
            status TEXT NOT NULL
        )",
        [],
    )?;
    Ok(conn)
}

fn load_tasks(conn: &Connection) -> Vec<Task> {
    let mut stmt = conn.prepare("SELECT id, description, status FROM tasks").unwrap();
    let tasks_iter = stmt
        .query_map([], |row| {
            Ok(Task {
                id: row.get(0)?,
                description: row.get(1)?,
                status: row.get(2)?,
            })
        })
        .unwrap();

    tasks_iter.filter_map(Result::ok).collect()
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
            let new_id = {
                let conn = init_db().unwrap();
                conn.execute(
                    "INSERT INTO tasks (description, status) VALUES (?1, ?2)",
                    params![&data.new_task, &data.new_status],
                ).unwrap();
                conn.last_insert_rowid() as u64
            };

            let tasks = Arc::make_mut(&mut data.tasks);
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
        Flex::column()
            .with_child(
                Flex::row()
                    .with_flex_child(
                        Label::new(|item: &Task, _env: &_| item.description.clone())
                            .with_line_break_mode(druid::widget::LineBreaking::WordWrap)
                            .expand_width(),
                        1.0,
                    )
                    .with_spacer(10.0)
                    .with_child(
                        Label::new(|item: &Task, _env: &_| format!("Statut: {}", item.status))
                            .fix_width(100.0),
                    )
                    .with_spacer(10.0)
                    .with_child(Button::new("Supprimer").on_click(|ctx, task: &mut Task, _env| {
                        ctx.submit_command(Command::new(DELETE_TASK, task.id, druid::Target::Auto));
                    })),
            )
            .with_spacer(10.0)
    })
        .lens(TodoState::tasks);

    Flex::column()
        .with_child(input)
        .with_spacer(20.0)
        .with_child(status_input)
        .with_spacer(20.0)
        .with_child(add_task_button)
        .with_spacer(20.0)
        .with_flex_child(Scroll::new(task_list).vertical().expand_height(), 1.0)
        .padding(20.0)
        .background(Color::rgb8(21, 22, 22))
}

fn main() {
    let conn = init_db().expect("Impossible d'initialiser la base de données");
    let initial_state = TodoState {
        tasks: Arc::new(load_tasks(&conn)),
        new_task: String::new(),
        new_status: String::new(),
    };

    let main_window = WindowDesc::new(build_ui())
        .title("Todo List")
        .window_size((500.0, 600.0));

    AppLauncher::with_window(main_window)
        .delegate(Delegate {})
        .launch(initial_state)
        .expect("Erreur au lancement de l'application");
}

struct Delegate;

impl druid::AppDelegate<TodoState> for Delegate {
    fn command(
        &mut self,
        _ctx: &mut druid::DelegateCtx,
        _target: druid::Target,
        cmd: &druid::Command,
        data: &mut TodoState,
        _env: &Env,
    ) -> druid::Handled {
        if let Some(task_id) = cmd.get(DELETE_TASK) {
            let conn = init_db().unwrap();
            conn.execute("DELETE FROM tasks WHERE id = ?1", params![task_id]).unwrap();

            let tasks = Arc::make_mut(&mut data.tasks);
            tasks.retain(|task| task.id != *task_id);
            return druid::Handled::Yes;
        }
        druid::Handled::No
    }
}
