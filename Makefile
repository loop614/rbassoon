BASSOON_SPACETIME := docker compose exec rbassoon_spacetime
BASSOON_SPACETIME_EXE := docker compose exec rbassoon_spacetime /clockwork/spacetime
BASSOON_SPACETIME_DB := repository

start:
	docker compose up --build

nuke:
	docker compose down --volumes
	docker compose build --no-cache
	docker compose up --remove-orphans

spacetime_open:
	$(BASSOON_SPACETIME) /bin/bash

temp:
	$(BASSOON_SPACETIME_EXE)

init:
	$(BASSOON_SPACETIME_EXE) init --lang=rust /server

publish:
	$(BASSOON_SPACETIME_EXE) publish --project-path server $(BASSOON_SPACETIME_DB)

call:
	$(BASSOON_SPACETIME_EXE) call $(BASSOON_SPACETIME_DB) send_message '["Hello world!"]'

sql:
	$(BASSOON_SPACETIME_EXE) sql $(BASSOON_SPACETIME_DB) "SELECT * FROM Message"

logs:
	$(BASSOON_SPACETIME_EXE) logs $(BASSOON_SPACETIME_DB)

per:
	$(BASSOON_SPACETIME) chmod a+rwx -R /server
