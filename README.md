# antr

antr is a simple to use and high performance file watcher.

antr watches the current directory and subdiretories for changes. On the event
of a change, it clears the screen and runs the passed command.

If the current directory is a git repository, antr will ignore changes to
git ignored files.

## Features

- Fast and responsive
- Simple usage, no configuration needed
- Respects git ignore
- Resource efficient
- Force a run by pressing Enter

## Usage

To run `make` every time there is a file change:

~~~sh
$ antr make
~~~

To run a single javascript file:

~~~sh
$ antr node file.js
~~~

A run can be forced by pressing Enter.

## Performance

A common issue with recursively watching a directory for changes is the need to
register watchers for every file, including those ignored by git. `antr`
addresses this by checking gitignore rules before setting up watchers,
resulting in improved speed and resource efficiency when working in git
repositories with many ignored files (e.g., node_modules, .direnv, target,
etc).
