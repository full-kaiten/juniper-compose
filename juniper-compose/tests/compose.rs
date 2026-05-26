use juniper::{graphql_object, EmptyMutation, EmptySubscription, RootNode, Variables};
use juniper_compose::{composable_object, composite_object};

#[derive(Default)]
struct UserQueries;

#[composable_object]
#[graphql_object]
impl UserQueries {
    fn user_name() -> &'static str {
        "alice"
    }
}

#[derive(Default)]
struct TaskQueries;

#[composable_object]
#[graphql_object]
impl TaskQueries {
    fn task_count() -> i32 {
        3
    }
}

composite_object!(Query(UserQueries, TaskQueries));

#[test]
fn composed_query_exposes_all_fields() {
    let schema = RootNode::new(
        Query,
        EmptyMutation::<()>::new(),
        EmptySubscription::<()>::new(),
    );

    let (value, errors) = juniper::execute_sync(
        "{ userName taskCount }",
        None,
        &schema,
        &Variables::new(),
        &(),
    )
    .expect("query should execute");

    assert!(errors.is_empty());
    assert_eq!(
        value.to_string(),
        "{\"userName\": \"alice\", \"taskCount\": 3}"
    );
}
