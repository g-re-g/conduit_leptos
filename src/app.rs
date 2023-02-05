use crate::validations::{Field, FieldError};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[cfg(feature = "ssr")]
pub fn register_server_functions() {
    let _ = AttemptLogin::register();
}

#[cfg(feature = "ssr")]
use actix_web::http::{
    header::HeaderMap, header::HeaderName, header::HeaderValue, header::SET_COOKIE, StatusCode,
};

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
                    <Route path="login" view=|cx| view! { cx, <LoginPage/> }/>

                </Routes>
            </main>
        </Router>
    }
}

fn get_errors<T>(
    latest_result: &dyn Fn() -> Option<Result<T, ServerFnError>>,
    do_map: &dyn Fn(T) -> Field<String>,
) -> Vec<FieldError> {
    latest_result()
        .map(|res| match res {
            Ok(login_attempt) => do_map(login_attempt).errors,
            Err(_) => vec![],
        })
        .unwrap_or(vec![])
}

#[component]
fn LoginForm(cx: Scope) -> impl IntoView {
    let attempt_login_form = create_server_multi_action::<AttemptLogin>(cx);
    let submissions = attempt_login_form.submissions();
    let pending_submissions = move || submissions.get().iter().find(|s| s.pending()()).is_some();
    let latest_result = move || {
        let subs = submissions.get();
        subs.iter().last().map(|s| s.value.get()).unwrap_or(None)
    };

    view! {cx,
      <MultiActionForm action=attempt_login_form>
        <fieldset disabled=pending_submissions>

           // Email Address
           <fieldset class="form-group">
              <FieldErrors errors=move || get_errors(&latest_result, &|res| res.email)/>
             <input class="form-control form-control-lg" type="text" placeholder="Email" name="email"/>
           </fieldset>

           // Password
           <fieldset class="form-group">
              <FieldErrors errors=move || get_errors(&latest_result, &|res| res.password)/>
              <input class="form-control form-control-lg" type="password" placeholder="Password" name="password"/>
            </fieldset>

            // Submit
            <button class="btn btn-lg btn-primary pull-xs-right">"Sign up"</button>
        </fieldset>
      </MultiActionForm>
    }
}

#[component]
fn FieldErrors<E>(cx: Scope, errors: E) -> impl IntoView
where
    E: Fn() -> Vec<FieldError> + 'static + Copy,
{
    view! {cx,
       <Show when=move || !errors().is_empty() fallback=move |_| {}>
          <ul class="error-messages">
            <For each=errors key=move |e| e.to_string() view=move |e| {
              view!{cx, <li>{e}</li>}
            } />
          </ul>
       </Show>
    }
}

#[component]
fn LoginPage(cx: Scope) -> impl IntoView {
    view! {cx,
        <div class="auth-page">
          <Header />
          <div class="container page">
            <div class="row">
              <div class="col-md-6 offset-md-3 col-xs-12">
                <h1 class="text-xs-center">"Log In"</h1>
                <p class="text-xs-center">
                  // TODO: Hook up this link
                  <a href="">"Don't Have an account?"</a>
                </p>
                <LoginForm/>
              </div>
            </div>
          </div>
        </div>
    }
}

#[component]
fn RegisterPage(cx: Scope) -> impl IntoView {
    view! {cx,
        <div class="auth-page">
          <Header />
          <div class="container page">
            <div class="row">
              <div class="col-md-6 offset-md-3 col-xs-12">
                <h1 class="text-xs-center">"Sign up"</h1>
                <p class="text-xs-center">
                  <a href="">"Have an account?"</a>
                </p>

                <ul class="error-messages">
                  <li>"That email is already taken"</li>
                </ul>

                <form>
                  <fieldset class="form-group">
                    <input class="form-control form-control-lg" type="text" placeholder="Your Name" />
                  </fieldset>
                  <fieldset class="form-group">
                    <input class="form-control form-control-lg" type="text" placeholder="Email" />
                  </fieldset>
                  <fieldset class="form-group">
                    <input class="form-control form-control-lg" type="password" placeholder="Password" />
                  </fieldset>
                  <button class="btn btn-lg btn-primary pull-xs-right">"Sign up"</button>
                </form>


              </div>
            </div>
          </div>
        </div>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage(cx: Scope) -> impl IntoView {
    view! { cx,
      <div class="home-page">
        // Header
        <Header />
        // Banner
        <Banner />
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
fn Header(cx: Scope) -> impl IntoView {
    view! {cx,
      <nav class="navbar navbar-light">
        <div class="container">
          <A class="navbar-brand" href="/">"conduit"</A>
          // TODO: "active" class to a when on page
          <ul class="nav navbar-nav pull-xs-right">
            <li class="nav-item">
              <a class="nav-link active" href="">"Home"</a>
            </li>
            <li class="nav-item">
              <a class="nav-link" href=""> <i class="ion-compose"></i>" New Article "</a>
            </li>
            <li class="nav-item">
              <a class="nav-link" href=""> <i class="ion-gear-a"></i>" Settings "</a>
            </li>
            <li class="nav-item">
              <a class="nav-link" href="">"Sign in"</a>
            </li>
            <li class="nav-item">
              <a class="nav-link" href="">"Sign up"</a>
            </li>
          </ul>
        </div>
      </nav>
    }
}

#[component]
fn Banner(cx: Scope) -> impl IntoView {
    view! { cx,
      <div class="banner">
        <div class="container">
          <h1 class="logo-font">"conduit"</h1>
          <p>"A place to share your knowledge."</p>
        </div>
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

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct LoginAttempt {
    pub email: Field<String>,
    pub password: Field<String>,
}

#[server(AttemptLogin, "/api")]
pub async fn attempt_login(
    cx: Scope,
    email: String,
    password: String,
) -> Result<LoginAttempt, ServerFnError> {
    // fake API delay
    std::thread::sleep(std::time::Duration::from_millis(500));
    Ok(LoginAttempt {
        email: Field::required(None),
        password: Field::required(None),
    })
}

#[cfg(feature = "ssr")]
fn set_header(cx: &Scope, key: HeaderName, val: HeaderValue) {
    let res_options_outer = use_context::<leptos_actix::ResponseOptions>(*cx);
    if let Some(res_options) = res_options_outer {
        res_options.insert_header(key, val);
    }
}
