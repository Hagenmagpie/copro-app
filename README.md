# ⚡ COPRO 协同耕种


## 1. 项目简介

🌱`copro`  是coproduction词的前5个字母，表示协同作业的开始。

有些英文词典把该单词翻译为"粪"，既然这样，那就“粪”吧。 想起《道德经》第39章的一段话：
```
🎷 天下有道，却走马以粪。
```
这句话可以理解为 —— 天下合乎"道"的时候，战马却到田间给农夫用来耕种。
就是要把这个粪字的意义发挥到极致，那么就开始`耕种`吧💪。

💥所以 `copro` 大致理解为`协同耕种`✨的意思吧。

💧这里的人都是耕种者（coproer）👪，耕种的是任务，耕种的是梦想💨。

## 2、项目目标

通过该系统可把一个整体任务拆解成多个子任务，每个子任务可以继续拆解，最终拆解到可以落地的粒度。
然后可以分配给不同的人员，每个人员可以查看自己的任务，完成任务后可以标记任务状态。
也可以通过设置任务状态，不同人员进行领取。
每个任务可以设置审核人员，或指定审核人员数量由系统随机分派。
每位完成任务的人员可以获得相应的积分、审核的人员可以获取相应的等级勋章。
积分可以用来兑换奖品（奖品形式待定），等级勋章可以用来承接更高级别的任务。

## 3、项目功能
实现用户注册、登录、任务发布、任务领取、任务完成、任务审核、积分兑换、等级勋章等功能。

- 用户注册
- 用户登录
- 任务发布
- 任务领取
- 任务完成
- 任务审核
- 积分兑换
- 等级勋章


## 4、🌾编译运行

### 4.1 本地运行

```
cargo run --release
```

### 4.2 使用Trunk来构建Web目标 

#### 4.2.1 安装 trunk 命令

**检查rust版本**

安装trunk命令，需要rustc 1.74+ 版本 。
查看rust版本 rustc --version 或者  rustc -V 。
rustc 如果低于1.74版本可通过下面命令进行更新

```
#更新都当前稳定版本
rustup update stable 
#或者指定某个版本
rustup update 1.77.1
```

更新后确认项目中2个位置是否更新到最新版本。

1、确认 rust-toolchain文件 中channel对应的版本号

```
[toolchain]
channel = "1.77.1"
```

2、确认Carog.toml文件中对应的版本号

```
[package]
rust-version = "1.77.1"
```

**开始安装 trunk 命令**🍷

```
cargo install --locked trunk
```
*如果安装时报错 Blocking waiting for file lock on package cache， 删除 ~/.cargo/.package-cache ，然后再次执行命令*

#### 4.2.2 trunk 运行服务

运行 `trunk serve` 命令以构建并服务于 http://127.0.0.1:8080 如果您编辑项目，Trunk 将自动重建。

```
trunk serve
```
🌔在浏览器中打开 http://127.0.0.1:8080/index.html#dev 。

注：assets/sw.js脚本将尝试缓存我们的应用程序，并在无法连接到服务器时加载缓存版本，从而允许您的应用程序离线工作（如 PWA）。附加#dev到index.html将跳过此缓存，允许我们在开发过程中加载最新版本。

#### 4.2.3 trunk 编译 html

```
trunk build --release
```
🍓它将生成一个dist目录作为“静态 html”网站。





## 参考链接
- 表情符号列表 https://github.com/Hagenmagpie/markdown-emojis
- eframe https://github.com/emilk/eframe_template/
- egui https://www.egui.rs/