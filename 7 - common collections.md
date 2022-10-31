# Common Collections

> &emsp;&emsp;集合数据类型（collections），是编程语言中可以表达多个值的类型。不同于标量类型只能表达单一的值。**这些集合将持有的数据存储在堆上，不需要在编译时确定大小。**这带来了极大的编程便利性。Rust中，广泛使用的集合类型主要有三种：动态数组 `Vector`, 字符串 `String`, 哈希映射 `hash map`。



## Vector 动态数组

Rust中的动态数组为`Vec<T>`，**允许在单个数据结构中，存储多个相同类型的数据。**

其中，`T`表示数组中元素的类型，Rust应用了泛型系统，这部分将在后面详细学习。

### Vector Definition

使用`Vec::new()`关联函数创建一个空数组。**注意：如果需要修改vector，仍必须将其声明为`mut`**

```rust
// TODO
```

**根据Vector中初始元素的类型，编译器通常能够自动完成类型推导。**当无法自动推导时（*初始化一个空的列表，也没有进行初始push*），需要手动指定元素类型。

另外，也可以使用`vec![]`宏，快速初始化一个vector：

```rust
// TODO
```

### Vector Update

使用`push`和`pop`方法来向vector中增加、删除元素（类似栈，如果需要队列形式，可以使用`VecDeque`双端队列）。

```rust
// TODO
```

### Vector Destruction

与其他变量相同，当变量离开作用域，会自动触发drop析构。**其所有元素都被销毁**。所以，不能向外部传输一个对局部vector元素的引用

### Vector element access

有两种方法可以访问vector中的元素.

#### vector index

与C/C++类似，使用方括号索引，可以访问vector对应位置的元素。

当对应位置的索引
#### Vec::get

`get`方法会返回