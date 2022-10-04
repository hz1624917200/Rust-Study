# Rust - Introduction

> &emsp;&emsp;本文对应《Rust权威指南》的第1-2章，仅精简到特性、未接触部分。本系列文章主要作为笔者读书笔记备份使用，若读者感兴趣，建议阅读原书。

## Hello world

```rust
fn main() {
    println!("Hello, world!");
}
```

* 函数名后没有叹号`!`，宏后面加叹号`!`以与普通函数区分



## Cargo

Cargo是Rust工具链中内置的构建系统及**包管理器**。在构建大型系统时，可以使用Cargo辅助管理依赖。

### 创建项目

使用Cargo来创建一个项目。

```
cargo new hello_cargo
cd hello_cargo
```

会创建一个项目目录，包含初始化的配置文件等，并创建一个初始git仓库。

Cargo使用**TOML (Tom's Obvious, Minimal Language)**作为标准配置文件

### 主要命令

* `cargo run` 运行项目（包含编译）
* `cargo build` 编译/生成项目
    * `--release` 在release下优化编译
* `cargo check` **不需要编译，检查代码是否能通过编译**



# Guessing Game

> A Tutorial project, the first complete program

## Dependency

通过在`cargo.toml`内加入具体的依赖包和版本号，完成依赖加入

* 一般从`crates.io`中获取包
* 具体发行版本号可以在上述网站中查询
* `cargo.toml`默认使用适配匹配，即会下载适配当前版本的最新包

### `Cargo.lock` 保证可重现性

为了保证在开发机器和部署机器上完全一致的表现，cargo使用`cargo.lock`锁定包的具体版本。

* 当一个依赖被添加到`cargo.toml`中并*构建（Build）*后，cargo会将使用的具体版本加入`cargo.lock`，以在不同环境中匹配相同包版本
* 使用`cargo update`更新缓存，绕过`cargo.lock`直接从远端拉取包版本，覆盖原锁

## Including

### Module

> 模块为语言内置，主要用于处理部分核心功能

使用use语句模块导入命名空间

```rust
use std::io

fn main() {

    let mut input;
    io::stdin().read_line(&mut input);
}
```

如果不使用use，则需要写出整个对象位置`std::io::stdin`

### Crate (Trait 特性/特质)

`trait` impot a group of functions from a crate. 在crate中，一些具有类似功能的函数会被组合在一起，成为一个`trait`。通过引入`trait`，就可以使用对应特性的功能。

```rust
use rand::Rng;

...
rand::thread_rng().gen_range(1, 101);	// old version
rand::thread_rng().gen_range(1..=100);	// new version rand(0.8.5)
```



## variable & const

使用`let`关键字声明变量。变量声明后，可以在其作用域的任何地方使用。

### 不可变变量 unmutable variable

**在rust中，变量默认不可变。**除了**声明时**可以为变量初始化外，**其余任何位置对变量的写入都是非法的**。

（*在其他语言中，这将导致使用不方便（不得不声明一个新的变量以完成下一步操作）。但是，rust提供Shadow 特性，使变量名可重用。这将在后面提及*）

### 可变变量 mutable variable

这是其他语言中关于“变量”的概念。可变变量允许在其他位置修改变量的内容。

使用`mut` 关键字声明可变变量（仍需要`let`，这是声明语句的关键字）

```rust
let mut mutable_var;
```



## 使用crate rand

> 我们需要使用rand包中的`gen_range`函数，它被包含在`Rng`特性中，需要提前在声明中引用。

### import crate

在`Cargo.toml`中的Dependencies条目下，加入需要引入的`rand`包版本，这里使用最新版`0.8.5`

可以在`crates.io`中直接复制对应toml

```rust
rand = "0.8.5"
```

下次`build`时，cargo会自动更新库，并抓取对应适配版本的`rand`包

### Import Rng

在文件开头加入：

```rust
use rand::Rng;
```

现在，可以使用该特性下定义的所有相关功能的函数。

### use

**Attention: **此处由于包版本变化，**接口与书中所述不同**。

新版本加入`SampleRange`类型，位于`Trait rand::distributions::uniform::SampleRange`

可以从文档\提示中找到Range的填充方法，共有两种:

* 左闭右开：`a..b`
* 闭区间：`a..=b`

```rust
let secret_rand = rand::thread_rng().gen_range(1..=100);
```

**ps: rust文档系统非常完善，可以通过文档快速了解包具有哪些特性，有哪些函数**，这可以避免大量的重复造轮子。可以通过在线/离线（仍然需要下载数据）的方式查看文档。离线查看命令：

```shell
cargo doc --open
```



## print/println macro

`marco`将以`!`结尾，具体特性会在后面介绍。

* `println` 宏将在末尾输出`\n`，这将触发缓冲区清空，将输出整行内容
* `print` 宏直接输出原本内容，**不会触发缓冲区清空**，内容不会直接显示

### Formatting specifier

使用`{}`作为格式化字符串中的占位符，在后面增加参数，会将参数内容放入并输出

* 必须一一对应，否则将在编译阶段(check)报错
* 具体格式规范将在后面提到



## Compare - match

在本例中，我们使用`match`分支结构实现比较。**match分支具有高级的Pattern匹配机制，可以方便地处理所有分支（arm）**。另外，*match会被检查，确保所有的分支都被覆盖*。

```rust
match guess.cmp(&secret_rand) {
        Ordering::Less => println!("Too small!"),
        Ordering::Equal => {
            println!("You win!");
        }
        Ordering::Greater => todo!()
    }
```

* 分支之间使用`,`分隔（若包含语句块，则不需要分隔，**单个语句需要**）
* 使用default `_`来匹配所有的分支。（**分支匹配顺序从上到下，`_`类似于C/C++中的default**）

### Enum Ordering 

Ordering 是`cmp`函数返回的枚举类型。共有三种变体

```rust
pub enum Ordering {
    /// An ordering where a compared value is less than another.
    #[stable(feature = "rust1", since = "1.0.0")]
    Less = -1,
    /// An ordering where a compared value is equal to another.
    #[stable(feature = "rust1", since = "1.0.0")]
    Equal = 0,
    /// An ordering where a compared value is greater than another.
    #[stable(feature = "rust1", since = "1.0.0")]
    Greater = 1,
}
```

使用以下三种变体可以对所有分支进行匹配。



## 类型推导

一般情况下，Rust自动类型推导机制会识别函数的返回类型，并将类型和值(immutable)绑定在变量上。

* **当一个函数涉及泛型时，将无法完成自动推导**。但是，rust是强类型语言，必须指定类型，**此时需要手动指定类型**，或使用**turbofish`<>`**完成指定

```rust
let guess: u32 = guess.trim().parse()	// type annotation
	.expect("Parse error");
let guess = guess.trim().parse::<u32>()	// turbo fish
	.expect("parse error");
```



## Conceptions

* **assosiated function 关联函数**： 表示与类有关联关系的函数，相当于其他语言中的静态方法`static method`
* `string::new()`函数，用于构造一个初始类实例。在很多类中都有`new`方法，**是创建类实例的惯用函数名**
* **`result`类型**：用于一些函数的返回值。是一个**枚举类型(enumerate)**
    * `Ok`: 当前操作执行成功，并附带返回值
    * `Err`: 当前操作执行失败，附带失败原因
        * `Err`变体可以通过`expect()`函数快速捕获
    * **一般需要使用Result对象，如果对Result对象不加处理，编译器会产生警告**。这从代码编写层面增强了程序的安全性。

