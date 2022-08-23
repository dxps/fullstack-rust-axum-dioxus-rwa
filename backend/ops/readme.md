## Database Migrations

This can actually be needed later during development for running database migration, 
If newer database changes are introduced during development, these can be applied in two ways:
a) By running `SKIP_DOCKER=true ./ops/init_db.sh` (from the project root location);
b) Or being more explicit by running:
   ```shell
   ❯ export DATABASE_URL=postgres://fs_rs_rwa:fs_rs_rwa@localhost:5441/fs_rs_rwa
   ❯ sqlx migrate run
   ``` 
