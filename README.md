# Contribution Guide
This guide details the general rules to follow when working. Each project may have its own variation of this contribution guide.

> **_NOTE:_**  Never push code directly to the organization repo

## Initial Setup
1. Install git

2. Fork the project repo

3. Clone your fork on your local computer
```
git clone https://github.com/<your-username>/<repository-name>.git
```

4. Change working directory
```
cd <repository-name>
```

5. Add the upstream repository
```
git remote add upstream https://github.com/VIZZ-WEB-SOLUTIONS/<repository-name>.git
```

6. Now `git remote -v` should show 2 repositories.
	- `origin` Your forked repository
	- `upstream` Vizz Web Solutions Dev repository

7. install dependencies

## Develop your contributions

1. Pull the latest changes
```
git fetch --all
git pull --rebase upstream main
```

2. Create a branch for the changes you want to work on rather than working off of your local main branch:
```
// new branch name should relate to the features you are working on
git checkout -b <new-branch-name> upstream/main
```

3. Write some code!

## Submitting a pull request
1. git push -u origin `<new-branch-name>`

2. In order to make the team aware of your changes, you can make a PR to the `VIZZ-WEB-SOLUTIONS/<repository_name>` repository from your fork.

3. Ask other team members to review the code.

4. Resolve all merge conflicts.
