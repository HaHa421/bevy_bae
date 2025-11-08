use bevy::prelude::*;
use bevy_bae::prelude::*;

fn main() {
    let _behavior = trunk_thumper_domain();
}

fn trunk_thumper_domain() -> impl Bundle {
    (
        Name::new("Be Trunk Thumper"),
        tasks!(Select[
            (
                Name::new("Beat up enemy"),
                conditions![(
                    Name::new("Can see enemy"),
                    Condition::eq("can_see_enemy", true),
                )],
                tasks!(Sequence[
                    (
                        Name::new("Navigate to enemy"),
                        effects![(
                            Name::new("Set location to enemy position"),
                            Effect::set("location", "enemy"),
                        )],
                        Operator::new(navigate_to_enemy),
                    ),
                    (Name::new("Do trunk slam"), Operator::new(do_trunk_slam)),
                ]),
            ),
            (
                Name::new("Patrol bridges"),
                conditions![(Name::new("Always true"), Condition::always_true())],
                tasks!(Sequence[
                    (
                        Name::new("Choose best bridge to check for enemies"),
                        Operator::new(choose_bridge_to_check),
                    ),
                    (
                        Name::new("Go to bridge"),
                        effects![(
                            Name::new("Set location to bridge"),
                            Effect::set("location", "bridge"),
                        )],
                        Operator::new(navigate_to_bridge),
                    ),
                    (
                        Name::new("Check if anything is out of the ordinary"),
                        Operator::new(check_bridge),
                    ),
                ]),
            )
        ]),
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
