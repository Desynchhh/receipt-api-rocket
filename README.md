# Setup guide:
1. Create a `.env` file in the root of the project. The project requires the `.env` to include 2 keys: `DATABASE_URL` and `JWT_TOKEN_SECRET`.
2. Create a `Rocket.toml` file in the root of the project, and add a `secret_key` key with a (base64 encoded) string value to it. This is necessary due to the use of private cookies in Rocket rs.
3. Install the Diesel CLI tool. Run the command `cargo install diesel_cli --no-default-features --features "sqlite"` to do this.
4. Setup the database and Diesel. Do this by running the Diesel cli command `diesel setup` in the console.
5. Run the database migrations via the `diesel migration run -a` command.
6. There you go! The project should now be ready for continued development.
