use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub od_sph: Option<f32>,
    #[prop_or_default]
    pub od_cyl: Option<f32>,
    #[prop_or_default]
    pub od_axis: Option<u8>,
    #[prop_or_default]
    pub os_sph: Option<f32>,
    #[prop_or_default]
    pub os_cyl: Option<f32>,
    #[prop_or_default]
    pub os_axis: Option<u8>,
}

#[function_component]
pub fn Prescription(props: &Props) -> Html {
    let od_sph = props
        .od_sph
        .map(|x| format!("{:.2}", x))
        .unwrap_or_default();
    let od_cyl = props
        .od_cyl
        .map(|x| format!("{:.2}", x))
        .unwrap_or_default();
    let od_axis = props.od_axis.map(|x| format!("{}", x)).unwrap_or_default();
    let os_sph = props
        .os_sph
        .map(|x| format!("{:.2}", x))
        .unwrap_or_default();
    let os_cyl = props
        .os_cyl
        .map(|x| format!("{:.2}", x))
        .unwrap_or_default();
    let os_axis = props.os_axis.map(|x| format!("{}", x)).unwrap_or_default();

    html! {
        <table class={classes!("prescription")}>
            <thead>
                <tr>
                    <th></th>
                    <th>{ "SPH" }</th>
                    <th>{ "CYL" }</th>
                    <th>{ "AXIS" }</th>
                </tr>
            </thead>
            <tr class={classes!("od")}>
                <td>{ "OD" }</td>
                <td class={classes!("sph")}>{od_sph}</td>
                <td class={classes!("cyl")}>{od_cyl}</td>
                <td class={classes!("axis")}>{od_axis}</td>
            </tr>
            <tr class={classes!("os")}>
                <td>{ "OS" }</td>
                <td class={classes!("sph")}>{os_sph}</td>
                <td class={classes!("cyl")}>{os_cyl}</td>
                <td class={classes!("axis")}>{os_axis}</td>
            </tr>
        </table>
    }
}
