FROM python:3.9.12-slim-buster

RUN apt-get update && apt-get install -y libsqlite3-dev build-essential libcairo2-dev libpango1.0-dev ffmpeg texlive-full

ENV PATH="/root/.local/bin:$PATH"

RUN pip install pipx
RUN pipx install poetry
ENV PATH="/root/.local/pipx/venvs/poetry/bin/:$PATH"

COPY pyproject.toml /project/pyproject.toml
COPY poetry.lock /project/poetry.lock

WORKDIR /project

RUN --mount=type=cache,target=/root/.cache/pip \
    poetry install --no-root

COPY . /project

RUN --mount=type=cache,target=/root/.cache/pip \
    poetry install --no-interaction