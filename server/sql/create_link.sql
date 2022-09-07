INSERT INTO linky.link (shortlink, longlink)
VALUES ($1, $2)
RETURNING $table_fields;