use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub axis: i32,
    pub num_bars: u8,
    pub onclick: Callback<()>,
}

#[function_component]
pub fn ParallelBars(props: &Props) -> Html {
    let onclick = Callback::from({
        let onclick = props.onclick.clone();
        move |_| {
            onclick.emit(());
        }
    });

    html! {
        <div {onclick} class={classes!("parallel-bars")} style={format!("transform: rotate({}deg)", props.axis)}>
            {vec![html!{<div />}; props.num_bars as usize * 2]}
        </div>
    }
}
