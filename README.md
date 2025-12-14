# Mcospkg - A powerful package manager by a little guy ✨
Welcome to use mcospkg!

Now I wonder this project will available in both Windows and linux, and it is being popular soon 😏

Note that this project uses GPL-3.0 License, please follow the rules of that license, thanks!

For more infomation, please type `mcospkg-info` after the installation.

## Description
This project imagine by a 12-year-old boy (THE SAME PERSON) at first, cause there's less package manager by Chinese, so as a Chinese young man and developer, I, and my team, will take this mission on.

## Extra Documents
For more documents, please look at the directory `docs/`

如果你是中国人🇨🇳, 那么我们贴心地准备了翻译(在`docs/Chinese`下)

## Build & Install
If you have installed mcospkg, just run this command:
`sudo mcospkg update mcospkg`

To build it, ensure you had installed these applications(packages) on Linux:

 - Rust(stable, latest, with Cargo)
 - gcc/clang(with cc)
 - openssl & libssl-dev
 - pkg-config
 - git

For Windows platform, seems you just need to install MSVC (latest), and it will be compiled successfully.

After installing them, follow these steps:

1. Clone from repository

Run these commands:

```bash
git clone https://github.com/zhangxuan2011/mcospkg.git
cd mcospkg
```

2. Build
Run this command to build this project:

`cargo build --release -j8`

In this, you can specify the building jobs (In this example, Jobs = 8)

**NOTE**!!!! You must specify the argument `--release` otherwise you **CAN'T** do more steps in this building process. 

3. Install

Run the `install.sh` we've given:

`./install.sh`

This will install the mcospkg to `/` (Defined in `PREFIX`)

## Build Version Rules

In each updates, we may change the build version of the program.

There are 4 levels of the build updates:
 - Code Struct Changes (Build += 1),
 - Output Style Changes (Build += 2),
 - Feature Updates (Build += 3),
 - Important Feature Updates (Build += 4).

Now let me introduce the rules:
1. Each commit messages must mark the latest build numbers;
2. If 2 same levels' commit are next to each other, the build version only increases once, not twice.

Build numbers since: **9121 (v0.9.1)**.
