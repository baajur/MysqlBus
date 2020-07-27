/*
@author: xiao cai niao
@datetime: 2020/7/24
*/
use crate::mysql::Result;
use crate::{readvalue, MyError};
use crate::mysql::connection::AllUserInfo;
use crate::server::sql_parser::SqlStatement;
use crate::mysql::pool::{PlatformPool, MysqlConnectionInfo};
use std::collections::HashMap;
use tracing::{debug, info};

trait CheckSqlType{
    fn check_sql_type(&self, sql_type: &SqlStatement) -> bool;
    fn check_show_type(&self) -> bool;
}

impl CheckSqlType for User{
    fn check_sql_type(&self, sql_type: &SqlStatement) -> bool{
        return match sql_type {
            SqlStatement::Query => self.select.clone(),
            SqlStatement::Delete => self.delete.clone(),
            SqlStatement::Insert => self.insert.clone(),
            SqlStatement::Update => self.update.clone(),
            SqlStatement::Create => self.create.clone(),
            SqlStatement::Drop => self.drop.clone(),
            SqlStatement::AlterTable => self.alter.clone(),
            SqlStatement::ChangeDatabase => self.check_show_type(),
            _ => {
                false
            }
        }
    }
    fn check_show_type(&self) -> bool{
        if self.select || self.delete || self.insert || self.update || self.create || self.drop || self.alter{
            return true;
        }
        return false;
    }

}

impl CheckSqlType for DBPri{
    fn check_sql_type(&self, sql_type: &SqlStatement) -> bool{
        return match sql_type {
            SqlStatement::Query => self.select.clone(),
            SqlStatement::Delete => self.delete.clone(),
            SqlStatement::Insert => self.insert.clone(),
            SqlStatement::Update => self.update.clone(),
            SqlStatement::Create => self.create.clone(),
            SqlStatement::Drop => self.drop.clone(),
            SqlStatement::AlterTable => self.alter.clone(),
            SqlStatement::ChangeDatabase => self.check_show_type(),
            _ => {
                false
            }
        }
    }
    fn check_show_type(&self) -> bool{
        if self.select || self.delete || self.insert || self.update || self.create || self.drop || self.alter{
            return true;
        }
        return false;
    }
}

impl CheckSqlType for TablePri{
    fn check_sql_type(&self, sql_type: &SqlStatement) -> bool{
        return match sql_type {
            SqlStatement::Query => self.select.clone(),
            SqlStatement::Delete => self.delete.clone(),
            SqlStatement::Insert => self.insert.clone(),
            SqlStatement::Update => self.update.clone(),
            SqlStatement::Create => self.create.clone(),
            SqlStatement::Drop => self.drop.clone(),
            SqlStatement::AlterTable => self.alter.clone(),
            SqlStatement::ChangeDatabase => self.check_show_type(),
            _ => {
                false
            }
        }
    }
    fn check_show_type(&self) -> bool{
        if self.select || self.delete || self.insert || self.update || self.create || self.drop || self.alter{
            return true;
        }
        return false;
    }
}

//查询数据时mysql返回的字段元数据
#[derive(Debug, Clone)]
pub struct MetaColumn{
    pub catalog: String,
    pub schema: String,
    pub table: String,
    pub org_table: String,
    pub name: String,
    pub org_name: String,
    pub character_set: u16,
    pub column_length: u32,
    pub column_type: u8,
    pub flag: u16
}

impl MetaColumn{
    pub fn new(buf: &Vec<u8>) -> MetaColumn {
        let mut offset: usize = 0;
        let mut var_size = buf[0] as usize ; //字段信息所占长度
        offset += 1;
        let catalog = readvalue::read_string_value(&buf[offset..offset+var_size]);
        offset += var_size;

        var_size = buf[offset] as usize;
        offset += 1;
        let schema = readvalue::read_string_value(&buf[offset..offset+var_size]);
        offset += var_size;

        var_size = buf[offset] as usize;
        offset += 1;
        let table = readvalue::read_string_value(&buf[offset..offset+var_size]);
        offset += var_size;

        var_size = buf[offset] as usize;
        offset += 1;
        let org_table = readvalue::read_string_value(&buf[offset..offset+var_size]);
        offset += var_size;

        var_size = buf[offset] as usize;
        offset += 1;
        let name = readvalue::read_string_value(&buf[offset..offset+var_size]);
        offset += var_size;

        var_size = buf[offset] as usize;
        offset += 1;
        let org_name = readvalue::read_string_value(&buf[offset..offset+var_size]);
        offset += var_size;

        offset += 1;

        let character_set = readvalue::read_u16(&buf[offset..offset+2]);
        offset += 2;

        let column_length = readvalue::read_u32(&buf[offset..offset+4]);
        offset += 4;

        let column_type = buf[offset];
        offset +=1;

        let flag = readvalue::read_u16(&buf[offset..offset+2]);

        MetaColumn{
            catalog,
            schema,
            table,
            org_table,
            name,
            org_name,
            character_set,
            column_length,
            column_type,
            flag
        }
    }
}

