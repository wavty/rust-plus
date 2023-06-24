use sqlparser::dialect::Dialect;

#[derive(Debug, Default)]
pub struct TyrDialect;

// 创建自己的 sql dialect。TyrDialect 支持 identifier 可以是一个简单的 url
impl Dialect for TyrDialect {
    fn is_identifier_start(&self, ch: char) -> bool {
        ch.is_alphabetic() || ch == '_'
    }

    fn is_identifier_part(&self, ch: char) -> bool {
        ch.is_alphanumeric() || [':', '/', '?', '=', '-', '_', '.', '&'].contains(&ch)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlparser::parser::Parser;

    // 测试辅助函数
    pub fn example_sql() -> String {
        let url = "https://raw.githubusercontent.com/owid/covid-19-data/master/public/data/owid-covid-data.csv";

        let sql = format!(
            "SELECT location name, total_cases, new_cases, total_deaths, new_deaths \
        FROM {} \
        WHERE new_deaths >= 500 ORDER BY new_cases DESC LIMIT 6 OFFSET 5",
            url
        );
        sql
    }

    #[test]
    fn it_works() {
        assert!(Parser::parse_sql(&TyrDialect::default(), &example_sql()).is_ok());
    }
}
