
use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct ConnectionFormProps {
    connect_to_peer: Callback<String>,
}

#[component]
pub fn ConnectionForm(props: ConnectionFormProps) -> Element {
    let mut peer_node_id = use_signal(|| "".to_string());

    rsx! {
        section {
            h2 { "Bidirectional Connection" }

            fieldset {
                label {
                    for: "peer_node_id",
                    "peer node id:"
                }

                input {
                    id: "peer_node_id",
                    value: "{peer_node_id}",
                    oninput: move |event| peer_node_id.set(event.value().clone()),
                    style: "min-width: 40em;"
                }
            }

            button {
                onclick: move |_| {
                    props.connect_to_peer.call(peer_node_id.read().clone());
                },
                "connect"
            }
        }
    }
}
