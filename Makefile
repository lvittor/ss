build:
	DOCKER_BUILDKIT=1 docker build -f Dockerfile.python -t ss-python:latest .

run:
	@docker run -it --rm -v $(PWD)/TP1/scripts/:/project/scripts ss-python:latest poetry run python3 scripts/generate.py

run-interactive:
	@docker run -it --rm -v $(PWD):/app ss-python:latest
	