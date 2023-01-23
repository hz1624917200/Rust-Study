# 8 - Error handling 错误处理

> &emsp;&emsp;Rust提供了较为独特的错误处理机制。不同于C/C++的通用异常处理，Rust将可恢复与不可恢复错误区分对待。对于可恢复错误，Rust定义了`Result<T, E>`类型作为函数返回值，以便程序后续能够特殊处理。对于不可恢复错误，Rust程序会立即终止运行。



## 不可恢复错误与 panic

&emsp;&emsp;Rust通过`panic!`宏提供不可恢复错误的处理方案。在运行到`panic!`宏位置时，程序会终止运行，并进行`栈展开`（回溯调用栈，并依次清理调用栈中函数的数据）。最后，程序退出并将调用栈返回给用户。（通过设置`RUST_BACKTRACE`环境变量启用）

&emsp;&emsp;但是，栈展开需要额外的空间和效率开销，如果需要高效能，紧密型发布，可以将程序对`panic`的处理改为`abort`。如下，在`cargo.toml`中，加入：

```toml
[profile.release]
panic = 'abort'
```



## 可恢复错误与 Result

&emsp;&emsp;但是，大部分错误不需要“终止程序运行”这样的粗暴处理方式，而是可以通过一些动作修复的。例如，如果找不到文件，可以通过创建一个新文件的方式解决。对于这些**可恢复错误**，Rust提供`Result`类型专门支持处理。

```rust
enum Result<T, E> {
   Ok(T),
   Err(E),
}
```

其中包含两个**泛型参数**：

* `T`: Type 返回结果类型，如果正确，则Result类型将返回`Ok(Type)`类型变体
* `E`: Error 返回错误类型，如果程序运行错误，则将返回`Error(E)`类型变体

例如，如下打开文件代码将返回一个类型为`enum2$<core::result::Result<std::fs::File,std::io::error::Error> >`的变量

```rust
use std::fs::File;

fn main() {
    let f = File::open("test.txt");
}
```

* 如果文件存在，且正确被打开，则变体类型为`std::fs::File`
* 如果文件不存在，或遇到其他问题，则会返回`std::io::error::Error`类型的变体

### 对Result进行解包

有多种方法对Result结果进行解包，最基础的一种是之前使用的`except`

```rust
let f = File::open("text.txt").expect("Open file error");
```

该方法会将`Ok()`变体的Inner Type解包为变量值，同时对遇到的`Error`变体进行`panic`

同样的，使用`unwrap`方法的效果与except完全相同，**但是`unwrap`函数提供了自定义错误提示消息，`expect`则没有**。同时`unwrap`还提供一些方法变体，如`unwrap_or`等，提供解包时的默认值或替代方案等做法。

这里对两个`unwrap`族函数进行区分：

```rust
unwrap_or();		// eagerly evaluated
unwrap_or_else();	// lazily evaluated
```

* `unwrap_or` 采用及早求值策略，无论变体是否为`Error`，都会先行计算该值
* `unwrap_or_else` 采用惰性求值策略，等到变体为`Error`时，再行操作

**以上两种策略对函数型表达式（包含副作用的计算）会有不同的结果。**



当然，也可以采用常规的`match`方法对Result解包：

```rust
let version = parse_version(&[1, 2, 3, 4]);
match version {
    Ok(v) => println!("working with version: {v:?}"),
    Err(e) => println!("error parsing header: {e:?}"),
}
```

### 匹配多种错误

一般情况下，**引发错误的原因不只一种**，那么返回时Error也就不只一种类型。如果需要对不同的错误类型进行匹配，可以嵌套`match`语句。

```rust
let f = match File::open("text.txt") {
    Ok(file) => file,
    Err(error) => match error.kind() {
        ErrorKind::NotFound => File::create("text.txt").unwrap_or_else(|_| panic!("Create file error")),
        other_error => panic!("Unexpected error occurred: {:?}", other_error),
    },
};
```

以上代码分为两个部分：

* 文件打开正常，直接返回文件
* 文件打开异常，进行下一步判断
  * 如果找不到文件，则进行新建，并将新建的结果返回给原变量（**这里使用`unwrap_or_else`方法+闭包进行结果解包**）
  * 如果是其他错误，则终止，并报错

这里使用了一些用法：

* `error.kind()`：该语句的错误返回值为`io::Error`，具有`kind`方法，将返回一个枚举类型，其中包含产生错误的各种原因。

另外，还可以通过`map_error`语句结合闭包（也即其他语言的匿名函数）进行多类型错误处理，这将在后面讲到。

### 错误传播

有时候，我们并不需要在当前函数中处理错误，而是可以将其传播到外部，**由调用者处理**。这被称为**错误传播 (Error Propagation)**。使用该方案，则需要将当前函数的返回值也修改为`Result`类型。

如下例所示，函数`read_string`中的错误被传递到了`main`函数上，由main函数进行处理。

```rust
use std::{io::{self, Read}, fs::File};

fn read_string() -> Result<String, io::Error> {
	let f = File::open("text.txt");

	let mut f = match f {
		Ok(f) => f,
		Err(e) => return Err(e),
	};

	let mut s = String::new();
	
	match f.read_to_string(&mut s) {
		Ok(_) => Ok(s),
		Err(e) => Err(e),
	}
}

fn main () {
	let s = read_string().unwrap_or_else(|err| panic!("IO error: {:?}", err));
	println!("{}", s);
}
```

**但是，这样就比较麻烦了，必须为每一个返回`Result`的位置增加一个`match`**，因此，rust提供了简化的语法糖`?`，可以直接解包，并传递错误。

```rust
use std::{io::{self, Read}, fs::File};

fn read_string() -> Result<String, io::Error> {
	let mut f = File::open("test.txt")?;
	let mut s = String::new();
	f.read_to_string(&mut s)?;
	Ok(s)
}

fn main() {
	let s = read_string().unwrap_or_else(|err| panic!("IO error: {}", err));
	println!("{}", s);
}
```

这样看起来简洁多了，这个语法糖让rust错误处理变得更加优雅。

我们甚至还可以利用链式方法调用进行进一步简化：

```rust
fn read_string() -> Result<String, io::Error> {
	let mut s = String::new();
	File::open("test.txt")?.read_to_string(&mut s)?;
	Ok(s)
}
```

这样，在保证代码鲁棒性的同时，我们极大简化了代码复杂度。

需要注意的是，**`?`运算符只能用于返回Result\Option\任何实现了`std::ops::Try` trait 类型的函数**。
