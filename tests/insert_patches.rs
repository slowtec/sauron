#![deny(warnings)]
use sauron_core::{
    html::{attributes::*, *},
    mt_dom::patch::*,
    *,
};

use test_fixtures::simple_program;
use wasm_bindgen_test::*;

mod test_fixtures;

wasm_bindgen_test_configure!(run_in_browser);
#[wasm_bindgen_test]
fn test_patch_insert_node() {
    console_log::init_with_level(log::Level::Trace).ok();
    console_error_panic_hook::set_once();

    let document = web_sys::window().unwrap().document().unwrap();

    let old: Node<()> = main(
        vec![class("test1")],
        vec![ul(
            vec![class("todo")],
            vec![
                li(vec![key(1)], vec![text("item1")]),
                li(vec![key(2)], vec![text("item2")]),
                li(vec![key(3)], vec![text("item3")]),
            ],
        )],
    );

    // we remove the key1
    let update1: Node<()> = main(
        vec![class("test1")],
        vec![ul(
            vec![class("todo")],
            vec![
                li(vec![key(0)], vec![text("item0")]),
                li(vec![key(1)], vec![text("item1")]),
                li(vec![key(2)], vec![text("item2")]),
                li(vec![key(3)], vec![text("item3")]),
            ],
        )],
    );

    let patches = diff(&old, &update1);
    log::debug!("patches: {:#?}", patches);
    assert_eq!(
        patches,
        vec![InsertNode::new(
            Some(&"ul"),
            TreePath::start_at(2, vec![0, 0, 0]),
            &li(vec![key(0)], vec![text("item0")])
        )
        .into()]
    );

    let mut old_html = String::new();
    old.render(&mut old_html).expect("must render");

    let simple_program = simple_program();
    let mut dom_updater = DomUpdater::new_append_to_mount(
        &simple_program,
        old,
        &sauron_core::body(),
    );

    let container = document
        .query_selector(".test1")
        .expect("must not error")
        .expect("must exist");

    let expected = "<main class=\"test1\"><ul class=\"todo\"><li key=\"1\">item1</li><li key=\"2\">item2</li><li key=\"3\">item3</li></ul></main>";

    assert_eq!(expected, container.outer_html());

    dom_updater.update_dom(&simple_program, update1);

    let container = document
        .query_selector(".test1")
        .expect("must not error")
        .expect("must exist");

    let expected1 = "<main class=\"test1\"><ul class=\"todo\"><li key=\"0\">item0</li><li key=\"1\">item1</li><li key=\"2\">item2</li><li key=\"3\">item3</li></ul></main>";

    assert_eq!(expected1, container.outer_html());
}

#[wasm_bindgen_test]
fn test_patch_insert_node_in_the_middle() {
    console_log::init_with_level(log::Level::Trace).ok();
    console_error_panic_hook::set_once();

    let document = web_sys::window().unwrap().document().unwrap();

    let old: Node<()> = main(
        vec![class("test_middle")],
        vec![ul(
            vec![class("todo")],
            vec![
                li(vec![key(1)], vec![text("item1")]),
                li(vec![key(2)], vec![text("item2")]),
                li(vec![key(3)], vec![text("item3")]),
            ],
        )],
    );

    // we remove the key1
    let update1: Node<()> = main(
        vec![class("test_middle")],
        vec![ul(
            vec![class("todo")],
            vec![
                li(vec![key(1)], vec![text("item1")]),
                li(vec![key(0)], vec![text("item0")]),
                li(vec![key(2)], vec![text("item2")]),
                li(vec![key(3)], vec![text("item3")]),
            ],
        )],
    );

    let patches = diff(&old, &update1);
    log::debug!("patches: {:#?}", patches);
    assert_eq!(
        patches,
        vec![InsertNode::new(
            Some(&"ul"),
            TreePath::start_at(4, vec![0, 0, 1]),
            &li(vec![key(0)], vec![text("item0")])
        )
        .into()]
    );

    let mut old_html = String::new();
    old.render(&mut old_html).expect("must render");

    let simple_program = simple_program();
    let mut dom_updater = DomUpdater::new_append_to_mount(
        &simple_program,
        old,
        &sauron_core::body(),
    );

    let container = document
        .query_selector(".test_middle")
        .expect("must not error")
        .expect("must exist");

    let expected = "<main class=\"test_middle\"><ul class=\"todo\"><li key=\"1\">item1</li><li key=\"2\">item2</li><li key=\"3\">item3</li></ul></main>";

    assert_eq!(expected, container.outer_html());

    dom_updater.update_dom(&simple_program, update1);

    let container = document
        .query_selector(".test_middle")
        .expect("must not error")
        .expect("must exist");

    let expected1 = "<main class=\"test_middle\"><ul class=\"todo\"><li key=\"1\">item1</li><li key=\"0\">item0</li><li key=\"2\">item2</li><li key=\"3\">item3</li></ul></main>";

    assert_eq!(expected1, container.outer_html());
}

