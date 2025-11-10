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
                Name::new("Fight enemy"),
                tasks!(Sequence[
                    (
                        conditions![
                            Condition::eq("can_see_enemy", true),
                        ],
                        Operator::new(navigate_to_enemy),
                        effects![(
                            Effect::set("location", "enemy"),
                        )],
                    ),
                    Operator::new(do_trunk_slam),
                ]),
            ),
            (
                Name::new("Patrol bridges"),
                tasks!(Sequence[
                    Operator::new(choose_bridge_to_check),
                    (
                        Operator::new(navigate_to_bridge),
                        effects![
                            Effect::set("location", "bridge"),
                        ],
                    ),
                    Operator::new(check_bridge),
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
