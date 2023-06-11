#[cfg(feature = "sled-storage")]
mod utilize_various {
    use gluesql_core::ast_builder::{table, Build};

    use {
        gluesql::prelude::{Glue, JsonStorage},
    };

    // Json 스토리지에 테이블 생성 및 조회
    pub fn hello_glue_api_from_json() {
        let json_storage = JsonStorage::new("data/hello_glue").unwrap();
        let mut glue = Glue::new(json_storage);

        let drop_table_if_exists = table("Hello").drop_table_if_exists().build();
        
        let create_table_query = table("Hello")
            .create_table()
            .add_column("id INTEGER")
            // Gluesql에서는 string type으로 TEXT 타입만 제공
            .add_column("name TEXT")
            .build();

        let insert_values_query = table("Hello")
            .insert()
            .columns("id, name")
            .values(vec![
                "1, 'glue'",
                "2, 'seongtae'",
                "3, 'sql'",
            ])
            .build();
        
        let select_query = table("Hello")
        .select().project("*").build();
    
        let sqls = [
            drop_table_if_exists,
            create_table_query,
            insert_values_query,
            select_query,
        ];

        for sql in sqls {
            match sql {
                Ok(sql) => {
                    let result = glue.execute_stmt(&sql).unwrap();
                    println!("{:?}", result);
                }
                Err(e) => {
                    println!("{:?}", e);
                }
            }
        }

    }

}

fn main() {
    #[cfg(feature = "json-storage")]
    utilize_various::hello_glue_api_from_json();
}
