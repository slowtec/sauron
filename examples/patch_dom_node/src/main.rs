use std::{cell::RefCell, rc::Rc};

use sauron_core::{
    dom::{self, DomNode},
    prelude::Node,
    vdom,
};
use sauron_html_parser::parse_html;

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    log::info!("Start example");

    let ev_callback = |_| {};

    let body_node = DomNode::from(web_sys::Node::from(dom::util::body()));
    let mount_node = Rc::new(RefCell::new(Some(body_node)));

    let old_node: Node<()> = parse_html::<()>("<div></div>").unwrap().unwrap();
    let new_node: Node<()> = parse_html::<()>("<div>Hello world</div>").unwrap().unwrap();

    let root = dom::create_dom_node(&old_node, ev_callback);
    let root_node = Rc::new(RefCell::new(Some(root)));

    let vdom_patches = vdom::diff(&old_node, &new_node);
    log::debug!("Created {} VDOM patch(es)", vdom_patches.len());

    let dom_patches = dom::convert_patches(
        &root_node.borrow().as_ref().unwrap(),
        &vdom_patches,
        ev_callback,
    )
    .unwrap();
    log::debug!("Created {} DOM patch(es)", dom_patches.len());

    dom::apply_dom_patches(root_node, mount_node, dom_patches).unwrap();
}
