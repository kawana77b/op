# op

## description

`op` is a CLI tool that opens the path given as an argument in a specified filer or browser.

I created it because I wanted to use a common command on multiple platforms when using the desktop filer.

## what is this?

This is a very personal tool that I created because I wanted to use the default filer with common cross-platform commands.

I use it often because it is easy. I am satisfied.  
This is a very personal tool and I do not intend to actively maintain it as FOSS. I will accept some feedback, but may not actively reply. However, you will see it. Any feedback would be greatly appreciated. Folks are free in exchange for not holding me accountable. Please respect the license of the dependency.

## env

Windows, Mac, Linux

**Must be in a desktop environment.**

## install

If you can use the Rust environment, you can safely install it.

```bash
cargo install kawana77b-op
```

Or you can get the binary from the Release page.

Disclaimer: When you get binaries from the release page, please note that this tool is not specifically signed and may be detected by security scans such as Windows Defender. I take no responsibility or action for this issue.

## usage

Open the current directory with the OS prescribed filer:

```bash
op
```

Open the specified directory:

```bash
op ./foo/bar
```

Open the URL in the OS-defined browser:

```bash
op https://www.google.com
```

Open the remote repository in the git project in the prescribed browser:

```bash
op git
```
