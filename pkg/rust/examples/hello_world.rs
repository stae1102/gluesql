#[cfg(feature = "sled-storage")]
mod hello_world {
    use {
        gluesql::{
            prelude::{Glue, Payload, Value},
            sled_storage::SledStorage,
        },
        std::fs,
    };

    pub fn run() {
        /*
            Initiate a connection
        */
        /*
            Open a Sled database, this will create one if one does not yet exist
        */
        let sled_dir: &str = "/tmp/gluesql/hello_world";
        // 모든 콘텐츠를 삭제한다.
        fs::remove_dir_all(sled_dir).unwrap_or(());
        let storage: SledStorage = SledStorage::new(sled_dir).expect("Something went wrong!");
        /*
            Wrap the Sled database with Glue
        */
        let mut glue: Glue<SledStorage> = Glue::new(storage);

        /*
            Create table then insert a row

            Write queries as a string
        */
        let queries: &str = "
            CREATE TABLE greet (name TEXT);
            INSERT INTO greet VALUES ('World');
        ";

        glue.execute(queries).expect("Execution failed");

        /*
            Select inserted row
        */
        let queries: &str = "
            SELECT name FROM greet
        ";

        // raw query를 실행할 때는 execute 메서드 사용
        let result: Vec<Payload> = glue.execute(queries).expect("Failed to execute");

        /*
            Query results are wrapped into a payload enum, on the basis of the query type
            Payload Enum으로 덮인 쿼리 결과를 rows에 레퍼런스 벡터로 담는다.
        */
        assert_eq!(result.len(), 1);
        let rows: &Vec<Vec<Value>> = match &result[0] {
            Payload::Select { labels: _, rows } => rows,
            _ => panic!("Unexpected result: {:?}", result),
        };

        // 첫 번째 row
        let first_row: &Vec<Value> = &rows[0];

        /* 
            이터레이터의 next() 메서드로 값 추출

            Value enum으로 덮인 값을 추출
        */
        let first_value: &Value = first_row.iter().next().unwrap();

        /*
            Row values are wrapped into a value enum, on the basis of the result type
        */
        let to_greet: &String = match first_value {
            Value::Str(to_greet) => to_greet,
            value => panic!("Unexpected type: {:?}", value),
        };

        println!("Hello {}!", to_greet); // Will always output "Hello World!"

        let insert_query: &str = "
            UPDATE greet SET name = 'glue' WHERE name = 'World';
        ";

        glue.execute(insert_query).expect("Execution failed");

        let result_after_updating: Vec<Payload> = glue.execute(queries).expect("Failed to execute");
        
        assert_eq!(result_after_updating.len(), 1);

        let rows_after_updating: &Vec<Vec<Value>> = match &result_after_updating[0] {
            Payload::Select { labels: _, rows } => rows,
            _ => panic!("Unexpected result: {:?}", result_after_updating),
        };

        let second_row = &rows_after_updating[0];

        let second_value = second_row.iter().next().unwrap();

        let to_glue: &String = match second_value {
            Value::Str(to_glue) => to_glue,
            value => panic!("Unexpected type:{:?}", value),
        };

        println!("User has been changed to {}!", to_glue);
    }
}

fn main() {
    #[cfg(feature = "sled-storage")]
    hello_world::run();
}
