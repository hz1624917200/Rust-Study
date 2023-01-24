# Rust - General Programming Concepts

> &emsp;&emsp;本文对应《Rust权威指南》的第3章，主要介绍Rust中的通用编程概念，如变量、数据类型、函数、程序控制流等。

## Variables & Mutability 变量与可变性

### Mutability 可变性

注意，此处的**可变性**(**Mutability**)与**Python**变量中的**可变变量**所指相同，而不是与常量相对的变化性。

**Rust中的变量默认不可变， 使用**`mut`**关键字**可以创建一个可变变量。

```rust
let x = 123;		// immutable
let mut x = 123; 	// mutable
```

PS: 关于这样设计的原因，笔者看到了一个编码习惯方面的解释：**在实现同样功能的情况下，程序员倾向于使用更短的语法。**在Rust中，大多数情况下，不可变变量可以胜任工作。如果可变变量的声明比不可变变量短，那么程序员倾向直接使用可变变量，**而这产生了不安全性。**将不可变设计得比可变短，**体现了语言鼓励不可变变量。**

* 如果试图改变不可变变量的值，则会编译时报错 "*cannot assign twice to immutable variable*"
* Rust编译器**保证声明为不可变的值一定不会改变**
* 当数据结构较为清凉的时候，采用偏向**函数式**的风格，通过创建新变量来赋值，可能使代码更加易于理解

### Const 常量

常量与不可变变量的概念也不相同。我们使用`const`关键字修饰一个常量

```rust
const MAX_POINTS: u32 = 100;
```

* 必须**显式标注常量类型**
* 常量可以声明在**任何作用域**中，包括全局作用域
* 常量只能绑定到**一个常量表达式上**
* *常量类似C/C++中的常量宏*

### Shadow 隐藏

Rust提供**隐藏机制**，**允许用户使用同一个变量名绑定不同的变量**。

使用`let`声明的新变量可以**覆盖旧的同名变量。**这被称为旧变量被新变量**隐藏(shadow)**了。我们可以不停使用let同一变量名来隐藏变量。

* 隐藏保持了变量的可变性（不可变变量被不可变变量隐藏后还是不可变）
* 隐藏允许**绑定不同类型的变量**，而不能直接改变可变变量的类型

* 需要注意的是，**隐藏仅限于变量作用域中**，两个同名变量的作用域交叠，当一个变量离开作用域，另一个变量仍可用。隐藏不等同于替换



## Data Types 数据类型

Rust 中，数据类型可以分为两类：**标量类型(Scalar), 复合类型(Compound)**

注意：Rust是**静态类型语言**，所有变量的具体类型都需要在编译过程中确定。*（这和变量类型是否被显式声明无关，使用隐式声明，编译器也能在一定条件下推导变量类型）*。

### Scalar 标量类型

**标量类型**是单个值类型的同城，Rust 内建(builtin)了4中基础标量类型：

* 整数 (integer)
* 浮点数 (float)
* 布尔型 (bool)
* 字符型 (char)

#### Integer

整型下，根据长度和有无符号位，分为以下几种细分类型

| Size        | Signed | Unsigned |
| ----------- | ------ | -------- |
| 8b          | i8     | u8       |
| 16b         | i16    | u16      |
| 32b         | i32    | u32      |
| 64b         | i64    | u64      |
| 128b        | i128   | u128     |
| arch-depend | isize  | usize    |

* 每一个类型都明确表明其大小和有无符号
* 一般使用默认推导类型的`i32`，运行效率最高

**整型溢出**

* 在debug模式中，编译器默认会提供溢出检查，溢出时发生`panic`
* 在release模式中，不会进行检查和`panic`，替换为补码环绕（类似C）
* 可以使用标准库类型`Wrapping`手动启用环绕

#### Float

共有两种浮点数类型，`f32`单精度和`f64`双精度，两者效率接近。Rust默认将浮点数推导为`f64`。

#### Numerical Operations 数值运算

Rust支持基本数学运算符。但是，**Rust不支持不同类型的运算(整型内部也不可)，需要将字面量指定成对应类型（在后面添加即可）**

```Rust
let a = 5 / 2;		// correct, a == 2
let a = 5.0 / 2;	// error
let a = 5.0 / 2.0;	// correct, a == 2.5, f64
let a = 5f64 / 2f32;// error, type mismatch
let a = 5f32 / 2.0;	// correct, a == 2.5, f32
```

#### bool

布尔型只有两个值，`true` & `false`，占用1Byte

#### char

用于表示最基础的单个字符，**使用单引号指定**`''`（与C类似）

**Attention**: `char`类型占4字节，使用unicode编码，提供了外文、表情支持



### Compound 复合类型

**Compound Type** 将多个不同类型的值组合为一个类型。Rust提供两种Builtin基础复合类型

* tuple 元组
* array 数组

#### tuple

元组可以将不同类型的值组合进一个符合类型。**元组拥有一个固定长度**，长度无法在声明结束后修改。

元组使用圆括号`()`包含，逗号`,`分隔

```rust
let tup = (500, 6.4, 'a', true);
```

* destructuring 解包：类似Python语法，使用let语句，用多个变量承接元组变量的值

```rust
let (x, y, z, a) = tup;
```

* index 索引，使用点号 `.`后跟索引，使用对应位置上的值

#### array

Rust数组拥有**固定长度**，声明后无法改变。使用方括号`[]`来声明。数组的每个元素**必须是相同类型**。另外，可以用类型标注，指定元素类型

