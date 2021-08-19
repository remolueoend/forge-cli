# forge-cli

```
forge 0.1
remolueoend

CLI for editing and creating issues and merge requests on Gitlab, Github, et al.

The most prominent commands of this tool allow you to edit and create MRs and issues in your local text
editor. They also offer you to edit and create MRs and issues in ORG mode instead of markdown by automatically
converting your input before upload.

Most parameters can be passed as environment variables instead of CLI arguments, allowing you to store these
values locally per project in an .envrc/.env file (As long as you're not pushing FORGE_CLI_TOKEN, you're
fine).

USAGE:
    forge [FLAGS] -p <project-path> -t <token> [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -v               Sets the level of verbosity
    -V, --version    Prints version information

OPTIONS:
    -p <project-path>        The path of the current project, e.g. 'myusername/myproject' or
                             'mygroup/myproject' [env: FORGE_CLI_PROJECT_PATH=]
    -t <token>               The authentication token to be sent as Bearer token to the remote forge. [env:
                             FORGE_CLI_TOKEN=]

SUBCOMMANDS:
    create-issue    
                    Opens a new empty buffer in the system text editor and uploads the content to the
                    remote host as a new issue after the editor is closed.
                    The format used is similar to git commits:
                    1. The first line of text is regarded as issue title
                    2. There must be an empty line between the title and the body.
                    3. All following lines are uploaded as issue description.
                    
                    When called with the -o flag, the input is expected to follow the ORG format and is
                    automatically converted to markdown before upload.
    edit-mr         
                    Guesses the merge request related to the currently checked out branch (if no branch
                    specified via -b) and then will:
                    1. download its description into a temporary file,
                    2. open the system editor to edit the merge request description,
                    3. update the merge request on the remote host.
                    
                    When called with the -o flag, the downloaded markdown description is converted to ORG
                    and back to markdown before and after editing it.
    help            Prints this message or the help of the given subcommand(s)
    open-mr         
                    Opens the MR of the currently checked out branch in your default browser.
                    The branch can be overwritten using the --branch flag.

```