# 9 - Generic Types, Traits, and Lifetimes

> &emsp;&emsp;泛型（Generics），在许多语言中都有出现，主要为了表征一类共有的特性，而不是指代一个特定的类型。我们使用一些抽象的性质表述一些类型，而不需要指定其具体类型。而trait 特征，则是我们约束泛型行为的方法。通过trait，我们可以限定泛型为一个具有某些特定行为的类型，而不是任意类型。最后，我们将讨论生命周期的概念，生命周期也是一类泛型，用于向编译器提供引用之间的相互关系，确保引用过程中的有效性。

## Generic Types 泛型数据类型

&emsp;&emsp;为了更好地进行**代码复用和模块化**设计，我们使用函数完成一系列特定的工作。但是，**对于相同的功能，不同的参数类型**，在传统语言中，我们需要**声明多个函数**来完成。例如，简单的比较大小函数，针对不同的参数类型，我们可能需要`comp_char`, `comp_int`, `comp_long_long`等等，但是本质上这些代码的功能是完全相同的，只是将两个参数进行**数值上的大小比较**。这样仍然会造成代码冗余，而我们可以使用**泛型**来解决这个问题。

 

### 结构体泛型

在结构体中加入**泛型参数**，就可以在结构体定义内部使用这些泛型。

```rust
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let integer = Point {x: 5, y: 5};
    let float = Point {x: 5.0, y: 4.0};
    println!("{:#?}\n{:#?}", integer, float);
}
```

* 泛型参数跟在结构体名后，用`<>`包含

* 泛型参数通常是**单个大写字母**

* 泛型参数**同时只能是一种类型**，例如，以下语句是错误的：
  ```rust
  let test = Point {x: 5, y: 5.0};
  ```

* 当然，泛型参数可以是多个，在具体使用时会对每个参数分别进行类型推导

### 基于泛型结构体实现方法

