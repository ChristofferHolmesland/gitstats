# gitstats
A rust CLI program to count your contributions to local git repositories

## Usage
```bash
./gitstats
```

Arguments
- file_path: root folder to scan for repositories. Default value: where the program is executed.
- name: committer name to count commits for. Default value: result of `git config --get user.name`.
- email: committer email to count commits for. Default value: result of `git config --get user.email`.