/// 对应mysql中user表信息
#[derive(Clone, Debug)]
pub struct User{
    pub host: String,
    pub user: String,
    pub select: bool,
    pub update: bool,
    pub insert: bool,
    pub delete: bool,
    pub create: bool,
    pub drop: bool,
    pub alter: bool
}
impl User{
    fn new() -> User{
        User{
            host: "".to_string(),
            user: "".to_string(),
            select: false,
            update: false,
            insert: false,
            delete: false,
            create: false,
            drop: false,
            alter: false
        }
    }

    fn init_value(&mut self, row_value: &HashMap<String, String>) {
        if let Some(v) = row_value.get(&"Host".to_string()){
            self.host = v.clone();
        }
        if let Some(v) = row_value.get(&"User".to_string()){
            self.user = v.clone();
        }
        if let Some(v) = row_value.get(&"Select_priv".to_string()){
            if v == &"Y".to_string(){
                self.select = true;
            }
        }
        if let Some(v) = row_value.get(&"Insert_priv".to_string()){
            if v == &"Y".to_string(){
                self.insert = true;
            }
        }
        if let Some(v) = row_value.get(&"Update_priv".to_string()){
            if v == &"Y".to_string(){
                self.update = true;
            }
        }
        if let Some(v) = row_value.get(&"Delete_priv".to_string()){
            if v == &"Y".to_string(){
                self.delete = true;
            }
        }
        if let Some(v) = row_value.get(&"Create_priv".to_string()){
            if v == &"Y".to_string(){
                self.create = true;
            }
        }
        if let Some(v) = row_value.get(&"Drop_priv".to_string()){
            if v == &"Y".to_string(){
                self.drop = true;
            }
        }
        if let Some(v) = row_value.get(&"Alter_priv".to_string()){
            if v == &"Y".to_string(){
                self.alter = true;
            }
        }
    }
}

/// 对应mysql中table_pri表
#[derive(Clone, Debug)]
pub struct TablePri{
    pub host: String,
    pub user: String,
    pub db: String,
    pub table: String,
    pub select: bool,
    pub update: bool,
    pub insert: bool,
    pub delete: bool,
    pub create: bool,
    pub drop: bool,
    pub alter: bool
}

impl TablePri{
    fn new() -> TablePri{
        TablePri{
            host: "".to_string(),
            user: "".to_string(),
            db: "".to_string(),
            table: "".to_string(),
            select: false,
            update: false,
            insert: false,
            delete: false,
            create: false,
            drop: false,
            alter: false
        }
    }

    fn init_value(&mut self, row_value: &HashMap<String, String>){
        if let Some(v) = row_value.get(&"Host".to_string()){
            self.host = v.clone();
        }
        if let Some(v) = row_value.get(&"User".to_string()){
            self.user = v.clone();
        }
        if let Some(v) = row_value.get(&"Db".to_string()){
            self.db = v.clone();
        }
        if let Some(v) = row_value.get(&"Table_name".to_string()){
            self.table = v.clone();
        }
        if let Some(v) = row_value.get(&"Table_priv".to_string()){
            let value = v.to_lowercase();
            let value_ver = value.split(",");
            let value_ver = value_ver.collect::<Vec<&str>>();
            for i in &value_ver{
                match i.as_ref(){
                    "Select" => self.select = true,
                    "Update" => self.update = true,
                    "Delete" => self.delete = true,
                    "Insert" => self.insert = true,
                    "Alter" => self.alter = true,
                    "Drop" => self.drop = true,
                    "Create" => self.create = true,
                    _ => {}
                }
            }
        }
    }
}

