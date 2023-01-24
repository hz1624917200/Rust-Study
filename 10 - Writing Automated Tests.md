# 10 - Writing Automated Tests

> &emsp;&emsp;测试是保障程序正确性的重要手段。功能上的漏洞不能直接被编译器检测和捕获，必须通过执行一些测试例并比较期望结果得出。手动编写测试用例是一个较为冗长繁琐的过程，Rust提供了一些自动化测试方法，能够标准化、高效化进行功能的自动测试。

## 编写测试

Rust中的测试是一个**测试函数**，用于验证目标代码是否能够按照期望的方式运行，并输出期望结果。测试函数函数体一般包含三个部分：

1. 准备所需的数据或状态
2. 调用需要测试的代码
3. 使用**断言 assert **检验运行结果与期望输出是否一致

### 测试函数

Rust的测试函数是一个标注有`test`属性的函数。**属性（attribute）**元数据用于修饰Rust代码（`derive`也是）。只要标注`test`属性，命令行`cargo test`就会逐一调用所有的`test`函数，并生成报告。

创建library类型的工程，模板文件中自带单元测试样例：

```rust
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
```

* 测试函数由`assert`族宏构成，每个`assert`宏都是一个测试样例
  * 上例使用了`assert_eq`，顾名思义，该宏将判断左右参数是否相等
  * 还有一个是`assert_ne`，即不相等。还有最通用的`assert`，判断布尔表达式
  * **`assert`除了第一个表达式参数后，还可以有`panic`**宏的format string，以及后续参数。这些参数将被传递给`panic`，用于错误提示消息输出
* 当一个测试函数的所有`assert`通过后，会输出一个测试通过的信息，并将通过数+1
* 如果一个测试函数的某个`assert`未通过，则输出错误信息（与普通panic相同），但仍会继续执行测试，并最终输出统计信息

### 进行测试

为了测试，另行编写一个测试函数如下：

```rust
#[test]
fn my_test() {
    assert_eq!(2, 3);
}
```

运行`cargo test`，输出如下：

```
running 2 tests		// progress
test tests::it_works ... ok
test my_test ... FAILED

failures:		// part 1

---- my_test stdout ----
thread 'my_test' panicked at 'assertion failed: `(left == right)`
  left: `2`,
 right: `3`', src\lib.rs:7:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:		// part 2
    my_test
				// summary
test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass `--lib`
NativeCommandExitException: Program "cargo.exe" ended with non-zero exit code: 101.
```

输出主要由四部分构成（不包含`Doc-tests`，遇到测试失败）:

* progress 测试运行中，输出测试进度（通用）
* part 1: 运行失败，各失败的标准输出（即panic输出信息）
* part 2: 运行失败的测试列表
* summary: 测试统计摘要（通用）

### 其他测试语法

可以在`test`属性的基础上，额外增加`should_panic`属性，用于检测对象是否按照预期的情况`panic`

**当测试函数运行过程中发生panic，则通过测试，否则不通过**。

```rust
#[test]
#[should_panic]
fn some_panic_test() {
    ...
}
```

另外，为`should_panic`属性增加`expected`参数，可以指定panic输出的消息

```rust
#[should_panic(expected="Some expected panic info")]
```

### 使用Result编写测试

也可以使用`Result`编写测试，测试函数的返回值是`Result<T, E>`，当测试函数返回`Err`变体，就会被统计到测试失败中。

使用该方法可以在函数中调用`?`语法糖，编写测试程序更加方便。



## 控制测试的运行方式

默认情况下，`cargo test`生成的二进制文件会**默认并行运行所有测试，并截获结果信息**。

有关`cargo test`测试的更多参数，可以使用`--help`参数查看

* `test_threads`指定并行测试的线程数
* `no-capture`不截获正确测试例的标准输出

* `ignored` 单独运行被忽略的测试
  * 为测试函数添加`ignore`属性，可以将这些函数排除在测试外，避免过于消耗资源的测试频繁运行



## 测试的组织结构

### 单元测试

单元测试只测试一小段代码的正确性。将项目分成一些小单元，分别对单元进行测试，能够大大降低整合测试的复杂度，优化排错效率。

**单元测试模块通常放在每个源代码中，并以`#[cfg(test)]`标注**，该标注使编译器**只在运行`cargo test`时才编译运行该模块**

单元测试**仍然可以测试私有函数**

### 集成测试

继承测试完全位于代码库之外，**通过调用库的接口**完成测试，也就是只测试公开的那部分API功能。当然，部分功能也包含于这些API中。当然，集成测试也可能得到一些单元测试测不出来的bug，更加贴近于用户。

集成测试需要建立`tests`目录，cargo会自动寻找集成测试文件，并对每一个文件处理为独立的包。

在集成测试文件中，使用`use`引用被测目标，其他测试函数的编写方式与单元测试相同（可以不需要mod和`#[cfg(test)]`）

如果想在集成测试中使用一些通用函数，可以将其放入`tests/common/mod.rs`中，这可以被rust识别，并且不会被编入普通测试