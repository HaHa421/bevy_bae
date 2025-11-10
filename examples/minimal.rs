use bevy::prelude::*;
use bevy_bae::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, MeshPickingPlugin, BaePlugin::default()))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Plan::new(),
        Sequence,
        tasks![Operator::new(greet), Operator::new(idle),],
    ));
}

fn greet(_: In<OperatorInput>) -> TaskStatus {
    info!("Oh hai!!! I greet you every second. Very polite, eh?");
    TaskStatus::Success
}

fn idle(_: In<OperatorInput>, time: Res<Time>, mut timer: Local<Option<Timer>>) -> TaskStatus {
    let timer = timer.get_or_insert_with(|| Timer::from_seconds(1.0, TimerMode::Once));
    timer.tick(time.delta());
    if timer.is_finished() {
        timer.reset();
        TaskStatus::Success
    } else {
        TaskStatus::Continue
    }
}