/// 对应mysql中db表
#[derive(Clone, Debug)]
pub struct DBPri{
    pub host: String,
    pub user: String,
    pub db: String,
    pub select: bool,
    pub update: bool,
    pub insert: bool,
    pub delete: bool,
    pub create: bool,
    pub drop: bool,
    pub alter: bool
}

impl DBPri{
    fn new() -> DBPri{
         DBPri{
            host: "".to_string(),
            user: "".to_string(),
            db: "".to_string(),
            select: false,
            update: false,
            insert: false,
            delete: false,
            create: false,
            drop: false,
            alter: false
        }

    }

    fn init_value(&mut self, row_value: &HashMap<String, String>){
        if let Some(v) = row_value.get(&"Host".to_string()){
            self.host = v.clone();
        }

        if let Some(v) = row_value.get(&"User".to_string()){
            self.user = v.clone();
        }
        if let Some(v) = row_value.get(&"Db".to_string()){
            self.db = v.clone();
        }
        if let Some(v) = row_value.get(&"Select_priv".to_string()){
            if v == &"Y".to_string(){
                self.select = true;
            }
        }
        if let Some(v) = row_value.get(&"Update_priv".to_string()){
            if v == &"Y".to_string(){
                self.update = true;
            }
        }
        if let Some(v) = row_value.get(&"Delete_priv".to_string()){
            if v == &"Y".to_string(){
                self.delete = true;
            }
        }
        if let Some(v) = row_value.get(&"Insert_priv".to_string()){
            if v == &"Y".to_string(){
                self.insert = true;
            }
        }
        if let Some(v) = row_value.get(&"Create_priv".to_string()){
            if v == &"Y".to_string(){
                self.create = true;
            }
        }
        if let Some(v) = row_value.get(&"Drop_priv".to_string()){
            if v == &"Y".to_string(){
                self.drop = true;
            }
        }
        if let Some(v) = row_value.get(&"Alter_priv".to_string()){
            if v == &"Y".to_string(){
                self.alter = true;
            }
        }

    }
}


/// 记录用户权限信息
#[derive(Clone, Debug)]
pub struct UserPri{
    pub user: String,                   //用户名
    pub user_pri: Option<Vec<User>>,
    pub db_pri: Option<Vec<DBPri>>,
    pub table_pri: Option<Vec<TablePri>>
}

impl UserPri{
    pub fn new(user: &String) -> UserPri{
        UserPri{
            user: user.clone(),
            user_pri: None,
            db_pri: None,
            table_pri: None
        }
    }

    pub fn check_user_name(&self, user_name: &String) -> bool{
        if user_name == &self.user{
            return true;
        }
        return false
    }

    async fn check_privileges(&self, check_struct: &CheckPrivileges) -> Result<()>{
        if check_struct.check_information_schema(){return Ok(())}
        if self.check_user_privileges(check_struct).await{return Ok(())}
        info!("aa");
        if self.check_db_privileges(check_struct).await{return Ok(())}
        info!("bb");
        if self.check_table_privileges(check_struct).await{return Ok(())}
        info!("cc");
        let err = format!("Access denied for user '{}'@'{}' to tables info '{:?}'", &check_struct.user_name, &check_struct.host, &check_struct.cur_sql_table_info);
        return Err(Box::new(MyError(err.into())));
    }

    async fn check_user_privileges(&self, check_struct: &CheckPrivileges) -> bool{
        if let Some(user_pri_all) = &self.user_pri{
            for user_pri in user_pri_all{
                if check_host(&check_struct.host, &user_pri.host){
                    return user_pri.check_sql_type(&check_struct.sql_type);
                    //return self.check_sql_privileges(db_pri, check_struct).await;
                }
            }
        }
        return false;
    }

    async fn check_db_privileges(&self, check_struct: &CheckPrivileges) -> bool{
        if let Some(db_pri_all) = &self.db_pri{
            let mut true_count = 0;
            for db_pri in db_pri_all{
                if (&db_pri.db == &check_struct.cur_db || check_struct.check_cur_sql_db_info(&db_pri.db))
                    && check_host(&check_struct.host, &db_pri.host){
                    if db_pri.check_sql_type(&check_struct.sql_type){
                        true_count += 1;
                    }
                    //return self.check_sql_privileges(db_pri, check_struct).await;
                }
            }
            if true_count == check_struct.cur_sql_table_info.len(){
                return true;
            }
        }
        return false;
    }