#[wasm_bindgen_test]
fn multiple_insert_should_work() {
    console_log::init_with_level(log::Level::Trace).ok();
    console_error_panic_hook::set_once();

    let document = web_sys::window().unwrap().document().unwrap();

    let old: Node<()> = main(
        vec![class("test5")],
        vec![ul(
            vec![class("todo")],
            vec![
                li(vec![key(1)], vec![text("item1")]),
                li(vec![key(2)], vec![text("item2")]),
                li(vec![key(3)], vec![text("item3")]),
            ],
        )],
    );

    // we remove the key1
    let update1: Node<()> = main(
        vec![class("test5")],
        vec![ul(
            vec![class("todo")],
            vec![
                li(vec![key("c")], vec![text("itemc")]),
                li(vec![key("b")], vec![text("itemb")]),
                li(vec![key("a")], vec![text("itema")]),
                li(vec![key(0)], vec![text("item0")]),
                li(vec![key(1)], vec![text("item1")]),
                li(vec![key(2)], vec![text("item2")]),
                li(vec![key(3)], vec![text("item3")]),
            ],
        )],
    );

    let patches = diff(&old, &update1);
    log::debug!("patches: {:#?}", patches);
    assert_eq!(
        patches,
        vec![
            InsertNode::new(
                Some(&"ul"),
                TreePath::start_at(2, vec![0, 0, 0]),
                &li(vec![key("c")], vec![text("itemc")])
            )
            .into(),
            InsertNode::new(
                Some(&"ul"),
                TreePath::start_at(2, vec![0, 0, 0]),
                &li(vec![key("b")], vec![text("itemb")])
            )
            .into(),
            InsertNode::new(
                Some(&"ul"),
                TreePath::start_at(2, vec![0, 0, 0]),
                &li(vec![key("a")], vec![text("itema")])
            )
            .into(),
            InsertNode::new(
                Some(&"ul"),
                TreePath::start_at(2, vec![0, 0, 0]),
                &li(vec![key(0)], vec![text("item0")])
            )
            .into(),
        ]
    );

    let mut old_html = String::new();
    old.render(&mut old_html).expect("must render");

    let simple_program = simple_program();
    let mut dom_updater = DomUpdater::new_append_to_mount(
        &simple_program,
        old,
        &sauron_core::body(),
    );

    let container = document
        .query_selector(".test5")
        .expect("must not error")
        .expect("must exist");

    let expected = "<main class=\"test5\"><ul class=\"todo\"><li key=\"1\">item1</li><li key=\"2\">item2</li><li key=\"3\">item3</li></ul></main>";

    assert_eq!(expected, container.outer_html());

    dom_updater.update_dom(&simple_program, update1);

    let container = document
        .query_selector(".test5")
        .expect("must not error")
        .expect("must exist");

    let expected1 = "<main class=\"test5\"><ul class=\"todo\"><li key=\"c\">itemc</li><li key=\"b\">itemb</li><li key=\"a\">itema</li><li key=\"0\">item0</li><li key=\"1\">item1</li><li key=\"2\">item2</li><li key=\"3\">item3</li></ul></main>";

    assert_eq!(expected1, container.outer_html());
}