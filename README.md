# `anna`

## Description

Anna is a simple trello-like CLI application written in the Rust programming language.

## Usage

	anna [entity] [action] ...

### Boards

#### Create

	anna board create [board-name] --description [description]

#### View

	anna board view [board-name]

#### Update

	anna board update [board-name] --name [new-board-name] --name [new-description]

#### Delete

	anna board delete [board-name]

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