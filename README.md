# ECS HelpeRS

[![codecov](https://codecov.io/gh/bondiano/ecs_helpers/graph/badge.svg?token=BLQ31XSEO0)](https://codecov.io/gh/bondiano/ecs_helpers)

**ECS Helpers** - A tool for managing the deployment process of an application in Amazon Elastic Container Service (ECS).

Mostly drop-in replacement for [ecs_helper](https://github.com/dualboot-partners/ecs_helper/tree/master) written in Rust to provide better performance and less container size.

## Introduction

**ECS Helpers** is a command-line tool that allows you to control the deployment process of your application in Amazon Elastic Container Service. The tool provides various commands for building and pushing images, deploying your application, exporting images, logging in to Amazon Elastic Container Registry (ECR), running commands, exporting environment variables, and more. To use it an ECS Cluster with a service running there and have `task_definitons` is required. Docker images are stored in the ECR Elastic Container Registry.

## Installation

**Prerequisites**:

- [Docker](https://docs.docker.com/get-docker/)
- [Cargo](https://github.com/rust-lang/cargo)

### Using Cargo

To use **ECS Helpers**, you need to install the `ecs_helpers` crate. You can install it using the following command:

```bash
cargo install ecs_helpers
```

You can use the `ecs_helpers` command followed by the desired command and arguments to control your application deployment process.

### Using docker

Alternatively, you can use the Docker image `bondiano/ecs_helpers`. This image contains the `ecs_helpers` and Docker.

To use the Docker image, you can run the following command:

```bash
docker run bondiano/ecs_helpers ecs_helpers
```

## The available commands are

- **build_and_push**: builds and pushes the Docker image to Amazon Elastic Container Registry (ECR).
- **deploy**: deploys the Docker image to Amazon Elastic Container Service (ECS).
- **export_images**: exports Docker images to a file.
- **ecr_login**: logs in to Amazon Elastic Container Registry (ECR).
- **run_command**: runs a command in a container.
- **export_env_secrets**: exports environment variables to a file.
- **deploy**: deploys the Docker image to Amazon Elastic Container Service (ECS).

You can select the desired command by passing the argument to the `ecs_helpers` command. For example, to build and push an image with the tag api, you can use the following command:

```bash
ecs_helpers build_and_push --image=api
```

## Using in GitLab CI

**ECS Helpers** can also be used in GitLab CI by using a pre-built Docker image. Here's an example of how to use **ECS Helpers** in a GitLab CI pipeline:

```yaml
stages:
  - build
  - deploy

variables:
  DOCKER_DRIVER: overlay2
  DOCKER_TLS_CERTDIR: ''
  DOCKER_IMAGE: docker:20.10.6
  PROJECT: test_project
  ECS_HELPERS_IMAGE: bondiano/ecs_helpers:latest
  AWS_REGION: us-east-1
  AWS_DEFAULT_REGION: us-east-1
  APPLICATION: app
  PROJECT: project-name
  USE_IMAGE_TAG_ENV_PREFIX: "true"

.ci_deploy: &ci_deploy
only:
  - master
  - staging

build_app:
  <<: *ci_deploy
  stage: build
  image: $ECS_HELPERS_IMAGE
  script:
    - mkdir -p ./apps/api/dist/apps && cp -r ./dist/apps/api ./apps/api/dist/apps
    - ecs_helpers build_and_push --image=api --cache -d ./apps/api

deploy_app:
  <<: *ci_deploy
  stage: deploy
  image: $ECS_HELPERS_IMAGE
  variables:
  APPLICATION: app
  script:
    - ecs_helpers deploy
```

In this example, ECS Helpers is used to build and push the api Docker image in the `build_app` job, and to deploy the application in the `deploy_app` job.

When a new version of an application is deployed, a new task definition revision is created in the target service.
