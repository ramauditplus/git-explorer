use std::collections::HashMap;

use leptos::prelude::*;

mod api;
mod models;

use api::{fetch_latest_commit, fetch_repositories};
use models::{CommitInfo, Repository};

#[component]
fn App() -> impl IntoView {
    // User input
    let (username_input, set_username_input) = signal("leptos-rs".to_string());

    let (active_username, set_active_username) = signal("leptos-rs".to_string());

    let (search, set_search) = signal(String::new());

    let (language, set_language) = signal(String::new());

    let (sort_by_stars, set_sort_by_stars) = signal(false);

    let (selected_repo, set_selected_repo) = signal(None::<Repository>);

    let commit_cache = StoredValue::new(HashMap::<String, CommitInfo>::new());

    let repositories = LocalResource::new(move || {
        let username = active_username.get();

        fetch_repositories(username)
    });

    let latest_commit = AsyncDerived::new_unsync(move || {
        let selected = selected_repo.get();

        async move {
            let Some(repo) = selected else {
                return Ok::<Option<CommitInfo>, String>(None);
            };

            let key = repo.full_name;

            let cached = commit_cache.with_value(|cache| cache.get(&key).cloned());

            if let Some(commit) = cached {
                return Ok(Some(commit));
            }

            let commit = fetch_latest_commit(key.clone()).await?;

            commit_cache.update_value(|cache| {
                cache.insert(key, commit.clone());
            });

            Ok(Some(commit))
        }
    });

    view! {
        <main>
            <h1>"GitHub Repository Explorer"</h1>

            <section>
                <h2>"Search User"</h2>

                <input
                    type="text"
                    placeholder="GitHub username"
                    prop:value=move || username_input.get()
                    on:input=move |ev| {
                        set_username_input.set(event_target_value(&ev));
                    }
                />

                <button on:click=move |_| {
                    set_selected_repo.set(None);
                    set_active_username.set(username_input.get().trim().to_string());
                }>"Search"</button>
            </section>

            <hr />

            <section>
                <h2>"Filters"</h2>

                <input
                    type="text"
                    placeholder="Search repository"
                    prop:value=move || search.get()
                    on:input=move |ev| {
                        set_search.set(event_target_value(&ev));
                    }
                />

                <input
                    type="text"
                    placeholder="Filter by language"
                    prop:value=move || language.get()
                    on:input=move |ev| {
                        set_language.set(event_target_value(&ev));
                    }
                />

                <button on:click=move |_| {
                    set_sort_by_stars
                        .update(|value| {
                            *value = !*value;
                        });
                }>
                    {move || {
                        if sort_by_stars.get() { "Sort by recent" } else { "Sort by stars" }
                    }}
                </button>
            </section>

            <hr />

            <section>
                <h2>"Repositories"</h2>

                {move || {
                    match repositories.get() {
                        None => view! { <p>"Loading repositories..."</p> }.into_any(),
                        Some(Err(error)) => {

                            view! { <p>"Error: " {error}</p> }
                                .into_any()
                        }
                        Some(Ok(repos)) => {
                            let search_text = search.get().to_lowercase();
                            let language_text = language.get().to_lowercase();
                            let mut filtered: Vec<Repository> = repos
                                .into_iter()
                                .filter(|repo| {
                                    let name_matches = repo
                                        .name
                                        .to_lowercase()
                                        .contains(&search_text);
                                    let language_matches = repo
                                        .language
                                        .as_deref()
                                        .unwrap_or("")
                                        .to_lowercase()
                                        .contains(&language_text);
                                    name_matches && language_matches
                                })
                                .collect();
                            if sort_by_stars.get() {
                                filtered
                                    .sort_by(|a, b| {
                                        b.stargazers_count.cmp(&a.stargazers_count)
                                    });
                            }
                            if filtered.is_empty() {

                                view! { <p>"No repositories found."</p> }
                                    .into_any()
                            } else {
                                view! {
                                    <ul>
                                        {filtered
                                            .into_iter()
                                            .map(|repo| {
                                                let selected = repo.clone();

                                                view! {
                                                    <li>
                                                        <h3>{repo.name}</h3>

                                                        <p>{repo.description.unwrap_or_default()}</p>

                                                        <p>
                                                            "Language: "
                                                            {repo.language.unwrap_or_else(|| "Unknown".to_string())}
                                                        </p>

                                                        <p>"Stars: " {repo.stargazers_count}</p>

                                                        <a
                                                            href=repo.html_url.clone()
                                                            target="_blank"
                                                            rel="noopener noreferrer"
                                                        >
                                                            "Open GitHub"
                                                        </a>

                                                        <button on:click=move |_| {
                                                            set_selected_repo.set(Some(selected.clone()));
                                                        }>"View Details"</button>
                                                    </li>
                                                }
                                            })
                                            .collect_view()}
                                    </ul>
                                }
                                    .into_any()
                            }
                        }
                    }
                }}
            </section>

            <hr />

            <section>
                <h2>"Selected Repository"</h2>

                {move || {
                    match selected_repo.get() {
                        None => view! { <p>"No repository selected."</p> }.into_any(),
                        Some(repo) => {

                            view! {
                                <div>
                                    <h3>{repo.full_name}</h3>

                                    <p>"Stars: " {repo.stargazers_count}</p>

                                    <a href=repo.html_url target="_blank" rel="noopener noreferrer">
                                        "Open repository"
                                    </a>
                                </div>
                            }
                                .into_any()
                        }
                    }
                }}
            </section>

            <hr />

            <section>
                <h2>"Latest Commit"</h2>

                {move || {
                    match latest_commit.get() {
                        None => view! { <p>"Loading commit..."</p> }.into_any(),
                        Some(Err(error)) => {

                            view! { <p>"Error: " {error}</p> }
                                .into_any()
                        }
                        Some(Ok(None)) => {

                            view! { <p>"Select a repository first."</p> }
                                .into_any()
                        }
                        Some(Ok(Some(commit))) => {

                            view! {
                                <div>
                                    <p>
                                        <strong>"SHA: "</strong>
                                        {commit.sha}
                                    </p>

                                    <p>
                                        <strong>"Message: "</strong>
                                        {commit.commit.message}
                                    </p>

                                    <p>
                                        <strong>"Author: "</strong>
                                        {commit
                                            .commit
                                            .author
                                            .as_ref()
                                            .map(|a| a.name.clone())
                                            .unwrap_or_default()}
                                    </p>

                                    <p>
                                        <strong>"Date: "</strong>
                                        {commit
                                            .commit
                                            .author
                                            .as_ref()
                                            .map(|a| a.date.clone())
                                            .unwrap_or_default()}
                                    </p>
                                </div>
                            }
                                .into_any()
                        }
                    }
                }}
            </section>
        </main>
    }
}

#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App);
}
