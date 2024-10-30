PREFIX=bondiano/ecs_helpers
VERSION_TAG=${PREFIX}:${VERSION}
LATEST_TAG=${PREFIX}:latest

build:
	docker build --build-arg ECS_HELPER_VERSION="-v ${VERSION}" . -t ${VERSION_TAG} -t ${LATEST_TAG}

push:
	docker push ${VERSION_TAG}

release: build push
