use async_graphql::*;

#[async_std::test]
pub async fn test_variables() {
    struct QueryRoot;

    #[Object]
    impl QueryRoot {
        #[field]
        pub async fn int_val(&self, value: i32) -> i32 {
            value
        }

        #[field]
        pub async fn int_list_val(&self, value: Vec<i32>) -> Vec<i32> {
            value
        }
    }

    let schema = Schema::new(QueryRoot, EmptyMutation, EmptySubscription);
    let query = QueryBuilder::new(
        r#"
            query QueryWithVariables($intVal: Int!, $intListVal: [Int!]!) {
                intVal(value: $intVal)
                intListVal(value: $intListVal)
            }
        "#,
    )
    .variables(
        Variables::parse_from_json(serde_json::json!({
            "intVal": 10,
             "intListVal": [1, 2, 3, 4, 5],
        }))
        .unwrap(),
    );
    let resp = query.execute(&schema).await.unwrap();
    assert_eq!(
        resp.data,
        serde_json::json!({
            "intVal": 10,
            "intListVal": [1, 2, 3, 4, 5],
        })
    );
}

#[async_std::test]
pub async fn test_variable_default_value() {
    struct QueryRoot;

    #[Object]
    impl QueryRoot {
        #[field]
        pub async fn int_val(&self, value: i32) -> i32 {
            value
        }
    }

    let schema = Schema::new(QueryRoot, EmptyMutation, EmptySubscription);
    let resp = schema
        .execute(
            r#"
            query QueryWithVariables($intVal: Int = 10) {
                intVal(value: $intVal)
            }
        "#,
        )
        .await
        .unwrap();
    assert_eq!(
        resp.data,
        serde_json::json!({
            "intVal": 10,
        })
    );
}
