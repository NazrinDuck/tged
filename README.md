> 所选题目：赛道一 控制台交互 复杂版

# Tged

## 代码简介

源代码架构：

```txt
tged/src
├── color.rs
├── file.rs
├── lib.rs
├── main.rs
├── prelude.rs
├── screen.rs
├── settings.rs
├── terminal
│   ├── cursor.rs
│   ├── mod.rs
│   └── term.rs
├── view
│   ├── bottombar.rs
│   ├── filetree.rs
│   ├── help.rs
│   ├── mainview.rs
│   ├── menu.rs
│   ├── msgbox.rs
│   └── topbar.rs
└── view.rs
```

内部逻辑：

```txt
    Screen <---- Module
    |             |
    |         |----------------|----------|
    |       FileMod        Settings     Term
    |         |                |          |
  Views       |              Theme     Cursor
    |        Files
    |         |
Content <--  Content
```

- color.rs: 控制颜色界面相关的代码
- file.rs: 文件系统代码，提供后端服务
- lib.rs: 类属性宏代码
- main.rs: 程序入口，初始化程序，多线程监听事件并调用对应函数
- prelude.rs: 引入必要的模块，方便组件开发
- screen.rs: 显示系统代码，包含显示/处理键盘事件等前端功能，与后端对接
- settings.rs: 设置相关，包含显示设置和主题颜色设置内容
- terminal: 终端相关内容
  - cursor.rs: 提供控制光标行为的接口
  - term.rs: 记录终端大小，提供获取终端大小的接口
- view: 各种模块
  - bottombar.rs: 底部状态栏，显示文件名，文件大小和目前视窗
  - filetree.rs: 左侧文件树，可以交互打开文件
  - help.rs: 内置帮助文档
  - mainview.rs: 主显示界面，负责主要交互，提供保存，查找替换功能
  - menu.rs: 顶部状态栏，显示一些有用的信息，也可以键入命令
  - msgbox.rs: 弹窗输入框，方便直接输入内容并返回给程序
  - topbar.rs: 顶部文件条，显示所有打开的文件并高亮目前的文件
- view.rs: 定义共有trait，提供接口

## 运行环境

> 无必须环境变量

- 测试系统：

简短信息：

```
❯ uname -a
Linux NazrinDuck 6.13.3-arch1-1 #1 SMP PREEMPT_DYNAMIC Mon, 17 Feb 2025 17:42:11 +0000 x86_64 GNU/Linux
```

详细信息：

`````
❯ fastfetch
                  -`                     NazrinDuck@NazrinDuck
                 .o+`                    ---------------------
                `ooo/                    OS: Arch Linux x86_64
               `+oooo:                   Host: 21LF (ThinkBook 14 G6+ AHP)
              `+oooooo:                  Kernel: Linux 6.13.3-arch1-1
              -+oooooo+:                 Uptime: 5 hours, 22 mins
            `/:-:++oooo+:                Packages: 1710 (pacman)
           `/++++/+++++++:               Shell: fish 3.7.1
          `/++++++++++++++:              Display (H27T13): 2560x1440 @ 60 Hz in 24" [External]
         `/+++ooooooooooooo/`            Display (LEN8AB1): 3072x1920 @ 60 Hz (as 1536x960) in 14" [Built-in] *
        ./ooosssso++osssssso+`           DE: KDE Plasma 6.3.1
       .oossssso-````/ossssss+`          WM: KWin (Wayland)
      -osssssso.      :ssssssso.         WM Theme: Utterly-Round-Dark
     :osssssss/        osssso+++.        Theme: Breeze (Amethyst) [Qt], Breeze-Dark [GTK2], Breeze [GTK3/4]
    /ossssssss/        +ssssooo/-        Icons: Tela-circle-dark [Qt], Tela-circle-dark [GTK2/3/4]
  `/ossssso+/:-        -:/+osssso+-      Font: Noto Sans (10pt) [Qt], Noto Sans (10pt) [GTK2/3/4]
 `+sso+:-`                 `.-/+oso:     Cursor: Sweet (24px)
`++:.                           `-/+/    Terminal: konsole 24.12.2
.`                                 `/    Terminal Font: FiraCode Nerd Font (11pt, Medium)
                                         CPU: AMD Ryzen 7 8845H (16) @ 5.14 GHz
                                         GPU: AMD Phoenix3 [Integrated]
                                         Memory: 7.68 GiB / 27.21 GiB (28%)
                                         Swap: 165.95 MiB / 8.00 GiB (2%)
                                         Disk (/): 137.63 GiB / 248.00 GiB (55%) - btrfs
                                         Disk (/home/NazrinDuck/data): 17.83 GiB / 195.50 GiB (9%) - btrfs
                                         Local IP (wlp4s0): 10.194.253.27/16
                                         Battery (L23N4PG1): 100% [AC Connected]
                                         Locale: zh_CN.UTF-8
`````

- 操作系统：
  Linux各种发行版（未广泛测试）

- 编程语言版本：

Rust 1.84.1

```
❯ cargo -V
cargo 1.84.1 (66221abde 2024-11-19)
```

- 其他
  建议使用等宽Nerd系列字体

## 操作说明

- 视图说明：
  - 主视图(MainView)：屏幕主要部分，显示当前文件的内容，提供增删查改和保存功能
  - 文件树(FileTree)：屏幕左边部分，列出文件/文件夹，可以回车打开文件/文件夹
  - 菜单(Menu)：屏幕顶部，显示一些有用的信息，也可以键入命令
  - 顶部状态栏(TopBar)：屏幕第二行，列出所有的文件名并高亮当前文件
  - 底部状态栏(BottomBar)：屏幕最底部，显示文件名，文件大小和目前视窗
- 键盘事件：
  - `<F1>`～`<F5>`按键被保留作固定功能
    - `<F1>`: 打开/关闭帮助
    - `<F2>`: 聚焦至主视图
    - `<F3>`: 聚焦至文件树
    - `<F4>`: 聚焦至菜单
    - `<F5>`: 顺序切换视图
  - 在主视图时
    - 通过方向键移动光标，键盘输入字符
    - 键入`<F6>`顺序切换当前文件
    - 键入`<F7>`逆序切换当前文件
    - 键入`<F8>`根据输入切换文件
    - 键入`<Alt+Left>`/`<Alt+Right>`改变主视图大小
    - 键入`<Ctrl+s>`保存当前文件，若没有名字则会有弹窗来输入
    - 键入`<Ctrl+f>`开启查找模式，输入字符串后通过方向键来定位所有匹配项
    - 再次键入`<Ctrl+f>`可以开启替换模式，输入要替换的内容并回车完成替换
  - 在文件树时
    - 通过方向键移动光标
    - 键入`<Enter>`打开文件/文件夹
  - 在菜单时
    - 通过方向键移动光标，键盘输入命令
    - 键入`<Enter>`提交命令

菜单命令：

| 命令 | 描述         |
| ---- | ------------ |
| quit | 退出程序     |
| save | 保存当前文件 |
