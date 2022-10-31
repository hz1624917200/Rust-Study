# Enums and Pattern Matching

> &emsp;&emsp;枚举类型，允许我们列举所有可能的值以表示这个类型。**枚举可以连同数据以编码信息**。Rust中的枚举类似于`F#` `Haskell`等函数式编程语言中的代数数据类型。

## Defining an Enum 定义枚举

例如，处理IP地址编码问题，需要针对两种标准进行讨论。一个地址只有两种版本选择，而这两种版本并不兼容。但是不同版本同属于IP地址协议，应当视为同种类型。此时，适合使用枚举类型定义IP地址。

```rust
enum IpAddr {
	V4,
	V6,
}
```

* 其中，`V4` `V6`为enum的两个**变体(variant)**

### 枚举值

使用`::`来创建对应枚举值的实例

```rust
let ipv4 = IpAddr::V4;
```

枚举值也可以用在结构体的字段中，声明类型为枚举名，其余与普通变量相同

### 关联数据

**枚举允许将与变体关联的数据嵌入枚举变体内**，增强枚举变体与其关联数据的耦合。这些关联数据也可以是结构体（标准库就采用这种实现方式）。

```rust
pub enum IpAddr {
    /// An IPv4 address.
    #[stable(feature = "ip_addr", since = "1.7.0")]
    V4(#[stable(feature = "ip_addr", since = "1.7.0")] Ipv4Addr),
    /// An IPv6 address.
    #[stable(feature = "ip_addr", since = "1.7.0")]
    V6(#[stable(feature = "ip_addr", since = "1.7.0")] Ipv6Addr),
}
```

* 标准库实现`IpAddr`的具体内容，其中`Ipv4Addr`和`Ipv6Addr`是两个另外定义的结构体

另一个实例中给出了更加灵活的使用方法

```rust
enum Message {
	Quit,
	Move {x: i32, y: i32},
	Write(String),
	ChangeColor(i32, i32, i32),
}
```

其中，

* `Quit`没有关联数据，表示直接退出
* `Move`包含一个**匿名结构体**，表示移动的Offset
* `Wirte`包含一个`String`，用于写入字符串
* `ChangeColor`包含了3个`i32`值，表示改变的目标颜色

通过以上方式，我们定义了一个统一的消息类型，进而可以设计一个统一的消息处理函数，而不需要多个接口。

### Option 枚举类型

> &emsp;&emsp;Option枚举定义于标准库中，描述了值可能不存在的情形，广泛适用于**空值处理**。空值(Null)本身是一个值，含义是没有值。在设计有空值的语言中，一个变量往往处于两种状态，空值和非空值。

Rust使用Option枚举来提供类似空值概念，可以用它来标识一个值无效或缺失。

```rust
pub enum Option<T> {
    /// No value.
    #[lang = "None"]
    #[stable(feature = "rust1", since = "1.0.0")]
    None,
    /// Some value of type `T`.
    #[lang = "Some"]
    #[stable(feature = "rust1", since = "1.0.0")]
    Some(#[stable(feature = "rust1", since = "1.0.0")] T),
}
```

* 该枚举被包含在与导入模块中，不需要显式引入
* `<T>`表示一个泛型参数，将在后面提到。简单说，它代表任何一种类型

* 当初始化为`some`时，编译器可自行完成类型推导，当初始化为`none`时，**必须手动指定类型T**
* 这个枚举类型使编程人员**必须考虑值为空**的情况（没有设计该枚举与对应类型直接相加等操作的情况）
* 只要一个值不是`Option<T>`的，那么**它就一定是非空的**

* 关于Option枚举的具体使用示例，参见[Option in std::option - Rust (rustwiki.org)](https://rustwiki.org/zh-CN/std/option/enum.Option.html)



## match 控制流运算符

之前已经用过，`match`运算符可将一个值与一系列模式进行比较，并根据匹配模式执行代码。**模式可以由字面量，变量名，通配符和其他组成**。当然，也可以用枚举类型进行模式匹配（这是常见用法）。

```rust
let mode = 'q';
match mode {
    'w' => todo!(),
    'r' => todo!(),
    'q' => return,
    _ => {println!("Input error!");},
};
```

* 枚举的所有分支（arm）必须返回**相同类型**的数据，并与使用该表达式的变量相匹配
* 分支采用自上而下的匹配方式，**并需要包含所有可能路径**（穷尽(exhausitive)匹配），没有任何动作的路径可以用空代码块`{}`代替。

### 使用match匹配 Option

```rust
fn main() {
	assert_eq!(Some(6), plus_one(Some(5)));
	assert_eq!(None, plus_one(None));
	println!("Bingo!")
}

fn plus_one(x: Option<i32>) -> Option<i32> {
	match x {
		None => None,
		Some(i) => Some(i + 1),
	}
}
```

* **Attention**: 使用 **match guard**以强化条件匹配能力

```rust
let num = Some(4);

match num {
    Some(x) if x % 2 == 0 => println!("The number {} is even", x),
    Some(x) => println!("The number {} is odd", x),
    None => (),
}
```

* 使用`_`**通配符**以匹配所有剩余情况，类似`Default`，在分支内部参数中同样适用
* 使用`|`表示多分支合并的**或**关系

### if let 简洁控制流

使用`if let`语句可以较简单地实现**只关心一种匹配而忽略其他匹配**的情况。以下两种表述方式在功能上相同：

```rust
let option_num = Some(3);

// method 1
match option_num {
    Some(value) => println!("The value is {value}"),
    None => (),
}

// method 2
if let Some(value) = option_num {
    println!("The value is {value}");
}
```

`if let`语句也可使用else，**可以看作是`match`结构的语法糖**。