# Rust - Ownership

> &emsp;&emsp;**所有权 (Ownership)**是Rust语言中最为独特的功能之一。通过所有权，Rust实现了在没有GC的前提下保障内存安全。

## Introduction

&emsp;&emsp;**所有权 (Ownership)**的概念在Rust中较为特殊。通常，没有垃圾回收机制（GC）的语言，如C/C++等，会让程序员手动进行内存管理；而具有GC的语言，如JAVA，提供了自动定期检查并回收不再需要的垃圾内存的机制。

&emsp;&emsp;**Rust语言采用第三种方式，基于所有权系统的内存管理。**这种内存管理是在**编译时**确定的，当Rust编译器检查到一部分内存不再被使用，会将收回内存的工作**静态编译进程序中**。这种方式既保证了内存安全，又减少了GC的开销，大大增加了程序运行效率。



### Ownership Rules 所有权规则

* 每个**值(value)**对应一个**变量(variable)**作为**所有者(owner)**
* 同一时间，值**有且仅有一个所有者**
* 当所有者**离开**自己作用域时，持有的信息会被**释放**



### Variable Scope 变量作用域

作用域，在Rust中通常为被括号`{}`**包含的部分**。

* 变量**从声明开始到作用域结束的区域内**都有效



### String 类型

在Rust中，主要有两种字符串存储模式：

* `str`类型：又被称为`string slice`，是Rust最基础的字符串类型，一般用于**字面量**
    * 由一个指针和编码字符串字节组成
    * 字节硬编码于二进制代码中，不可修改

* String类型存储在**堆**上，方便我们学习Rust内存管理
    * String是最常用的字符串类型，拥有对字符串内容的**所有权**
    * 使用`String::from`函数，将**字符串字面量**`str`转换为可变的String



### Memory and Allocation 内存与分配

对`String`类型，由于需要支持其可变特性，必须将其安排在堆上。

* 声明时，向系统动态申请内存，用于存放字符串内容
* 当使用完成时，将自动调用`drop`函数进行是释放
    * 编译器通过**变量作用域**规则判断一个变量的值是否不再需要，进而判断销毁时机

Attention：**在对象生命周期结束时释放资源**的模式，在C++中称为**资源获取即初始化(Resource Acquisition Is Initialization, RAII)**

#### move 移动

在一些简单数据类型上，Rust会复制两个对象，并直接在栈上储存

```rust
let x = 5;
let y = x;		// value copied
```

但是对于一些复杂对象，如`String`结构，则会采用`move`，**只转移所有权**

一个`String`包含三个部分：

* pointer 指向String内容（字符串首地址）
* len 存储字符串长度
* capacity 字符串容量

&emsp;&emsp;如果需要对一个String的值生成两个引用，则会**拷贝两份ptr、len和capacity**。但这样带来了比较大的隐患：当一个变量离开作用域，则会drop，如果非同时离开作用域，则会导致**UAF(Use After Free)**或者**DF(Double Free)**

&emsp;&emsp;`move`操作会将右值变量**设为无效**。转移后的变量不能被使用，离开作用域也不会触发`drop`。如果尝试使用变量，则会报编译错误`used of moved value`。这与浅拷贝(shallow copy)不同。

* Rust**永远不会自动创建数据的深度拷贝，所以自动赋值操作永远是高效的**

#### clone 克隆

某些类型提供了`clone`方法，提供数据的深度拷贝。

```rust
let s1 = String::from("hello");
let s2 = s1.clone();

println!("s1 = {}, s2 = {}", s1, s2);
```

#### 栈上数据的复制

对于栈上的简单数据，

* 定长
* 可完全放入栈中

我们不需要使用`move`以保证安全，换句话说，**深拷贝和浅拷贝造成的结果是一样的**。可以直接进行复制。

Rust提供`Copy trait`，可以用于整数这类**完全存储在栈上的数据类型**。定义这种trait后，变量可以**在复制给其他变量后保持可用性。**

* 如果实现了`Drop`(**即具有堆上数据，需要另外进行清理**)，则不能实现`Copy`
* 一般来说，**简单标量**及其**组合**类型都是Copy的，需要**分配内存或资源的类型都不会是Copy的**


### 所有权与函数

* **将值传递给函数**在语义上类似于**对变量进行赋值**。将变量传递给函数将**触发移动(move)或复制(copy)**。

* 函数的**返回值**同样等效于**赋值**，会将所有权转移出函数
    * 返回值的作用域从调用函数的行开始，一直到调用函数的作用域结束



