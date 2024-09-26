# antr

antr is a file system events watcher that runs arbitrary commands.

antr watches the current directory and subdiretories for changes. On the event
of a change, it clears the screen and runs the passed command.

If the current directory is a git repository, antr will ignore changes to
git ignored files.

antr is inspired mainly be [entr](http://entrproject.org/), with a few
differences:

- Easier usage
- Only works with the current directory
- Easier to watch directories for new files
- Integration with git ignore

# Usage

To run `make` every time there is a file change:

~~~sh
$ antr make
~~~

To run a single javascript file:

~~~sh
$ antr node file.js
~~~
