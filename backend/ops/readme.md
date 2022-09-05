## Database Migrations

This can actually be needed later during development for running database migration, 
If newer database changes are introduced during development, these can be applied in two ways:

a) By running `./ops/update_db.sh` from `backend` location.<br/>

b) Or being very explicit and running:
   ```shell
   ❯ export DATABASE_URL=postgres://fs_rs_rwa:fs_rs_rwa@localhost:5441/fs_rs_rwa
   ❯ sqlx migrate run
   ``` 

If newer migrations are introduced (using `sqlx migrate add {some_name}`) after the initial database init (and its migration),
you can update the database model by running `./ops/update_db.sh`, as described before in option a).
