# Bevy BAE (Behavior As Entities)

BAE is an implementation of Hierarchical Task Networks (HTN) for Bevy, with a focus on composability, readability, and data-driven design.


What does Behavior as Entities mean? It means you define the AI's behavior as a regular old Bevy `Bundle`:

```rust,no_run
use bevy::prelude::*;
use bevy_bae::prelude::*;

fn trunk_thumper_domain() -> impl Bundle {
    (
        Name::new("Be Trunk Thumper"),
        Plan::new(),
        Select,
        tasks![
            (
                Name::new("Fight enemy"),
                Sequence,
                tasks![
                    (
                        conditions![Condition::eq("can_see_enemy", true),],
                        Operator::new(navigate_to_enemy),
                        effects![Effect::set("location", "enemy"),],
                    ),
                    Operator::new(do_trunk_slam),
                ],
            ),
            (
                Name::new("Patrol bridges"),
                Sequence,
                tasks![
                    Operator::new(choose_bridge_to_check),
                    (
                        Operator::new(navigate_to_bridge),
                        effects![Effect::set("location", "bridge"),],
                    ),
                    Operator::new(check_bridge),
                ],
            )
        ],
    )
}

fn navigate_to_enemy(In(_input): In<OperatorInput>) -> OperatorStatus {
    OperatorStatus::Success
}

fn do_trunk_slam(In(_input): In<OperatorInput>) -> OperatorStatus {
    OperatorStatus::Success
}

fn choose_bridge_to_check(In(_input): In<OperatorInput>) -> OperatorStatus {
    OperatorStatus::Success
}

fn navigate_to_bridge(In(_input): In<OperatorInput>) -> OperatorStatus {
    OperatorStatus::Success
}

fn check_bridge(In(_input): In<OperatorInput>) -> OperatorStatus {
    OperatorStatus::Success
}
```
