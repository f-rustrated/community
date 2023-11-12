include .env
export $(shell sed 's/=.*//' .env)

upgrade:
	sqlx migrate run --database-url $(DATABASE_URL)

downgrade:
	sqlx migrate revert --database-url $(DATABASE_URL)
