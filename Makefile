BASSOON_ES := docker compose exec rbassoon_es

nuke:
	docker compose down --volumes
	docker compose build --no-cache
	docker compose up --remove-orphans

start_es:
	docker compose up rbassoon_es --remove-orphans

start_rust:
	cargo run

es_health:
	wget localhost:9200/_cluster/health &> /dev/null

es_up:
	docker compose up rbassoon_es --remove-orphans

watch:
	cargo watch -w src -x run
