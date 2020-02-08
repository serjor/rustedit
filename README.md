# RUSTEDIT, the unneeded reddit crate for Rust

Rusted is a extremely simple library to send links to reddit. It doesn't do too much else, actually.

# EXAMPLE (a sort of)
As this library doesn't do too much, if you are looking for a simple way to send links to reddit, you can use it as follows:

``` rust
use rustedit;

let mut access_token = String::new();

let at = rustedit::get_access_token(
    user,
    password,
    app_id,
    app_token,
);

if at.is_ok() == true {
    access_token = at.ok().unwrap().0;

    rustedit::submit(
        &access_token.to_string(),
        subreddit,
        link,
        title,
        description,
    );
}




```