对结构体实现方法的部分在[《Rust权威指南》读书笔记5 - Struct_rust 读struct 内容_Zheng__Huang的博客-CSDN博客](https://blog.csdn.net/Zheng__Huang/article/details/127160284)一文中已经讲过。如果需要为泛型结构体实现方法，则需要在`impl`**后加上泛型参数**。

```rust
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
    fn y(&self) -> &T {
        &self.y
    }
}
```

* 方法可以省略self的类型，如果需要可变引用，则加上`mut`
* 示例中两个方法的返回值是泛型参数`T`，也就是结构体的泛型类型

### 枚举中的泛型

同样，枚举中变体的持有数据可以是泛型，例如之前经常使用的Option

```rust
enum Option<T> {
    Some(T),
    None,
}
```

另外，Result类型应用了两个泛型参数：

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

### 泛型效率

Rust编译器采用泛型代码**单态化**，在编译期将泛型代码转换为特定类型的代码。这样可能造成可执行文件的较大体积，**但不会产生任何性能损失**，这更符合Rust的设计理念。另外，单态化也与一般情况下没有泛型的语言实现相同，但是大大减少了人工编码的时间和源代码长度。



## trait 特征：定义共享行为

`trait`特征用于描述**某些类型具有的，并能为其他类型所共享的功能**，为类型的行为提供了一种抽象化的约束方法。即，我们可以通过`trait`将一个任意泛型约束为一类具有特定行为功能的类型。

*泛型与其他语言中`interface`接口的功能较相似，但还有其特殊部分*。

### Defining a trait

**trait 定义了一组实现特定功能的方法集合，这个集合可以被不同的类型实现，并提供给外部调用**。trait内容为一系列函数的签名（不需要函数体）

```rust
pub trait Summary {
	fn summarize(&self) -> String;
}
```

### implement a trait

通过`impl <trait> for <type>`语句块，为一个类型（结构体）实现一个特性：

```rust
pub struct NewsArticle {
	pub headline: String,
	pub location: String,
	pub author: String,
	pub content: String,
	pub date: String,
}

impl Summary for NewsArticle {
	fn summarize(&self) -> String {
		format!("{}, by {} on {} ({})", self.headline, self.author, self.date, self.location)
	}
}
```

* 在`impl`块内，IDE会为你自动补全定义在`trait`中的函数签名，其余实现过程和一般的方法类似

* **必须完整实现trait中的所有函数**，否则编译器会报错
* 只能**与定义结构体在相同的库中**，才能实现库的trait，这被称为**孤儿规则**，不过对trait没有这个限制，标记了`pub`的trait可以被引用到其他库中

实现trait后，我们可以用调用普通方法的方式调用它们。

```rust
fn main () {
	let news_a = NewsArticle {
		headline: String::from("Zheng Huang's paper got published"),
		location: String::from("China"),
		author: String::from("Unknown"),
		content: String::from("This is content of the news"),
		date: String::from("1-24-2023"),
	};
	println!("Summary of the news: {}", news_a.summarize());
}
```

### default implementation

可以为trait中的方法提供**默认行为**，只需要在签名后跟结构体即可，这些方法可以在具体实现中**被重载**。

```rust
pub trait Summary {
	fn summarize(&self) -> String {
		String::from("Read more...")
	}
}
```

* 在默认实现中，**可以调用trait中的其他方法，即使该方法没有默认实现**
* 但是在具体类型实现的过程中，不可以调用方法的默认实现（但是原本就可以不实现某个方法，使其采用默认实现，所以该场景可以避免）

### traits as parameters/return values

我们还可以将`trait`作为参数传入函数，**代表一类实现了`trait`的类型**。

```rust
fn notify(item: impl Summary) {
	println!("New news! Summary of the news: {}", item.summarize())
}
```

同理，返回值也可以使用该语法，将返回一个施加特定trait约束的泛型类型

```rust
fn returns_summarizable() -> impl Summary
```

* 但是，这个返回值不是特定的类型，而是一个trait对象

### trait bound

上例实际上是**trait约束**的语法糖，完整写法如下：

```rust
fn notify<T: Summary>(item: T) {
	println!("New news! Summary of the news: {}", item.summarize())
}
```

* 语法糖适合用于短小的函数参数类型定义，如单参数等

* trait bound适合更复杂的定义模式，如，*需要两个参数为同一个泛型类型时*：

  ```rust
  pub fn notify<T: Summary>(item1: T, item2: T)
  ```

* 通过`+`语法，指定多个约束
  ```rust
  pub fn notify<T: Summary + Display>(item: &T) {
  ```

* 通过`where`从句简化trait约束

  ```rust
  fn some_function<T, U>(t: &T, u: &U) -> i32
  where
      T: Display + Clone,
      U: Clone + Debug,
  {
  ```

在`impl`的泛型参数后，同样可以使用trait bound，只有泛型的具体类型具有约束的特性时，才会实现`impl`块中的方法。

```rust
struct Pair<T> {
    x: T,
    y: T,
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
```

### 覆盖实现 blanket implementation

通过trait bound，我们可以在一类已经实现某trait的类型上继续实现新的类型，这被称为**覆盖实现 blanket implementation**。例如，标准库为实现`Display` trait的类型附加实现了`ToString` trait。这样，**所有实现了`Display`特征的类型都具有`ToString`特征**

```rust
impl<T: Display> ToString for T {
    // --snip--
}
```



### 实例：数组最大值

```rust
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
	let mut largest = list[0];

	for &item in list.iter() {
		if item > largest {
			largest = item;
		}
	}
	largest
}

fn main() {
	let my_list_i32 = [1, 5, 3, 2, 0];
	let my_list_float = [1.0, 5.3, 2.5, 1.1 ,0f64];
	println!("i32 list max: {}", largest(&my_list_i32));
	println!("float list max: {}", largest(&my_list_float));
}
```

* 以上例子使用`PartialOrd + Copy` trait 约束泛型，实现了寻找最大值的方法

* 也可以不使用`copy` trait，这种方法将返回一个元素的引用

  ```rust
  fn largest_ref<T: PartialOrd>(list: &[T]) -> &T {
  	let mut largest = &list[0];
  		
  	for item in list.iter(){
  		if item > largest {
  			largest = item;
  		}
  	}
  	largest
  }
  ```

  * 这里在进行元素比较时，使用了**自动解引用**，比较的仍是引用指向的值



## Lifetimes 生命周期

**生命周期也是一种泛型**，用于确保一个变量值在一段时间内的有效性。

在大多数情况下，生命周期是隐式并且可推导的。当几个变量的生命周期有一些特殊的联系时，我们可以手动指定某些变量的生命周期。

### 用生命周期避免悬垂引用

生命周期的主要作用是避免**悬垂引用**(Dangling Reference)，这在之前的例子中已经体现：

```rust
fn main() {
    let r;	// 外部作用域变量声明（并不是空值，只是表示变量存在于外部作用域中）
    {
        let x = 5;	// 内部作用域变量/值生命周期开始
        r = &x;		// 外部作用域变量生命周期开始，内部作用域变量生命周期结束
    }
    println!("r: {}", r);	// 引用指向生命周期结束的变量，悬垂引用发生！
}
```

### 借用检查器

Rust采用借用检查器borrow checker检查借用的有效性。**如果引用的生命周期长于被引用值的生命周期，则会报错**。即，被引用值必须比引用先起效，比引用后失效（可以同时）。

### 函数中的泛型生命周期

举个例子，我们需要比较两个字符串的长度，并返回较长的那个字符串的切片引用。

```rust
fn longer(x: &str, y: &str) -> &str {
	if x.len() > y.len() {
		x
	} else {
		y
	}
}

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longer(string1.as_str(), string2);
    println!("The longest string is {}", result);
}
```

* 以上代码看起来实现了功能，**但实际上是无法运行的**
* 返回值是一个**引用，包含一个借用值**，但是检查器无法判断其来自哪个参数（如果两个参数都不是，那就更不用说了，一定产生悬垂引用，但是报错似乎是相同的）
* 通过**向函数签名增加显式生命周期**，可以解决这个问题

### 生命周期标注

生命周期标注不会改变任何引用对象的生命周期长度，而是用于**描述多个引用生命周期之间的关系**。

语法：生命周期使用`'`开头，并全小写字母表示，通常的生命周期标注为`'a`

```rust
&i32		// normal reference
&'a i32		// reference with a explicit lifetime
```

**对单个变量的生命周期描述没有意义，生命周期标注的意义在于描述多个变量之间的生命周期关系**。两个相同生命周期标注的变量**必须拥有相同的生命周期**。

如下修改能够使上例代码顺利通过编译：

```rust
fn longer<'a>(x: &'a str, y: &'a str) -> &'a str {
```

* 函数签名要求：**两个变量`xy`，以及返回值的生命周期必须相同**
* **注意：生命周期标注不会修改变量的实际生命周期，它只是给借用检查器提供了借用相关的生命周期信息，帮助其做出正确判断**。

我们继续进行实验，以下代码不能通过编译：

```rust
fn longer<'a>(x: &'a str, y: & str) -> &'a str {
	if x.len() > y.len() {
		x
	} else {
		y
	}
}
```

提示如下：

```rust
error[E0621]: explicit lifetime required in the type of `y`
  |
1 | fn longer<'a>(x: &'a str, y: & str) -> &'a str {
  |                              ----- help: add explicit lifetime `'a` to the type of `y`: `&'a str`
...
5 |         y
  |         ^ lifetime `'a` required
```

提示缺少生命周期标注，*因为y同样可能被返回，则必须保证它的生命周期不短于返回值的生命周期*，如果不反悔`y`，则问题消失（虽然结果并不是我们想要的）

将两个生命周期不同的参数传入函数中：

```rust
    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longer(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }
```

代码可以通过编译，这是因为**生命周期标注不会影响具体的生命周期，只要保证两个参数和返回值在同一个作用域内全都有效即可**，返回值和一个参数同时在内部作用域中失效，故没有违背生命周期约束。

但如果将返回值的生命周期延长到外部作用域：

```rust
    let string1 = String::from("long string is long");
	let result;

    {
        let string2 = String::from("xyz");
        result = longer(string1.as_str(), string2.as_str());
	}
    println!("The longest string is {}", result);
```

则会报错：

```rust
error[E0597]: `string2` does not live long enough
  --> src\main.rs:20:43
   |
20 |         result = longer(string1.as_str(), string2.as_str());
   |                                           ^^^^^^^^^^^^^^^^ borrowed value does not live long enough
21 |     }
   |     - `string2` dropped here while still borrowed
