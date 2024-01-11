# `anna`

## Description

Anna is a simple trello-like CLI application written in the Rust programming language.

## Usage

	anna [entity] [action] ...

### Boards

#### Create

	anna board create --name <BOARD_NAME> --description <DESCRIPTION>

#### View

	anna board view --id <BOARD_ID>

#### Update

	anna board update --id <BOARD_ID> --name <BOARD_NAME>  --description <DESCRIPTION>

#### Delete

	anna board delete --id <BOARD_ID>

## Todos

- [ ] CRUD for boards
    - [ ] Create a board
	- [ ] View a board
    - [ ] Update a board
    - [ ] Delete a board
- [ ] CRUD for tasks
    - [ ] Create a task
	- [ ] View a task
    - [ ] Update a task
    - [ ] Delete a task