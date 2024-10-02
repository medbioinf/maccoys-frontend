// 3rd party imports
use dioxus::prelude::*;
use maccoys_exchange_entities::results_api::Spectrum;
use serde_json::json;

// internal imports
use crate::{configuration::Configuration, routes::Routes};

#[component]
pub fn Spectrum(search_uuid: String, ms_run_name: String, spectrum_id: String) -> Element {
    let maccoys_base_url = use_context::<Configuration>()
        .get_maccoys_base_url()
        .to_string();
    let search_uuid_clone = search_uuid.clone();
    let ms_run_name_clone = ms_run_name.clone();
    let spectrum_id_clone = spectrum_id.clone();

    let mut selected_identification_tab = use_signal(|| 0_usize);

    let spectrum_feature = use_resource(move || {
        let url = format!(
            "{}/api/searches/{}/{}/{}",
            maccoys_base_url, search_uuid_clone, ms_run_name_clone, spectrum_id_clone
        );
        async move { reqwest::get(url).await.unwrap().json::<Spectrum>().await }
    });

    let spectrum_plot = eval(
        r#"
            let plot_data = await dioxus.recv();

            let data = [{
                x: plot_data.get("data_x"),
                y: plot_data.get("data_y"),
                type: plot_data.get("data_type"),
            }];


            let layout = {
                xaxis: {
                    title: plot_data.get("xaxis_title"),
                },
                yaxis: {
                    title: plot_data.get("yaxis_title"),
                },
            };

            Plotly.newPlot("specturm-plot", data, layout);
        "#,
    );

    let score_plot = eval(
        r#"
            let plot_data = await dioxus.recv();

            let data = [{
                x: plot_data.get("histogram_x"),
                y: plot_data.get("histogram_y"),
                name: "\# PSMs",
                type: "bar",
            },{
                x: plot_data.get("exp_score_x").get("values"),
                y: plot_data.get("exp_score_y").get("values"),
                name: "Probability score",
                type: "line",
                yaxis: 'y2',
            }];

            console.log(data);

            let layout = {
                xaxis: {
                    title: "Score",
                },
                yaxis: {
                    title: "\# PSMs",
                },
                yaxis2: {
                    title: 'Probability score',
                    overlaying: 'y',
                    side: 'right'
                }
            };

            Plotly.newPlot(plot_data.get("element_id"), data, layout);
        "#,
    );

    rsx! {
        nav {
            ul { class: "breadcrumb",
                li { class: "breadcrumb-item",
                    "Search: "
                    Link {
                        to: Routes::Search {
                            search_uuid: search_uuid.clone(),
                        },
                        "{search_uuid}"
                    }
                }
                li { class: "breadcrumb-item",
                    "MS Run: "
                    Link {
                        to: Routes::MsRun {
                            search_uuid: search_uuid.clone(),
                            ms_run_name: ms_run_name.clone(),
                        },
                        "{ms_run_name}"
                    }
                }
                li { class: "breadcrumb-item active", "Spectrum: {spectrum_id}" }
            }
        }

        h2 { "Spectrum {spectrum_id}" }
        match &*spectrum_feature.read_unchecked() {
            Some(Ok(spectrum)) => {

                spectrum_plot.send(json!({
                    "data_x": spectrum.get_mz(),
                    "data_y": spectrum.get_intensity(),
                    "data_type": "bar",
                    "xaxis_title": "m/z",
                    "yaxis_title": "Intensity",
                })).unwrap();

                for (idx, identification) in spectrum.get_identifications().iter().enumerate() {
                    if identification.get_psms().is_none() {
                        continue;
                    }
                    let psms = identification.get_psms().as_ref().unwrap();
                    let (histogram_x, histogram_y) = identification.get_score_histogram().unwrap();

                    score_plot.send(json!({
                        "histogram_x": histogram_x,
                        "histogram_y": histogram_y,
                        "exp_score_x": psms["xcorr"],
                        "exp_score_y": psms["exp_score"],
                        "element_id": format!("score-plot-{}", idx),
                    })).unwrap();
                }


                rsx!{
                    div {
                        id: "specturm-plot"
                    }
                    ul {
                        class: "nav nav-tabs",
                        for (idx, identification) in spectrum.get_identifications().iter().enumerate() {
                            li {
                                button {
                                    r#type: "button",
                                    class: if idx != *selected_identification_tab.read_unchecked() { "nav-link" } else { "nav-link active" },
                                    onclick: move |_| {
                                        selected_identification_tab.set(idx);
                                    },
                                    "[{identification.get_precursor()} + {identification.get_charge()}H]"
                                    sup {"{identification.get_charge()}+"}
                                }
                            }
                        }
                    }
                    div {
                        class: "tab-content",
                        for (idx, identification) in spectrum.get_identifications().iter().enumerate() {
                            div {
                                class: if idx != *selected_identification_tab.read_unchecked() { "tab-pane fade" } else { "tab-pane fade show active" },
                                div {
                                    id: "score-plot-{idx}",
                                }
                                if let Some(goodness) = identification.get_goodnesses() {
                                    div {
                                        class: "dataframe mb-3",
                                        for (col_idx, col) in goodness.get_columns().iter().enumerate() {
                                            div {
                                                class: "dataframe-cell dataframe-head",
                                                style: "grid-column: {col_idx + 1} / span 1; grid-row: 1 / span 1",
                                                "{col.name()}"
                                            }
                                        }
                                        for (row_idx, row) in identification.iter_goodness_rows().unwrap().enumerate() {
                                            for (col_idx, col) in row.into_iter().enumerate() {
                                                div {
                                                    class: "dataframe-cell",
                                                    style: "grid-column: {col_idx + 1} / span 1; grid-row: {row_idx + 2} / span 1",
                                                    "{col}"
                                                }
                                            }
                                        }
                                    }
                                } else {
                                    p { "No goodness values" }
                                }
                                if let Some(psms) = identification.get_psms() {
                                    div {
                                        class: "dataframe",
                                        for (col_idx, col) in psms.get_columns().iter().enumerate() {
                                            div {
                                                class: "dataframe-cell dataframe-head",
                                                style: "grid-column: {col_idx + 1} / span 1; grid-row: 1 / span 1",
                                                "{col.name()}"
                                            }
                                        }
                                        for (row_idx, row) in identification.iter_psm_rows().unwrap().enumerate() {
                                            for (col_idx, col) in row.into_iter().enumerate() {
                                                div {
                                                    class: "dataframe-cell",
                                                    style: "grid-column: {col_idx + 1} / span 1; grid-row: {row_idx + 2} / span 1",
                                                    "{col}"
                                                }
                                            }
                                        }
                                    }
                                } else {
                                    p { "No PSMs" }
                                }
                            }
                        }
                    }
                }
            },
            Some(Err(_))=> rsx! {
                p { "Failed to load spectrum" }
            },
            None => rsx! {
                p { "Loading spectrum..." }
            }
        }
    }
}
