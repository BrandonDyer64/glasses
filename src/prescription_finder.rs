use yew::prelude::*;

use crate::parallel_bars::ParallelBars;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub callback: Callback<u8>,
}

#[function_component]
pub fn PrescriptionFinder(props: &Props) -> Html {
    let stage = use_state(|| 0);
    let axis_range = use_state(|| (0 - 45, 180 + 45));

    let middle = (axis_range.0 + axis_range.1) / 2;
    let first = (axis_range.0 * 1 + middle) / 2;
    let second = (middle + axis_range.1 * 1) / 2;

    let on_first = {
        let stage = stage.clone();
        let axis_range = axis_range.clone();
        Callback::from(move |_| {
            stage.set(*stage + 1);
            axis_range.set((axis_range.0, second));
        })
    };

    let on_second = {
        let stage = stage.clone();
        let axis_range = axis_range.clone();
        Callback::from(move |_| {
            stage.set(*stage + 1);
            axis_range.set((first, axis_range.1));
        })
    };

    html! {
        <div class={classes!("prescription-finder")}>
            <div>{format!("Axis between: {} : {}", first, second)}</div>
            <ParallelBars axis={first} num_bars={10 + *stage} onclick={on_first}/>
            <ParallelBars axis={second} num_bars={10 + *stage} onclick={on_second}/>
        </div>
    }
}
