use std::rc::Rc;

use haby_api_wrapper::core;
use leptos::{
    component,
    create_action,
    create_local_resource,
    create_rw_signal,
    create_signal,
    create_slice,
    event_target_value,
    expect_context,
    mount_to_body,
    provide_context,
    view,
    For,
    IntoView,
    Resource,
    Show,
    Signal,
    SignalGet,
    SignalSet,
    SignalSetter,
    SignalUpdate,
    SignalWith,
    Transition,
};

fn get_client() -> Rc<haby_api_wrapper::ApiWrapper> {
    expect_context()
}

#[component]
fn DebugPrint<T>(#[prop(into)] obj: Signal<T>) -> impl IntoView
where
    T: std::fmt::Debug + 'static,
{
    view! {
        {move || obj.with(|habit| format!("{habit:?}"))}
        <br/>
    }
}

#[component]
fn TextInput(
    #[prop(into)] getter: Signal<String>,
    #[prop(into)] setter: SignalSetter<String>,
) -> impl IntoView {
    view! {
        <input type="text"
            prop:value=getter
            on:input=move |ev| {
                setter(event_target_value(&ev))
            }
        />
    }
}

#[component]
fn Overlay() -> impl IntoView {}

#[component]
fn HabitCreator(habits_resource: Resource<(), Vec<core::Habit>>) -> impl IntoView {
    let habit = create_rw_signal(core::api::CreateHabit::default());
    let insert_habit = create_action(move |habit: &core::api::CreateHabit| {
        let habit = habit.clone();
        async move {
            let result = get_client().create_habit(habit).await;
            match result {
                Ok(habit) => {
                    habits_resource.update(|habits| {
                        if let Some(habits) = habits.as_mut() {
                            habits.push(habit)
                        }
                    });
                    Ok(())
                }
                Err(err) => Err(err),
            }
        }
    });

    let (name, set_name) = create_slice(
        habit,
        |habit| habit.name.clone(),
        |habit, name| habit.name = name,
    );

    view! {
        <h2>Create New Habit</h2>

        <TextInput getter=name setter=set_name /> <br />

        <DebugPrint obj=habit/>
        <DebugPrint obj=insert_habit.value()/>
        <button on:click=move |_| insert_habit.dispatch(habit())>Create</button>
    }
}

#[component]
fn HabitList() -> impl IntoView {
    let habits = create_local_resource(
        move || (),
        |_| async move { get_client().get_habits().await },
    );
    let (show_creator, update_show_creator) = create_signal(false);

    view! {
        <button on:click=move |_| update_show_creator(true) disabled=move || show_creator>
            New
        </button>
        <Show when=show_creator>
            <HabitCreator habits_resource=habits/>
        </Show>
        <br/>
        <h1>Habit List</h1>
        <Transition fallback=move || {
            view! { "loading..." }
        }>
            <For
                    each=move || habits().unwrap_or_default()
                    key=move |habit| habit.id
                    let:data
                >
                    <DebugPrint obj=move || data.clone()/>
            </For>
        </Transition>
    }
}

#[component]
fn DebugPage() -> impl IntoView {
    let version = create_local_resource(
        move || (),
        |_| async move { get_client().get_version().await },
    );

    view! {
        <h2>
            Backend-Core version is: {move || version} <br/> Frontend-Core version is:
            {haby_api_wrapper::VERSION}
        </h2>
    }
}

#[component]
pub fn App() -> impl IntoView {
    let client = haby_api_wrapper::ApiWrapper::default();
    provide_context(Rc::new(client));

    view! { <HabitList/> }
}

pub fn launch() {
    mount_to_body(App);
}
