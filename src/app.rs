use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! {
        cx,
        // Conduit standard boiler plate
        <meta charset="utf-8" />
        // Import Ionicon icons & Google Fonts our Bootstrap theme relies on
        <link
          href="//code.ionicframework.com/ionicons/2.0.1/css/ionicons.min.css"
          rel="stylesheet"
          type="text/css"
        />
        <link
          href="//fonts.googleapis.com/css?family=Titillium+Web:700|Source+Serif+Pro:400,700|Merriweather+Sans:400,700|Source+Sans+Pro:400,300,600,700,300italic,400italic,600italic,700italic"
          rel="stylesheet"
          type="text/css"
        />
        // Import the custom Bootstrap 4 theme from our hosted CDN
        <link rel="stylesheet" href="//demo.productionready.io/main.css" />

        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/conduit_leptos.css"/>

        // sets the document title
        <Title text="Conduit in Leptos"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route path="" view=|cx| view! { cx, <HomePage/> }/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage(cx: Scope) -> impl IntoView {
    view! { cx,
      <div class="home-page">
        <div class="banner">
          <div class="container">
            <h1 class="logo-font">"conduit"</h1>
            <p>"A place to share your knowledge."</p>
          </div>
        </div>
        // Main Content
        <div class="container page">
          <div class="row">
            <div class="col-md-9">

              // Feed toggle
              <div class="feed-toggle">
                <ul class="nav nav-pills outline-active">
                  <li class="nav-item">
                    <a class="nav-link disabled" href="">"Your Feed"</a>
                  </li>
                  <li class="nav-item">
                    <a class="nav-link active" href="">"Global Feed"</a>
                  </li>
                </ul>
              </div>

              // Article Previews
              <ArticlePreview/>
            </div>

            // Sidebar
            <div class="col-md-3">
              <div class="sidebar">
                <p>"Popular Tags"</p>
                  <TagList tags=vec!["Greg", "Leptos"]/>
              </div>
            </div>

          </div>
        </div>



      </div>
      <Footer/>
    }
}

struct ArticlePreview {
    author_name: String,
    date: String,
    title: String,
    blurb: String,
}

#[component]
fn ArticlePreview(cx: Scope) -> impl IntoView {
    let preview = ArticlePreview {
        author_name: "Eric Simons".to_string(),
        date: "January 20th".to_string(),
        title: "How to build webapps that scale".to_string(),
        blurb: "This is the description for the post.".to_string(),
    };
    view! {cx,
      <div class="article-preview">
        <div class="article-meta">
          <a href="profile.html"><img src="http://i.imgur.com/Qr71crq.jpg" /></a>
          <div class="info">
            <a href="" class="author">{preview.author_name}</a>
            <span class="date">{preview.date}</span>
          </div>
          <button class="btn btn-outline-primary btn-sm pull-xs-right">
            <i class="ion-heart"></i> 29
          </button>
        </div>
        <a href="" class="preview-link">
          <h1>{preview.title}</h1>
          <p>{preview.blurb}</p>
          <span>"Read more..."</span>
        </a>
      </div>
    }
}

#[component]
fn Footer(cx: Scope) -> impl IntoView {
    view! { cx,
      <footer>
        <div class="container">
          <a href="/" class="logo-font">"conduit"</a>
        </div>
      </footer>
    }
}

#[component]
fn TagList(cx: Scope, tags: Vec<&'static str>) -> impl IntoView {
    // TODO: make this nicer:
    let tags: Vec<String> = tags.iter().map(|s| s.to_string()).collect();
    view! { cx,
      <div class="tag-list">
        <For
          each=move || tags.clone()
          key=|tag| tag.clone()
          view=move |tag: String| {
              view! { cx,
                  <a href="" class="tag-pill tag-default">{tag}</a>
              }
          }
        />
      </div>
    }
}
