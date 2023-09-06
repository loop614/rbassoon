BASSOON_RUST := docker compose exec rbassoon_rust
BASSOON_SPACETIME := docker compose exec rbassoon_spacetime
BASSOON_SPACETIME_EXE := docker compose exec rbassoon_spacetime /clockwork/spacetime
BASSOON_SPACETIME_DB := repository

start:
	docker compose build
	docker compose up

nuke:
	docker compose down --volumes
	docker compose build --no-cache
	docker compose up --remove-orphans

spacetime_open:
	$(BASSOON_SPACETIME) /bin/bash

temp:
	$(BASSOON_RUST) echo $RUST_BACKTRACE

rust_open:
	$(BASSOON_RUST) /bin/bash

watch:
	$(BASSOON_RUST) cargo watch  -c -w src -x run

spacetime_init:
	$(BASSOON_SPACETIME_EXE) init --lang=rust /random

spacetime_publish:
	$(BASSOON_SPACETIME_EXE) publish --project-path spacetime $(BASSOON_SPACETIME_DB)

spacetime_send:
	$(BASSOON_SPACETIME_EXE) call $(BASSOON_SPACETIME_DB) send_message '["Hello world!"]'

spacetime_sql:
	$(BASSOON_SPACETIME_EXE) sql $(BASSOON_SPACETIME_DB) "SELECT * FROM Message"

spacetime_logs:
	$(BASSOON_SPACETIME_EXE) logs $(BASSOON_SPACETIME_DB)

per:
	$(BASSOON_SPACETIME) chmod a+rwx -R /spacetime
	$(BASSOON_RUST) chmod a+rwx -R /app
