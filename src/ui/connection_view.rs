use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct ConnectionViewProps {
    remote_node_id: Option<String>,
}

#[component]
pub fn ConnectionView(props: ConnectionViewProps) -> Element {
    rsx! {
        section {
            h2 { "Bidirectional Connection" }

            dl {
                dt { "remote node ID:" }
                dd {
                    match props.remote_node_id {
                        Some(n) => rsx! { "{n}" },
                        None => rsx! { "not connected" }
                    }
                }
            }
        }
    }
}
