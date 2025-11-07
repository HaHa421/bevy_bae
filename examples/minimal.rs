use bevy::prelude::*;
use bevy_bae::prelude::*;

fn main() {
    let _behavior = trunk_thumper_domain();
}

fn trunk_thumper_domain() -> impl Bundle {
    (
        Name::new("Be Trunk Thumper"),
        Select,
        tasks![
            (
                Sequence,
                Name::new("Beat up enemy"),
                conditions![(
                    Condition::eq("can_see_enemy", true),
                    Name::new("Can see enemy"),
                )],
                tasks![
                    (
                        Operator::new(navigate_to_enemy),
                        Name::new("Navigate to enemy"),
                        effects![(
                            Effect::set("location", "enemy"),
                            Name::new("Set location to enemy position"),
                        )],
                    ),
                    (Operator::new(do_trunk_slam), Name::new("Do trunk slam")),
                ],
            ),
            (
                Sequence,
                Name::new("Patrol bridges"),
                conditions![(Condition::always_true(), Name::new("Always true"))],
                tasks![
                    (
                        Operator::new(choose_bridge_to_check),
                        Name::new("Choose best bridge to check for enemies"),
                    ),
                    (
                        Operator::new(navigate_to_bridge),
                        Name::new("Go to bridge"),
                        effects![(
                            Effect::set("location", "bridge"),
                            Name::new("Set location to bridge"),
                        )],
                    ),
                    (
                        Operator::new(check_bridge),
                        Name::new("Check if anything is out of the ordinary"),
                    ),
                ],
            )
        ],
    )
}

fn navigate_to_enemy(_step: In<OperatorInput>) -> TaskStatus {
    info!("navigating to enemy");
    TaskStatus::Success
}

fn do_trunk_slam(_step: In<OperatorInput>) -> TaskStatus {
    info!("trunk slam");
    TaskStatus::Success
}

fn choose_bridge_to_check(_step: In<OperatorInput>) -> TaskStatus {
    info!("choosing bridge to check");
    TaskStatus::Success
}

fn navigate_to_bridge(_step: In<OperatorInput>) -> TaskStatus {
    info!("navigating to bridge");
    TaskStatus::Success
}

fn check_bridge(_step: In<OperatorInput>) -> TaskStatus {
    info!("checking bridge");
    TaskStatus::Success
}
