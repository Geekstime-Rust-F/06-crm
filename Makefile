export:
	@pg_dump --username=postgres --password --host=host.docker.internal --table=export_table --data-only --column-inserts stats > data.sql
run_nginx:
	@docker run -it -d -p 8080:80 --name crm-nginx -v ~/Documents/study/Rust/camp/crm/nginx:/etc/nginx nginx
