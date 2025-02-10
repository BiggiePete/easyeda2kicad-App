# PREPARING DB

install diesel CLI if need be

cd into tauri-src dir and run `diesel migration generate --diff-schema create_posts` to create a new migration

if you are trying to run with existing migrations, run this:
the to apply migrations, run `diesel migration run`
