# Bevy BAE (Behavior As Entities)

BAE is an implementation of Hierarchical Task Networks (HTN) for Bevy, with a focus on composability, readability, and data-driven design.


What does Behavior as Entities mean? It means you define the AI's behavior as a regular old Bevy `Bundle`:
```rust
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
                        effects![
                            Effect::set("location", "enemy"),
                        ],
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
```