    async fn check_table_privileges(&self, check_struct: &CheckPrivileges) -> bool{
        if let Some(tbl_pri_all) = &self.table_pri{
            let mut true_count = 0;
            for tbl_pri in tbl_pri_all{
                if check_host(&check_struct.host, &tbl_pri.host) &&
                    check_struct.check_cur_sql_table_info(&tbl_pri.db, &tbl_pri.table){
                    if tbl_pri.check_sql_type(&check_struct.sql_type){
                        true_count += 1;
                    }
                    //return self.check_sql_privileges(tbl_pri, check_struct).await;
                }
            }
            if true_count == check_struct.cur_sql_table_info.len(){
                return true;
            }
        }
        return false;
    }

    // async fn check_sql_privileges<F>(&self, pri_info: &F, check_struct: &CheckPrivileges) -> bool{
    //     return match check_struct.sql_type {
    //         SqlStatement::Query => pri_info.select,
    //         SqlStatement::Delete => pri_info.delete,
    //         SqlStatement::Insert => pri_info.insert,
    //         SqlStatement::Update => pri_info.update,
    //         SqlStatement::Create => pri_info.create,
    //         SqlStatement::Drop => pri_info.drop,
    //         SqlStatement::AlterTable => pri_info.alter,
    //         _ => {
    //             false
    //         }
    //     }
    // }
}

#[derive(Clone, Debug)]
pub struct AllUserPri{
    pub all_pri: Vec<UserPri>,
    pub platform_pool: PlatformPool
}

impl AllUserPri{
    pub fn new(platform_pool: &PlatformPool) -> AllUserPri{
        AllUserPri{
            all_pri: vec![],
            platform_pool: platform_pool.clone()
        }
    }

    /// 获取所有用户权限列表
    pub async fn get_pris(&mut self, all_user_info: &AllUserInfo) -> Result<()>{
        for (user, user_info) in &all_user_info.all_info{
            if &user_info.platform == &"admin".to_string(){
                continue;
            }
            let mut one_user_pri = UserPri::new(&user);
            if let Some(mut platform_pool_on) = self.platform_pool.get_platform_pool(&user_info.platform).await{
                let mut mysql_conn =platform_pool_on.get_pool(&SqlStatement::Query, &"".to_string()).await?;
                self.get_user_pri(&mut mysql_conn, &mut one_user_pri).await?;
                self.get_db_pri(&mut mysql_conn, &mut one_user_pri).await?;
                self.get_table_pri(&mut mysql_conn, &mut one_user_pri).await?;
                platform_pool_on.return_pool(mysql_conn, 0).await?;
                self.all_pri.push(one_user_pri);
            }
        }
        debug!("all user privileges: {:?}", &self.all_pri);
        Ok(())
    }

    /// 获取user表中权限信息
    async fn get_user_pri(&mut self, mysql_conn: &mut MysqlConnectionInfo, user_pri: &mut UserPri) -> Result<()>{
        let sql = format!("select * from mysql.user where user='{}'",user_pri.user);
        let value = mysql_conn.execute_command(&sql).await?;
        let mut pri = vec![];
        for row_value in value{
            let mut new_user_pri = User::new();
            new_user_pri.init_value(&row_value);
            pri.push(new_user_pri);
        }
        if pri.len() > 0{
            user_pri.user_pri = Some(pri);
        }
        Ok(())
    }

    /// 获取db表中权限信息
    async fn get_db_pri(&mut self, mysql_conn: &mut MysqlConnectionInfo, user_pri: &mut UserPri) -> Result<()>{
        let sql = format!("select * from mysql.db where user='{}'",user_pri.user);
        let value = mysql_conn.execute_command(&sql).await?;
        let mut db_pri = vec![];
        for row_value in value{
            let mut new_db_pri = DBPri::new();
            new_db_pri.init_value(&row_value);
            db_pri.push(new_db_pri);
        }
        if db_pri.len() > 0{
            user_pri.db_pri = Some(db_pri);
        }
        Ok(())
    }

    /// 获取table_pri表中的table权限信息
    async fn get_table_pri(&mut self, mysql_conn: &mut MysqlConnectionInfo, user_pri: &mut UserPri) -> Result<()>{
        let sql = format!("select * from mysql.tables_priv where user='{}'",user_pri.user);
        let value = mysql_conn.execute_command(&sql).await?;
        let mut tbl_pri = vec![];
        for row_value in value{
            let mut new_tbl_pri = TablePri::new();
            new_tbl_pri.init_value(&row_value);
            tbl_pri.push(new_tbl_pri);
        }
        if tbl_pri.len() > 0{
            user_pri.table_pri = Some(tbl_pri);
        }
        Ok(())
    }

