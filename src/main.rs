use parallel_bars::ParallelBars;
use prescription::Prescription;
use yew::prelude::*;

use crate::prescription_finder::PrescriptionFinder;

mod parallel_bars;
mod prescription;
mod prescription_finder;

#[function_component]
fn App() -> Html {
    let axis = use_state(|| 0);
    let on_found = {
        let axis = axis.clone();
        Callback::from(move |aaxis| {
            axis.set(aaxis);
        })
    };

    html! {
        <div>
            <PrescriptionFinder callback={on_found}/>
            <Prescription od_axis={*axis} />
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
