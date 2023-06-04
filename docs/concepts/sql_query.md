# SQL 查询工具

## 一、目标

设计一个支持对 csv 文件进行 SQL 查询的 rust 库，并且支持使用 Python3 和 Node.js 调用。

## 二、设计分析

### 1.一条 SQL 语句的执行流程

首先输入一条语句，然后进行词法分析，将语句分解成一个个 token，然后进行语法分析，将 token 组合成一个语法树，最后进行语义分析，将语法树转换成一个执行计划，然后执行计划进行执行，最后输出结果。更详细的执行流程见：[一文读懂 MySQL 底层架构实现](https://blog.csdn.net/qq_41345173/article/details/109692384)。

### 2.功能拆分

- SQL 语句解析 ([sqlparser-rs](https://github.com/sqlparser-rs/sqlparser-rs))

- 数据格式化，即把不同的数据源转换为抽象的 DataFrame ([polars-rs](https://docs.rs/polars/latest/polars/#))
