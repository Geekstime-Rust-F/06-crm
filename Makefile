export:
	@pg_dump --username=postgres --password --host=host.docker.internal --table=export_table --data-only --column-inserts stats > data.sql
