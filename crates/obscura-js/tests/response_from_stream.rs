//! A Response constructed from another Response's body stream must read the
//! stream's bytes, not stringify the ReadableStream object. Regression test for
//! #1361: `new Response(original.body)` produced a body of "[object Object]",
//! so `.json()` threw `SyntaxError: "[object Object]" is not valid JSON`.

use obscura_js::runtime::ObscuraJsRuntime;
use serde_json::json;

fn runtime() -> ObscuraJsRuntime {
    let mut rt = ObscuraJsRuntime::new();
    rt.set_dom(obscura_dom::parse_html("<!doctype html><body></body>"));
    rt.run_page_init();
    rt
}

#[tokio::test]
async fn response_consumes_a_supplied_body_stream() {
    let mut rt = runtime();
    rt.evaluate(
        r#"(() => {
            globalThis.result = null;
            const original = new Response('{"ok":true}');
            const wrapped = new Response(original.body);
            wrapped.json().then(
                value => { globalThis.result = JSON.stringify(value); },
                error => { globalThis.result = String(error); },
            );
        })()"#,
    )
    .unwrap();
    rt.run_event_loop_bounded(30).await.unwrap();
    assert_eq!(rt.evaluate("globalThis.result").unwrap(), json!("{\"ok\":true}"));
}

#[tokio::test]
async fn response_from_stream_reads_text() {
    let mut rt = runtime();
    rt.evaluate(
        r#"(() => {
            globalThis.result = null;
            const original = new Response('hello world');
            const wrapped = new Response(original.body);
            wrapped.text().then(
                value => { globalThis.result = value; },
                error => { globalThis.result = String(error); },
            );
        })()"#,
    )
    .unwrap();
    rt.run_event_loop_bounded(30).await.unwrap();
    assert_eq!(rt.evaluate("globalThis.result").unwrap(), json!("hello world"));
}
