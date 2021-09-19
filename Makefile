rundb:
	docker-compose -f ./dc/docker-compose.yml up -d
stopdb:
	docker-compose -f ./dc/docker-compose.yml stop
cleardb:
	docker-compose -f ./dc/docker-compose.yml down