## Refernces and Borrowing 引用与借用

> 使用引用`Reference`可以使用参数的部分功能，但同时**不会拥有所有权**。

### 引用

使用`&`表示引用语义，`*`表示解引用 (dereferencing)。创建引用的行为被称为**借用 (borrow)**

```rust
fn main() {
    let s1 = String::from("Hello");
    
    println!("length of string {} is {}", s1, calc_string_len(&s1));
}

fn calc_string_len(s: &String) -> usize {
    *s.len()
}
```

* ps: 此处的`*`不是必须的，涉及方法的**自动引用和解引用**，将在后面讲到

* 引用**不拥有数据的所有权，不会在离开作用域时销毁数据**



### 可变引用

使用`&mut`创建数据的可变引用，默认不可变

```rust
let s_ref = &mut s;
```

* **被引用对象必须是可变的**

* 可以使用可变对象的所有功能

* 同时只能有一个**可变借用**，这里借用包含原变量和引用变量，如下所示
    ```rust
    fn main() {
    	let mut s = String::from("Hello");
    	let s_ref = &mut s;
    	s.push_str("string");
    	s_ref.push_str("string");
    }
    ```

    * 这个程序会报`cannot borrow 's' as mutable more than once at a time`。并且，在**借用被使用的时候**才会触发(第五行)。这种设计可以在编译器层面防止**数据竞争**。

    * 上面的例子中，如果将s_ref在一个函数里使用，则不会产生问题，因为在使用原变量时，对其数据的引用已经销毁了
        ```rust
        fn main() {
        	let mut s = String::from("Hello");
        	add_string(&mut s);
        	s.push_str("string");
        	println!("String is {}", s);
        }
        
        fn add_string(s: &mut String) {
        	s.push_str(", world. ");
        }
        ```

    * **不能同时使用可变引用和不可变引用**，不可变引用保证了数据在其引用期间的不可变性，类似于读者锁。



### 悬垂引用

**Dangling Reference 悬垂引用**，在应用裸指针概念的语言中比较常见。当一个对象被销毁，但是对该对象的引用（指针）没有销毁，则会出现**悬垂引用**。这个指针指向的数据是无效的。如果使用这个引用，就会出现非常常见的错误——**UAF(Use After Free)**。

**Rust编译器会确保不会进入悬垂状态**。如果当前存在一个对某个数据的引用，编译器会确保数据不会在引用存在时**离开作用域（而被销毁）**或修改（仅针对不可变引用）。

以下的代码将被Rust识别为悬垂引用，并报编译错误：

```rust
fn dangling_test() -> &String {
	let s = String::from("Hello world");
	&s
}
```

编译器提示`missing lifetime specifier`， *"This function's return type contains a borrowed value, but there is no value for it to be borrowed from"*，表示该引用已经找不到被借用的对象，无法继续使用。



## Slice 切片

考虑一个需要引用一个字符串的**部分**（或全部）而**不需要所有权**的情景。如果单独使用数字索引作为记录，*是根据数据的某个特定状态得到的*，没有跟数据产生任何程度上的联系。这将导致更新同步上的诸多问题。Rust针对这个问题，采用了**切片(Slice)**的解决方案。注意，**切片类型不同于原类型**，不支持一些原类型的操作，如插入等。

### 字符串切片

切片与创建引用类似，后面跟范围区间：
```rust
let s = String::from("Hello world");
let s1 = &s[0..5];
let s2 = &s[..5];	// same as s1
let whole_s = &s[..];
```

* 可以使用简写的前后区间，表示开头和末尾
* **Attention: 如果对UTF-8字符串进行切片，切片边界必须在字符之间（不能在单个字符中间）**，否则会报运行时错误。
* 字符串切片的类型为`&str`，不能直接修改值

本质上，字符串字面量就是一个切片，它的类型为`&str`。在使用中，可以将完整的`String`用切片作为参数传入，不会影响功能，能让API更加通用。

### 通用数组切片

Rust提供了更加通用的数组切片：

```rust
let a = [1, 2, 3];
let b = &a[..2];

let mut a = [1, 2, 3];	// mutable
let b = &mut a[..];		// mutable slice
b[0] = 0;				// edit mutable data
```

* 不可变切片类型为`&[i32]`，可变切片类型为`&mut [i32]`

这里同样涉及可变与不可变引用：

* 如果存在一个不可变切片，则不能改变原数据
* 如果已经存在一个可变切片，则只有可变切片（已经借用的）允许数据修改，而不允许使用原变量修改
