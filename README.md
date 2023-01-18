<div align="center">
  <br>
  <img src="https://raw.githubusercontent.com/edfloreshz/done/main/data/icons/dev.edfloreshz.Done.svg" width="150" />
  <h1>Done</h1>
  <h3>To-do lists reimagined</h3>
  
  <a href="https://github.com/edfloreshz/done/actions/workflows/ci.yml">
    <img src="https://img.shields.io/github/actions/workflow/status/edfloreshz/done/ci.yml?style=for-the-badge" alt="build"/>
  </a>
  <a href="https://crates.io/crates/done">
    <img src="https://img.shields.io/crates/v/done?label=Done&style=for-the-badge" alt="crate"/>
  </a>
   <a href="https://crates.io/crates/done">
    <img src="https://img.shields.io/crates/d/done?style=for-the-badge" alt="downloads"/>
  </a>
  <br/>
  <a href="https://github.com/sponsors/edfloreshz">
    <img src="https://img.shields.io/badge/sponsor-30363D?style=for-the-badge&logo=GitHub-Sponsors&logoColor=#white"/>
  </a>
  <a href="https://matrix.to/#/#done-gh:matrix.org">
    <img src="https://img.shields.io/badge/matrix-000000?style=for-the-badge&logo=Matrix&logoColor=white"/>
  </a>
  <a href="https://github.com/edfloreshz/done">
    <img src="https://img.shields.io/badge/GitHub-100000?style=for-the-badge&logo=github&logoColor=white"/>
  </a>
  <a href="https://t.me/done_gh">
    <img src="https://img.shields.io/badge/Telegram-2CA5E0?style=for-the-badge&logo=telegram&logoColor=white"/>
  </a>
  <h4>This is still in very early development. Be aware it is a work in progress and far from complete yet.</h4>
  
</div>
<br/>

Done, the ultimate task management solution for seamless organization and efficiency. 

Our user-friendly app allows you to effortlessly consolidate your existing task providers into a single application for optimal productivity and organization. 

<div align="center">
  <img src="https://raw.githubusercontent.com/edfloreshz/done/81ea1f6d32cd491d1893f9ba730f511bc1cb0aea/data/resources/screenshots/tasks.png"/>
</div>



## Install
| Platform   | Command                                 |
|------------|-----------------------------------------|
| Arch Linux | `paru -S done-git`                    |
| Flathub    | <a href="https://flathub.org/apps/details/dev.edfloreshz.Done"><img src="https://flathub.org/assets/badges/flathub-badge-en.png" width="150"/></a> |

## Plugins
In order to realize its full potential, the Done app has been designed with a strong focus on versatility, flexibility and extensibility.

This is why we have implemented a plugin system, allowing for the addition of new task services, making it the go-to choice for 
anyone looking for a comprehensive and complete to-do list solution.

To get started creating plug-ins, head to [`PLUGINS.md`](PLUGINS.md).

## To do

### Accounts

- [ ] Allow multiple providers (Google, Microsoft To Do, Microsoft Exchange, Todoist, Nextcloud)

### Lists

- [x] Show lists
- [x] Add a new list
- [x] Delete an existing list
- [x] Rename an existing list
- [x] Update task counters

### Smart Lists
- [x] Today
- [x] Next 7 Days
- [x] All
- [x] Starred

### Tasks
- [x] Add a new task
- [x] Show tasks for every list
- [x] Mark a task as completed
- [x] Delete a task
- [x] Rename a task
- [ ] Add steps
- [ ] Add tags
- [ ] Add to My Day
- [x] Mark as Favorite
- [ ] Add notes

### Reminders
- [ ] Set a reminder
- [ ] Set a due date
- [ ] Set recurrence for a task

### Notifications
- [ ] Send notifications

### Backups
- [ ] Export tasks

## Dependencies to build
- gtk4
- libadwaita
- pkg-config

Ubuntu 22.04:
```bash
sudo apt install libadwaita-1-dev libgtk-4-dev libsqlite3-dev
```
Arch Linux:
```bash
sudo pacman -S libadwaita gtk4 sqlite
```

## Debug
To enable logging set `RUST_LOG` to `done=info`
```bash
RUST_LOG=done=info
```

Copyright and licensing
-----------------------

Copyright 2022 © Eduardo Flores

Done is released under the terms of the [Mozilla Public License v2](https://github.com/edfloreshz/done/blob/main/LICENSE)
