# Task

Query CLI offers an API that enables users to execute custom commands defined in the `Query.toml` file.

The `Query.toml` file should have the following structure to define the tasks:

```toml
[task]
task_1 = "echo 1"
task_2 = "echo 2"

[task.dev]
dev_1 = "echo dev 1"
dev_2 = "echo dev 2"

[task.bundle]
bundle_1 = "echo bundle 1"
bundle_2 = "echo bundle 2"
```

Usage:

```sh
query task [OPTIONS] [TASK] [SUBTASK]
```

Arguments:

- `[TASK]` - Name of the task to execute
- `[SUBTASK]` - Name of the subtask to execute

Options:

- `-l, --list` - List all the tasks
- `-y, --yes` - Confirm the execution of the task
- `-h, --help` - Print help

To execute a simple task, you have to run the following command:

```sh
query task task_1
```

To execute a task with a subtask, you have to run the following command:

```sh
query task dev dev_1
```

Executing a task with subtasks it will execute all the subtasks.

```sh
query task dev # It will execute dev_1 and dev_2
```

It will ask you to confirm the execution of the task. If you want to avoid the confirmation, you can use the `-y` option.

```sh
query task dev -y
```

To list all the tasks and subtasks, you have to run the following command:

```sh
query task -l
```

To list the subtasks of a task, you have to run the following command:

```sh
query task dev -l
```

> **Important!**
> On dev mode, the **dev** task will be executed before the default commands (**function**, **asset dist** and **asset public**).
