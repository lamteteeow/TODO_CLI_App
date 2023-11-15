# This script using "click" is to compare with rust-todo app using "ncurses" in CLI
# run in terminal: "python3 python-todo.py" to see all options
# run in terminal: "python-todo.py --help" to check all available commands
# example: "python3 python-todo.py add-todo --name "burningList" b"

import click

PRIORITIES = {
    "o": "optional",
    "l": "low",
    "m": "middle",
    "h": "high",
    "i": "important",
    "b": "burning"
}


@click.group
def mycommands():
    pass


@click.command()
@click.option("--name", prompt="Enter user\'s name", help="User\'s name")
def greet(name):
    click.echo(f"Good day {name}!")


@click.command()
@click.argument("priority", type=click.Choice(PRIORITIES.keys()), default="m")
# default for argument is required=1
@click.argument("todofile", type=click.Path(exists=False), required=0)
@click.option("-n", "--name", prompt="Enter the todo name", help="The name of the todo item")
@click.option("-d", "--description", prompt="Describe the todo item", help="The description of the todo item")
def add_todo(name, description, priority, todofile):
    filename = todofile if todofile is not None else "mytodos.txt"
    with open(filename, "a+") as f:
        # write todofile at cwd
        f.write(f"[{PRIORITIES[priority]}] {name}: {description} \n")


@click.command()
@click.argument("idx", type=int, required=1)
@click.argument("todofile", type=click.Path(exists=True), required=1)
def delete_todo(idx, todofile):
    filename = todofile if todofile is not None else "mytodos.txt"
    with open(filename, "r") as f:
        todo_list = f.read().splitlines()
        todo_list.pop(idx)
    with open(filename, "w") as f:
        f.write("\n".join(todo_list))
        f.write("\n")


@click.command()
@click.option("-p", "--priority", type=click.Choice(PRIORITIES.keys()))
@click.argument("todofile", type=click.Path(exists=True), required=0)
def list_todo(priority, todofile):
    filename = todofile if todofile is not None else "mytodos.txt"
    with open(filename, "r") as f:
        todo_list = f.read().splitlines()
        if priority is None:
            for idx, todo in enumerate(todo_list):
                print(f"[{idx}] - {todo}")
        else:
            for idx, todo in enumerate(todo_list):
                if f"[Priority: {PRIORITIES[priority]}]" in todo:
                    print(f"[{idx}] - {todo}")


mycommands.add_command(greet)
mycommands.add_command(add_todo)
mycommands.add_command(delete_todo)
mycommands.add_command(list_todo)


if __name__ == "__main__":
    mycommands()
