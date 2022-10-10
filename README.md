# rocket-postgres-prototype

Environment variable:
ROCKET_DATABASES='{public={url=\"postgres://postgres:password@127.0.0.1:5432/postgres\"}}'

OR rocket.toml:

[default.databases.public]
url = "postgres://postgres:password@127.0.0.1:5432/postgres"
