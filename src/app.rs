use crate::validations::{Field, FieldError};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[cfg(feature = "ssr")]
pub fn register_server_functions() {
    let _ = AttemptLogin::register();
    let _ = GetCurrentUser::register();
    let _ = Logout::register();
}

#[cfg(feature = "ssr")]
use actix_web::http::{
    header::HeaderMap, header::HeaderName, header::HeaderValue, header::SET_COOKIE, StatusCode,
};
#[cfg(feature = "ssr")]
use actix_web::FromRequest;

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
                    <Route path="logged-out" view=|cx| view! { cx, <LogoutPage/> }/>
                    <Route path="settings" view=|cx| view! { cx, <SettingsPage/> }/>
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
        let res = subs.iter().last().map(|s| s.value.get()).unwrap_or(None);

        if let Some(Ok(res)) = &res {
            if res.is_valid() {
                let nav = use_navigate(cx);
                let _ = nav("/", Default::default());
            }
        }

        res
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
fn LogoutPage(cx: Scope) -> impl IntoView {
    view! {cx,
        <div class="auth-page">
          <Header />
          <div class="container page">
            <div class="row">
              <div class="col-md-6 offset-md-3 col-xs-12">
                <h1 class="text-xs-center">"You have been logged out."</h1>
              </div>
            </div>
          </div>
        </div>
    }
}

#[component]
fn SettingsPage(cx: Scope) -> impl IntoView {
    let logout_action = create_server_action::<Logout>(cx);
    // TODO: how do we make this just a POST and not a server action?
    create_effect(cx, move |_| {
        console_log("logout action run");
        if let Some(Ok(_)) = &logout_action.value().get() {
            let nav = use_navigate(cx);
            let _ = nav("/logged-out", Default::default());
        }
        ()
    });
    view! {cx,
    <Header />
    <div class="settings-page">
      <div class="container page">
        <div class="row">
          <div class="col-md-6 offset-md-3 col-xs-12">
            <h1 class="text-xs-center">"Your Settings"</h1>

            <form>
              <fieldset>
                <fieldset class="form-group">
                  <input class="form-control" type="text" placeholder="URL of profile picture" />
                </fieldset>
                <fieldset class="form-group">
                  <input class="form-control form-control-lg" type="text" placeholder="Your Name" />
                </fieldset>
                <fieldset class="form-group">
                  <textarea
                    class="form-control form-control-lg"
                    rows="8"
                    placeholder="Short bio about you"
                  ></textarea>
                </fieldset>
                <fieldset class="form-group">
                  <input class="form-control form-control-lg" type="text" placeholder="Email" />
                </fieldset>
                <fieldset class="form-group">
                  <input class="form-control form-control-lg" type="password" placeholder="Password" />
                </fieldset>
                <button class="btn btn-lg btn-primary pull-xs-right">"Update Settings"</button>
              </fieldset>
            </form>
            <hr />
            <ActionForm action=logout_action>
              <button class="btn btn-outline-danger">"Or click here to logout."</button>
            </ActionForm>
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
    let current_user = create_server_action::<GetCurrentUser>(cx);
    current_user.dispatch(GetCurrentUser {});

    view! {cx,
      <nav class="navbar navbar-light">
        <div class="container">
          <A class="navbar-brand" href="/">"conduit"</A>
          // TODO: "active" class to a when on page
          <ul class="nav navbar-nav pull-xs-right">

       <Show when=move || current_user.value().get().is_some() fallback=move |_| {view!{cx,

            <li class="nav-item">
              <A class="nav-link" href="/login">"Sign in"</A>
            </li>
            <li class="nav-item">
              <a class="nav-link" href="">"Sign up"</a>
            </li>
        }}>

            <li class="nav-item">
              <a class="nav-link active" href="">"Home"</a>
            </li>
            <li class="nav-item">
              <A class="nav-link" href="/settings"> <i class="ion-gear-a"></i>" Settings "</A>
            </li>
            <li class="nav-item">
            // TODO: this panics when there's no current user
              <span>"logged in as:" {current_user.value().get().unwrap().unwrap().unwrap().email}</span>
            </li>
            <li class="nav-item">
              <a class="nav-link" href=""> <i class="ion-compose"></i>" New Article "</a>
            </li>
       </Show>
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
pub struct LoginForm {
    pub email: Field<String>,
    pub password: Field<String>,
}

impl LoginForm {
    pub fn is_valid(&self) -> bool {
        self.email.errors.is_empty() && self.password.errors.is_empty()
    }
}

#[server(AttemptLogin, "/api")]
pub async fn attempt_login(
    cx: Scope,
    email: String,
    password: String,
) -> Result<LoginForm, ServerFnError> {
    let req = use_context::<actix_web::HttpRequest>(cx).unwrap();
    let sess = actix_session::Session::extract(&req).await.unwrap();
    let form = LoginForm {
        email: Field::required(Some(email)).trim().min_length(10).email(),
        password: Field::required(Some(password)).trim().min_length(10),
    };

    if form.is_valid() {
        sess.insert("user_email", form.email.clone().input.unwrap());
        Ok(form)
    } else {
        Ok(form)
    }
}

#[server(Logout, "/api")]
pub async fn logout(cx: Scope) -> Result<(), ServerFnError> {
    let req = use_context::<actix_web::HttpRequest>(cx).unwrap();
    let sess = actix_session::Session::extract(&req).await.unwrap();
    sess.clear();
    Ok(())
}

#[cfg(feature = "ssr")]
fn set_header(cx: &Scope, key: HeaderName, val: HeaderValue) {
    let res_options_outer = use_context::<leptos_actix::ResponseOptions>(*cx);
    if let Some(res_options) = res_options_outer {
        res_options.insert_header(key, val);
    }
}

#[server(GetCurrentUser, "/api")]
pub async fn get_current_user(cx: Scope) -> Result<Option<CurrentUser>, ServerFnError> {
    match use_context::<actix_web::HttpRequest>(cx) {
        Some(req) => Ok(CurrentUser::extract(&req).await.ok()),
        None => Ok(None),
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct CurrentUser {
    email: String,
}

#[cfg(feature = "ssr")]
use actix_web;

#[cfg(feature = "ssr")]
impl FromRequest for CurrentUser {
    type Error = actix_web::Error;
    type Future =
        std::pin::Pin<Box<dyn std::future::Future<Output = Result<CurrentUser, Self::Error>>>>;

    fn from_request(
        req: &actix_web::HttpRequest,
        pl: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let fut = actix_session::Session::from_request(req, pl);
        Box::pin(async move {
            if let Ok(sessions) = fut.await {
                if let Ok(Some(email)) = sessions.get("user_email") {
                    return Ok(CurrentUser { email });
                }
            };

            Err(actix_web::error::ErrorUnauthorized("unauthorized"))
        })
    }
}
