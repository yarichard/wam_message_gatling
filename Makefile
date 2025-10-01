build:
GIT_TAG := $(shell git describe --tags --abbrev=7 --always 2>/dev/null || git rev-parse --short=7 HEAD)
IMAGE_NAME ?= wam_message_gatling:$(GIT_TAG)
DOCKERFILE ?= Dockerfile
CONTEXT ?= .

.PHONY: build run

build:
	docker build -t $(IMAGE_NAME) -f $(DOCKERFILE) $(CONTEXT)

WAM_SERVER_URL=http://localhost:3000
GATLING_MSG_NB=2
GATLING_MSG_SEC=5

run:
	docker run --rm \
		-e WAM_SERVER_URL=$(WAM_SERVER_URL) \
		-e GATLING_MSG_NB=$(GATLING_MSG_NB) \
		-e GATLING_MSG_SEC=$(GATLING_MSG_SEC) \
		$(IMAGE_NAME)
