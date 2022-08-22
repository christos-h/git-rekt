## Git Rekt

Are you self-conscious about your low productivity? Are recruiters screening you
because GitHub says you haven't been writing much code? Well screw them!

Git-Rekt creates a git repository and creates a bunch of historical commits
going back 5 years. You can then push this repository to GitHub and look like a
100x ninja rockstar dev!

### Installation

Clone the repo and then use the Rust build system `cargo` to install the binary:

```bash
cd repo && cargo install --path .
```

### Usage

Create an empty git repo and then run `git-rekt`.

```bash
git init <path>
git-rekt <path> --username <gh-username> --email <gh-email> 
```

The last step is to push your repository to GitHub. The actual changes to your
activity may take a little time to show up. 