# ECS HelpeRS

[![codecov](https://codecov.io/gh/bondiano/ecs_helpers/graph/badge.svg?token=BLQ31XSEO0)](https://codecov.io/gh/bondiano/ecs_helpers)

Mostly drop-in replacement for [ecs_helper](https://github.com/dualboot-partners/ecs_helper/tree/master) written in Rust to provide better performance and less container size.

## Installation

**Prerequisites**:

- [Docker](https://docs.docker.com/get-docker/)

## The available commands are

- **build_and_push**: builds and pushes the Docker image to Amazon Elastic Container Registry (ECR);
- **export_images**: exports Docker images to a file.
- **ecr_login**: logs in to Amazon Elastic Container Registry (ECR).
- **run_command**: runs a command in a container.
- **export_env_secrets**: exports environment variables to a file.