```rust
let a = [1, 2, 3, 4];
let a: [u64; 4] = [1, 2, 3, 4];
```

* 如果需要长度可变，可使用vector

**数组批量声明方法**

使用批量声明，可以快速创建含有相同元素的数组

```rust
let a: [u64; 3] = [1; 3];		// assert_eq!(a, [1, 1, 1]);
let a = [1u64; 3];				// same as line 1
```

**Accessing array elements**

使用方括号`[]`与索引访问元素。

```rust
let a = [1, 2, 3, 4];
assert_eq!(a[0], 1);
```

* **数组越界**：如果在运行过程中发生数组越界，程序会触发panic: `index out of bounds`



## Functions 函数

* 使用`fn`关键字声明函数。

* **Rust采用snake case(蛇形命名法)规范函数命名**，所有字母小写，单词采用下划线连接
* Rust不关心函数定义位置，只关心该函数是否在范围内可见

```rust
fn test_function() {
    println!("This is a test function");
}
```

### 函数参数

在英文中区分了函数的形参(**Parameter**)和实参 (**Argument**)，但在中文中常常混淆为“参数”。应当区分。函数参数在括号中填充

```rust
fn test1(x: i32) {
    println!("the value of x is {}", x);
}
```

* **在函数签名中，必须显式声明参数类型**。这经过了Rust设计者们的慎重考虑。这样可以明确地进行类型指定，而不需要其他代码的类型推导。

### 语句和表达式

语句不会返回值，表达式会。**Rust中的大部分代码都是表达式**

* 普通赋值语句不会返回值，这不同于C

* 表达式本身作为语句的一部分

* 以下都是表达式

    * 调用函数

    * 调用宏

    * **花括号**`{}` 除了能新建作用域，也能作为表达式使用。最后一行语句如果没有分号`;`，则被认为是`{}`表达式的返回值

        ```rust
        let y = {
            let x = 3;
            x + 1		// no semi-colon
        }
        ```

        * 如果后面加上分号，则返回空 `()`

### 函数返回值

Return Value 是函数向调用它的代码返回的值。**需要在箭头->后声明其类型**

同样，使用表达式（最后一行）**隐式传递返回值**，或使用`return`关键字提前返回。

```rust
fn five -> i32 {
    5		// return 5 implicitly
}
fn three -> i32 {
    return 3;	// return explicitly
}
```



## Control Flow 控制流

除了之前提到的`match`匹配，Rust同样提供了正常的控制流，如分支结构`if`和循环结构等

### if statement

`if`表达式的判断条件(condition)不需要使用括号包含。但是所有代码必须用大括号包装为**代码块**。

Condition必须产生一个`bool`类型的值，**这与其他语言中非零即true的设计不相同**。

```
let number = 3;
if number {			// error
	todo!();
}
if number != 0 {	// correct
	todo!();
}
```

代码格式规范：

```rust
if y == 1 {
    todo!()
} else if y == 2 {
    todo!()
} else {
    todo!()
}
```

* 由于`todo!`宏返回空，所以加不加分号不影响
* 当`if`表达式独立出现，**代码块必须只能返回空**，否则编译错误
* 过多的`if-else`导致代码杂乱无章，可以用之前介绍的`match`分支结构（在后面介绍）

* if 可作为表达式使用（返回特定类型的值）
    ```rust
    let x = if y == 1 {
        1
    } else if y == 2 {
        2
    } else {
        'a'		// error, type mismatch
    };
    ```

    * **注意：各代码块类型必须相同（或与类型标注一致）**
    * **注意：各分支必须全部覆盖（最后一定有else），否则该arm返回空，导致赋值失败**

* 我们可以使用`if let`语句精简代码，将赋值和匹配连在一起

### Repetition Structure 循环结构

#### loop

使用`loop`语句实现循环（死循环），通过`break`关键字退出

```rust
loop {
    todo!();
    if condition {
        break;
    }
}
```

* 通过在break后加参数以返回值（用于表达式）
    ```rust
    let mut counter = 0;
    let mut sum = 0;
    let result = loop {
        counter += 1;
        sum += counter;
        if counter == 10 {
            break sum * 2;
        }
    }
    ```

* **跳出多重循环**：使用**标签**以跳出多重循环，默认跳出最内层循环

    ```rust
    fn main() {
        let mut count = 0;
        'counting_up: loop {
            println!("count = {count}");
            let mut remaining = 10;
    
            loop {
                println!("remaining = {remaining}");
                if remaining == 9 {
                    break;
                }
                if count == 2 {
                    break 'counting_up;
                }
                remaining -= 1;
            }
    
            count += 1;
        }
        println!("End count = {count}");
    }
    ```

#### while condition loops

```rust
while condition {
    todo!()
}
```

### Looping through a collection with for

使用`for`语句来遍历一个集合

```rust
let a = [10, 20, 30];
for ele in a {		// element, ele: i32
    println!("Element is: {}", ele);
}
for ele in a.iter() {	// iterator, ele: &i32
    println!("Element is: {}", ele);
}
```

* 第一种方式（直接使用集合）传递集合中元素的**值**
* 第二种方式使用迭代器，直接传递元素的**引用**

使用`Range`生成循环

```rust
for number in 1..4 {
    println!("Number is: {}", number);
}
for number if (1..4).rev() {	// revert the sequence
    println!("Number is: {}", number);
}
```