    /// 判断用户权限
    ///
    /// 按权限从大到小的优先级进行判断，新判断user表中、再db表最后判断table表
    pub async fn check_privileges(&self, check_struct: &CheckPrivileges) -> Result<()>{
        for user_pri in &self.all_pri{
            if user_pri.check_user_name(&check_struct.user_name){
                user_pri.check_privileges(check_struct).await?;
                return Ok(())
            }
        }
        let err = format!("Access denied for user '{}'@'{}'", &check_struct.user_name, &check_struct.host);
        return Err(Box::new(MyError(err.into())))
    }

}

#[derive(Clone, Debug)]
pub struct TableInfo{
    pub db: Option<String>,
    pub table: String
}
/// 检查权限所需的结构体
#[derive(Clone, Debug)]
pub struct CheckPrivileges{
    pub cur_db: String,                         //记录当前连接默认db
    pub cur_sql_table_info: Vec<TableInfo>,     //记录当前执行sql使用的db、table信息
    pub sql_type: SqlStatement,                 //当前sql类型
    pub user_name: String,                      //当前使用的用户名
    pub host: String                            //当前连接愿IP
}
impl CheckPrivileges{
    pub fn new(cur_db: &Option<String>, sql_table_info: Vec<String>, sql_type: &SqlStatement, user_name: &String, host: &String) -> CheckPrivileges{
        let mut my_cur_db;
        if let Some(db) = cur_db{
            my_cur_db = db.clone();
        }else {
            my_cur_db = "".to_string();
        }

        let mut cur_sql_table_info = vec![];
        match sql_type{
            SqlStatement::ChangeDatabase => {
                my_cur_db = sql_table_info[0].clone();
            }
            _ => {
                for i in sql_table_info{
                    let one_vec = i.split(".");
                    let one_vec = one_vec.collect::<Vec<&str>>();
                    if one_vec.len() > 1{
                        cur_sql_table_info.push(TableInfo{db: Some(one_vec[0].to_string().clone()), table: one_vec[1].to_string().clone()});
                    }else {
                        cur_sql_table_info.push(TableInfo{db: None, table: one_vec[0].to_string().clone()});
                    }
                }
            }
        }

        CheckPrivileges{
            cur_db: my_cur_db,
            cur_sql_table_info,
            sql_type: sql_type.clone(),
            user_name: user_name.clone(),
            host: host.clone()
        }
    }
    fn check_cur_sql_db_info(&self, db: &String) -> bool{
        for table_info in &self.cur_sql_table_info{
            if let Some(my_db) = &table_info.db{
                if my_db == db {
                    return true;
                }
            }
        }
        return false;
    }

    fn check_cur_sql_table_info(&self, db: &String, table: &String) -> bool{
        for table_info in &self.cur_sql_table_info{
            if let Some(my_db) = &table_info.db{
                if my_db == db {
                    if table == &table_info.table{
                        return true;
                    }
                }
            }else {
                if &self.cur_db == db{
                    if table == &table_info.table{
                        return true;
                    }
                }
            }

        }
        return false;
    }

    fn check_information_schema(&self) -> bool{
        for tble_all in &self.cur_sql_table_info{
            if let Some(db) = &tble_all.db{
                if db == &String::from("information_schema"){
                    continue;
                }else {
                    return false;
                }
            }else {
                if &self.cur_db == &String::from("information_schema"){
                    continue;
                }else {
                    return false;
                }
            }
        }

        if &self.cur_db == &String::from("information_schema"){
            return true;
        }
        return false;
    }
}


/// 匹配地址
fn check_host(cur_host: &String, pri_host: &String) -> bool{
    if cur_host == pri_host{
        return true;
    }
    let mut cur_host_chars = cur_host.chars();
    let mut pri_host_chars = pri_host.chars();
    loop{
        if let Some(a) = cur_host_chars.next(){
            if let Some(b) = pri_host_chars.next(){
                if a == b{
                    continue
                }else {
                    if format!("{}",b) == "%".to_string(){
                        return true;
                    }
                    return false;
                }
            }else {
                return false;
            }
        }else {
            return false;
        }
    }
}