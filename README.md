# antr

antr is a file system events watcher that runs arbitrary commands.

antr watches the current directory and subdiretories for changes. On the event
of a change, it clears the screen and runs the passed command.

If the current directory is a git repository, antr will ignore changes to
git ignored files.

# Usage

To run `make` every time there is a file change:

~~~sh
$ antr make
~~~

To run a single javascript file:

~~~sh
$ antr node file.js
~~~