22 |     println!("The longest string is {}", result);
   |                                          ------ borrow later used here
```

**可以看到，返回值与参数始终保持着约束关系，当一个参数失效后，就不能使用返回值**

综上所述：**如果指定相同的生命周期**，则返回值的生命周期必须短于任意一个参数的生命周期。

### 结构体生命周期标注

**如果需要在结构体中使用引用，则需要为其标注生命周期**

```rust
struct ImportantExcerpt<'a> {
    part: &'a str,
}
```

这要求：整个结构体实例的生命周期**不能长于引用成员（加标注）**的生命周期

### 生命周期省略

在代码中，经常有一些固定的**生命周期标注模式**，早期的版本并不支持生命周期省略，但后来随着重复模式逐渐显著，它们被加入编译器中，使借用检查器能够自动进行推导。这些已经成惯例的生命周期标注模式不需要被显式标注，称为**生命周期省略**

在介绍规则之前，先介绍一些概念：

* **输入生命周期**：函数/方法参数中的生命周期称为输入生命周期
* **输出生命周期**：返回值的生命周期

目前的生命周期省略规则如下（以下是隐式规则，可以被显式改写）：

1. 每一个**引用参数**都有自己的生命周期函数，如`fn foo<'a, 'b>(x: &'a i32, y: &'b i32, z: i32)`

2. 只存在一个输入生命周期参数时，将**赋值给输出生命周期**。以下函数将被正常编译，而不需要显式指定生命周期
   ```rust
   fn foo(input: &str) -> &str {
   	return &input[1..]
   }
   ```

3. 在方法中（`&self` or `&mut self`），若拥有多个输入生命周期参数，**`self`的生命周期参数自动传播给输出生命参数**

第三条规则只能出现在方法的生命周期标注中

### 方法定义的生命周期标注

在为具有生命周期参数的结构体实现方法时，需要指定生命周期参数，语法如下：

```rust
struct ImportantExcerpt<'a> {
	part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
	fn foo(&self, s: &str) -> &str {
		self.part
	}
}
```

* 函数签名`foo`中使用了生命周期省略规则的第三条

### 静态生命周期

有一类特殊的生命周期`'static'`，它们在**程序运行过程中全局有效**。但是，这种长生命周期通常是冗余和不安全的，不建议使用静态生命周期解决悬垂引用的问题。

### using generic, trait and lifetime together

同时使用三种标注/约束的语法如下：

```rust
fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
```

