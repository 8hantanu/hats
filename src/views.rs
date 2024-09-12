use maud::{html, Markup};

/// Renders the form using Maud
pub fn render_form() -> Markup {
    html! {
        (maud::DOCTYPE)
        html {
            head {
                meta charset="UTF-8";
                meta name="viewport" content="width=device-width, initial-scale=1.0";
                link rel="stylesheet" href="https://yree.io/mold/assets/css/main.css";
                link rel="preconnect" href="https://fonts.googleapis.com";
                link rel="preconnect" href="https://fonts.gstatic.com" crossorigin="";
                link href="https://fonts.googleapis.com/css2?family=JetBrains+Mono:ital,wght@0,100..800;1,100..800&amp;display=swap" rel="stylesheet";
                title { "HTMX Form" }
                script src="https://unpkg.com/htmx.org" {}
            }
            body {
                div class="w" {
                    h1 { "Submit Your Name" }
                    form hx-post="/submit" hx-target="#response" {
                        label for="name" { "Name: " }
                        input type="text" id="name" name="name" required;
                        button type="submit" { "Submit" }
                    }
                    div id="response" {}
                }
            }
        }
    }
}

/// Renders the response when a form is submitted
pub fn render_response(name: &str) -> Markup {
    html! {
        p { "Hello, " (name) "!" }
    }
}
