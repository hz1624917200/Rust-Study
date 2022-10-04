# 5 - Struct 结构体

> &emsp;&emsp; 结构体，是一种自定义数据类型，允许程序员将不同类型的数据结合起来，形成相关联的整体。Rust的结构体还提供**方法**和**关联函数**，可以指定那些与结构体数据相关的行为。

## Defining and Instanting 定义与实例化

使用`struct`关键字以定义结构体。

```rust
struct User {
	id: u64,
	name: String,
	email: String,
	active: bool,
}
```

使用`let`语句声明结构体的实例，使用`mut`指定可变性（必须全部可变或不可变，没有部分可变）

```rust
let user1 = User {
    id: 1u64,
    name: String::from("Zheng"),
    email: String::from("888888@outlook.com"),
    active: true,
};
```

* 使用`.`指定结构体成员，如`user1.name`

如果**参数或变量**与字段名同名，可以使用简化写法初始化变量

```rust
fn new_user(id: u64, name: String, email: String) -> User {
	User { 
		id, 
		name, 
		email, 
		active: true 
	}
}
```

使用**结构体更新**语法，可以基于已有实例的字段创建新实例

```rust
	let user3 = User {
		id: 3u64,
		..user2
	};
```

* 使用`..[instant_name]`语法，剩余字段将使用`user2`的字段
* **Attention: 结构体更新等同于变量赋值，将发生value move**，源变量可能导致不可用

    ```rust
    let user3 = User {
        id: 3u64,
        ..user2
    };
    println!("{:#?}", user3);
    println!("user2: {:p}; user3: {:p}", &user2, &user3);
    ```

    以上代码将会产生错误：`borrow of a partially moved value`。这是由于`user2`的部分变量（字符串String）被赋值给了`user3`，导致所有权发生转移，结构体整体（不包括未发生value move的字段）将不再可用。

使用**元组结构体**可定义匿名字段

```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
```

* 注意，不同名的相同结构元组结构体**将视为不同的类型，无法相互转换**

使用**空结构体**，只实现trait，不需要存储任何数据时可以使用。

**结构体数据的所有权**：当一个结构体拥有其全部数据的所有权，则只要结构体有效，其数据就有效。如果存在对其他数据的引用，则需要使用*生命周期*支持。

### Derived Traits 派生 trait

派生`trait`，指通过`derive`注解 `derive attribute`来派生的`trait`，可以为自定义类型添加许多功能。关于派生trait列表，详见附录C

这里使用`Debug trait`展示自定义结构体的成员信息。

```rust
#[derive(Debug)]
struct User {
	id: u64,
	name: String,
	email: String,
	active: bool,
}

fn main() {
    let user1 = User {
		id: 1u64,
		name: String::from("Zheng"),
		email: String::from("888888@outlook.com"),
		active: true,
	};
    println!("{:#?}", user1);
}
```

* 以上代码会显示`user1`的全部字段信息
* 也可以使用`{:?}`，单行输出
* 实际上，它调用了`std::fmt::Display`，这被包含在派生trait中。



## Method 方法

方法是定义在结构体（或枚举类型`enum`，`trait`对象）的上下文中的类似函数的过程，**第一个参数是self，用于指代调用方法的结构体实例**。

### Define

需要在`impl`块中定义方法，

```rust
#[derive(Debug)]
struct Rectangle {
	width: u32,
	height: u32,
}

impl Rectangle {
	fn area(&self) -> u32 {
		self.width * self.height
	}
}

fn main() {
	let rect1 = Rectangle {
		width: 30,
		height: 20,
	};
	println!("Size of {:?} is {}", rect1, rect1.area());
}
```

* `self`参数自动拥有`impl`的结构体类型，不需要另行指定
* `self`参数同样拥有可变引用和直接获取所有权
* 除了方法**必须在`impl`块中，以及第一个参数名必须为self以外**，其余与函数相同
* 可以使用多个`impl`块来定义方法、关联函数，在泛型和trait中有应用价值

### 自动引用与解引用

在方法调用中，Rust支持**自动引用与解引用，使调用者匹配方法的签名**，如添加`& &mut *`等。

### Associated function 关联函数

 之前已经提到过，还有不需要接收实例参数的**关联函数**。因为其不做用于某个结构体实例，所以称其为函数。使用`::`来使用关联函数。
