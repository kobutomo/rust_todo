rundb:
	docker-compose -f ./dc/docker-compose.yml up
cleardb:
	docker-compose -f ./dc/docker-compose.yml down