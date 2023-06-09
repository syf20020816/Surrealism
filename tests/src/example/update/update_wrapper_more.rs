use surrealism::{DefaultInitServiceImpl, SurrealRes, Wrapper, UpdateWrapper, UseWrapper, Criteria, TimeUnit, TableId};

#[tokio::main]
async fn main() -> SurrealRes<()> {
    ///初始化连接
    ///init connection
    let db = DefaultInitServiceImpl::new().init().unwrap();
    ///创建UseWrapper
    /// new UseWrapper
    let mut use_wrapper = UseWrapper::new();
    /// 设置命名空间和数据库
    /// Set namespace and database
    use_wrapper.use_ns("test").use_db("test");
    /// 提交语句
    /// commit statement
    let res_use = db.use_commit(&mut use_wrapper).await;
    dbg!(res_use);
    /// 构建UpdateWrapper
    /// UPDATE user:1008 SET name = 'Stewie' , userId = 'stewie001' , age = 3 RETURN AFTER TIMEOUT 2m;
    let mut update_wrapper = UpdateWrapper::new();
    update_wrapper
        .from("user")
        .id(TableId::<String>::Str("1008".to_string()))
        .set("name", "Stewie")
        .set("userId", "stewie001")
        .set("age", 3)
        .return_after()
        .timeout(2, TimeUnit::MINUTE);
    Ok(())
}