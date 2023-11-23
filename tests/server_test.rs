use httpdate::fmt_http_date;
use std::time::SystemTime;

use yeti;

type Result = std::result::Result<(), yeti::effect::Effect>;

#[tokio::test]
async fn evaluate_server_with_string_route_and_no_port() -> Result {
    let env = yeti::core::environment();
    let (env, actual) =
        yeti::evaluate_source(env, r#"(server/start {:routes {"/" "Hello Yeti"}})"#).await?;
    assert!(matches!(actual, yeti::Expression::NativeType(_)));
    let (env, actual) = yeti::evaluate_source(env, r#"(http/get "http://localhost:3000")"#).await?;
    let now = SystemTime::now();
    let formatted_date = fmt_http_date(now);
    let expected_response_str = format!(
        r#"
        {{:headers {{:content-length "10"
                     :content-type "text/plain; charset=utf-8"
                     :date "{}"}}
          :status 200
          :url "http://localhost:3000/"
          :text "Hello Yeti"}}
        "#,
        formatted_date
    );
    let (_, expected) = yeti::evaluate_source(env, &expected_response_str).await?;
    assert_eq!(actual, expected);
    Ok(())
}

#[tokio::test]
async fn evaluate_server_with_string_route() -> Result {
    let env = yeti::core::environment();
    let (env, actual) = yeti::evaluate_source(
        env,
        r#"
        (server/start {:port 3001
                       :routes {"/" "Hello Yeti"}})
        "#,
    )
    .await?;
    assert!(matches!(actual, yeti::Expression::NativeType(_)));
    let (env, actual) = yeti::evaluate_source(env, r#"(http/get "http://localhost:3001")"#).await?;
    let now = SystemTime::now();
    let formatted_date = fmt_http_date(now);
    let expected_response_str = format!(
        r#"
        {{:headers {{:content-length "10"
                     :content-type "text/plain; charset=utf-8"
                     :date "{}"}}
          :status 200
          :url "http://localhost:3001/"
          :text "Hello Yeti"}}
        "#,
        formatted_date
    );
    let (_, expected) = yeti::evaluate_source(env, &expected_response_str).await?;
    assert_eq!(actual, expected);
    Ok(())
}

#[tokio::test]
async fn evaluate_server_with_html_route() -> Result {
    let env = yeti::core::environment();
    let (env, actual) = yeti::evaluate_source(
        env,
        r#"
        (server/start {:port 3002
                       :routes {"/" [:ul [:li "first"] [:li "second"]]}})
        "#,
    )
    .await?;
    assert!(matches!(actual, yeti::Expression::NativeType(_)));
    let (env, actual) = yeti::evaluate_source(env, r#"(http/get "http://localhost:3002")"#).await?;
    let now = SystemTime::now();
    let formatted_date = fmt_http_date(now);
    let expected_response_str = format!(
        r#"
        {{:headers {{:content-length "38"
                     :content-type "text/html; charset=utf-8"
                     :date "{}"}}
          :status 200
          :url "http://localhost:3002/"
          :html [:html [:head] [:body [:ul [:li "first"] [:li "second"]]]]}}
        "#,
        formatted_date
    );
    let (_, expected) = yeti::evaluate_source(env, &expected_response_str).await?;
    assert_eq!(actual, expected);
    Ok(())
}

#[tokio::test]
async fn evaluate_server_with_function_route() -> Result {
    let env = yeti::core::environment();
    let (env, actual) = yeti::evaluate_source(
        env,
        r#"
        (server/start {:port 3003
                       :routes {"/" (fn [req] [:ul [:li "first"] [:li "second"]])}})
        "#,
    )
    .await?;
    assert!(matches!(actual, yeti::Expression::NativeType(_)));
    let (env, actual) = yeti::evaluate_source(env, r#"(http/get "http://localhost:3003")"#).await?;
    let now = SystemTime::now();
    let formatted_date = fmt_http_date(now);
    let expected_response_str = format!(
        r#"
        {{:headers {{:content-length "38"
                     :content-type "text/html; charset=utf-8"
                     :date "{}"}}
          :status 200
          :url "http://localhost:3003/"
          :html [:html [:head] [:body [:ul [:li "first"] [:li "second"]]]]}}
        "#,
        formatted_date
    );
    let (_, expected) = yeti::evaluate_source(env, &expected_response_str).await?;
    assert_eq!(actual, expected);
    Ok(())
}

#[tokio::test]
async fn evaluate_server_show_request_as_str() -> Result {
    let env = yeti::core::environment();
    let (env, _) = yeti::evaluate_source(env, "(def value (atom nil))").await?;
    let (env, _) = yeti::evaluate_source(
        env,
        r#"
        (defn handler [req]
          (reset! value req)
          [:p "hello"])
        "#,
    )
    .await?;
    let (env, _) = yeti::evaluate_source(
        env,
        r#"(def s (server/start {:port 3004 :routes {"/" handler}}))"#
    )
    .await?;
    let (env, actual) = yeti::evaluate_source(env, r#"(http/get "http://localhost:3004")"#).await?;
    let now = SystemTime::now();
    let formatted_date = fmt_http_date(now);
    let expected_response_str = format!(
        r#"
        {{:headers {{:content-length "12"
                     :content-type "text/html; charset=utf-8"
                     :date "{}"}}
          :status 200
          :url "http://localhost:3004/"
          :html [:html [:head] [:body [:p "hello"]]]}}
        "#,
        formatted_date
    );
    let (env, expected) = yeti::evaluate_source(env, &expected_response_str).await?;
    assert_eq!(actual, expected);
    let (env, actual) = yeti::evaluate_source(env, "@value").await?;
    let (_, expected) = yeti::evaluate_source(
        env,
        r#"
        {:headers {:accept "*/*", :host "localhost:3004"}
         :method "GET"
         :path "/"}
        "#
    ).await?;
    assert_eq!(actual, expected);
    Ok(())
}

#[tokio::test]
async fn evaluate_server_query_parameters() -> Result {
    let env = yeti::core::environment();
    let (env, _) = yeti::evaluate_source(env, "(def value (atom nil))").await?;
    let (env, _) = yeti::evaluate_source(
        env,
        r#"
        (defn handler [req]
          (reset! value req)
          [:p "hello"])
        "#,
    )
    .await?;
    let (env, _) = yeti::evaluate_source(
        env,
        r#"(def s (server/start {:port 3005 :routes {"/" handler}}))"#
    )
    .await?;
    let (env, actual) = yeti::evaluate_source(env, r#"(http/get "http://localhost:3005?foo=bar&baz=qux")"#).await?;
    let now = SystemTime::now();
    let formatted_date = fmt_http_date(now);
    let expected_response_str = format!(
        r#"
        {{:headers {{:content-length "12"
                     :content-type "text/html; charset=utf-8"
                     :date "{}"}}
          :status 200
          :url "http://localhost:3005/?foo=bar&baz=qux"
          :html [:html [:head] [:body [:p "hello"]]]}}
        "#,
        formatted_date
    );
    let (env, expected) = yeti::evaluate_source(env, &expected_response_str).await?;
    assert_eq!(actual, expected);
    let (env, actual) = yeti::evaluate_source(env, "@value").await?;
    let (_, expected) = yeti::evaluate_source(
        env,
        r#"
        {:headers {:accept "*/*", :host "localhost:3005"}
         :method "GET"
         :query {:baz "qux", :foo "bar"}
         :path "/"}
        "#
    ).await?;
    assert_eq!(actual, expected);
    Ok(())
}

#[tokio::test]
async fn evaluate_server_url_parameters() -> Result {
    let env = yeti::core::environment();
    let (env, _) = yeti::evaluate_source(env, "(def value (atom nil))").await?;
    let (env, _) = yeti::evaluate_source(
        env,
        r#"
        (defn handler [req]
          (reset! value req)
          [:p "hello"])
        "#,
    )
    .await?;
    let (env, _) = yeti::evaluate_source(
        env,
        r#"(def s (server/start {:port 3006 :routes {"/hello/:name" handler}}))"#
    )
    .await?;
    let (env, actual) =
        yeti::evaluate_source(env, r#"(http/get "http://localhost:3006/hello/joe")"#).await?;
    let now = SystemTime::now();
    let formatted_date = fmt_http_date(now);
    let expected_response_str = format!(
        r#"
        {{:headers {{:content-length "12"
                     :content-type "text/html; charset=utf-8"
                     :date "{}"}}
          :status 200
          :url "http://localhost:3006/hello/joe"
          :html [:html [:head] [:body [:p "hello"]]]}}
        "#,
        formatted_date
    );
    let (env, expected) = yeti::evaluate_source(env, &expected_response_str).await?;
    assert_eq!(actual, expected);
    let (env, actual) = yeti::evaluate_source(env, "@value").await?;
    let (_, expected) = yeti::evaluate_source(
        env,
        r#"
        {:headers {:accept "*/*", :host "localhost:3006"}
         :method "GET"
         :params {:name "joe"}
         :path "/hello/joe"}
        "#
    ).await?;
    assert_eq!(actual, expected);
    Ok(())
}

#[tokio::test]
async fn evaluate_server_form_data() -> Result {
    let env = yeti::core::environment();
    let (env, _) = yeti::evaluate_source(env, "(def value (atom nil))").await?;
    let (env, _) = yeti::evaluate_source(
        env,
        r#"
        (defn handler [req]
          (reset! value req)
          [:p "hello"])
        "#,
    )
    .await?;
    let (env, _) = yeti::evaluate_source(
        env,
        r#"(def s (server/start {:port 3007 :routes {"/" handler}}))"#
    )
    .await?;
    let (env, actual) =
        yeti::evaluate_source(env, r#"(http/post "http://localhost:3007" {:form {:foo "bar" :baz "qux"}})"#).await?;
    let now = SystemTime::now();
    let formatted_date = fmt_http_date(now);
    let expected_response_str = format!(
        r#"
        {{:headers {{:content-length "12"
                     :content-type "text/html; charset=utf-8"
                     :date "{}"}}
          :status 200
          :url "http://localhost:3007/"
          :html [:html [:head] [:body [:p "hello"]]]}}
        "#,
        formatted_date
    );
    let (env, expected) = yeti::evaluate_source(env, &expected_response_str).await?;
    assert_eq!(actual, expected);
    let (env, actual) = yeti::evaluate_source(env, "@value").await?;
    let (_, expected) = yeti::evaluate_source(
        env,
        r#"
        {:headers {:accept "*/*"
                   :host "localhost:3007"
                   :content-type "application/x-www-form-urlencoded"
                   :content-length "15"}
         :method "POST"
         :form {:foo "bar"
                :baz "qux"}
         :path "/"}
        "#
    ).await?;
    assert_eq!(actual, expected);
    Ok(())
}

#[tokio::test]
async fn evaluate_server_post_json_data() -> Result {
    let env = yeti::core::environment();
    let (env, _) = yeti::evaluate_source(env, "(def value (atom nil))").await?;
    let (env, _) = yeti::evaluate_source(
        env,
        r#"
        (defn handler [req]
          (reset! value req)
          [:p "hello"])
        "#,
    )
    .await?;
    let (env, _) = yeti::evaluate_source(
        env,
        r#"(def s (server/start {:port 3008 :routes {"/" handler}}))"#
    )
    .await?;
    let (env, actual) =
        yeti::evaluate_source(env, r#"(http/post "http://localhost:3008" {:json {:foo "bar" :baz "qux"}})"#).await?;
    let now = SystemTime::now();
    let formatted_date = fmt_http_date(now);
    let expected_response_str = format!(
        r#"
        {{:headers {{:content-length "12"
                     :content-type "text/html; charset=utf-8"
                     :date "{}"}}
          :status 200
          :url "http://localhost:3008/"
          :html [:html [:head] [:body [:p "hello"]]]}}
        "#,
        formatted_date
    );
    let (env, expected) = yeti::evaluate_source(env, &expected_response_str).await?;
    assert_eq!(actual, expected);
    let (env, actual) = yeti::evaluate_source(env, "@value").await?;
    let (_, expected) = yeti::evaluate_source(
        env,
        r#"
        {:headers {:accept "*/*"
                   :host "localhost:3008"
                   :content-type "application/json"
                   :content-length "25"}
         :method "POST"
         :json {:foo "bar"
                :baz "qux"}
         :path "/"}
        "#
    ).await?;
    assert_eq!(actual, expected);
    Ok(())
}

#[tokio::test]
async fn server_route_returns_json() -> Result {
    let env = yeti::core::environment();
    let (env, actual) = yeti::evaluate_source(
        env,
        r#"(server/start {:port 3009 :routes {"/" {:foo :bar}}})"#,
    )
    .await?;
    assert!(matches!(actual, yeti::Expression::NativeType(_)));
    let (env, actual) = yeti::evaluate_source(env, r#"(http/get "http://localhost:3009")"#).await?;
    let now = SystemTime::now();
    let formatted_date = fmt_http_date(now);
    let expected_response_str = format!(
        r#"
        {{:headers {{:content-length "13"
                     :content-type "application/json"
                     :date "{}"}}
          :status 200
          :url "http://localhost:3009/"
          :json {{:foo "bar"}}}}
        "#,
        formatted_date
    );
    let (_, expected) = yeti::evaluate_source(env, &expected_response_str).await?;
    assert_eq!(actual, expected);
    Ok(())
}

#[tokio::test]
async fn evaluate_stop() -> Result {
    let tokens = yeti::Tokens::from_str("(def s (server/start {:port 9090}))");
    let expression = yeti::parse(tokens);
    let environment = yeti::core::environment();
    let (environment, _) = yeti::evaluate(environment, expression).await?;
    let tokens = yeti::Tokens::from_str("(server/stop s)");
    let expression = yeti::parse(tokens);
    let (_, actual) = yeti::evaluate(environment, expression).await?;
    let expected = yeti::Expression::Nil;
    assert_eq!(actual, expected);
    Ok(())
}
