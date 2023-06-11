#[cfg(feature = "sled-storage")]
mod api_usage {
    use {
        futures::executor::block_on,
        gluesql::prelude::{Glue, SledStorage},
    };

    fn mutable_api() {
        // filename을 string slice로 입력하여 tree에서 id ofsset을 찾고, 유휴한 상태로 초기화하며, transaction의 타임아웃을 설정(ms)
        let storage: SledStorage = SledStorage::new("data/mutable-api").unwrap();

        // 저장 공간을 GlueSQL에 할당
        let mut glue: Glue<SledStorage> = Glue::new(storage);

        let sqls = [
            // id를 컬럼으로 가지는 테이블을 생성
            "CREATE TABLE Glue (id INTEGER);",
            // id = 100 row 입력
            "INSERT INTO Glue VALUES (100);",
            // id = 200 row 입력
            "INSERT INTO Glue VALUES (200);",
            // 테이블을 drop
            "DROP TABLE Glue;",
        ];

        // vector를 순회하면서 sql을 실행하고 unwrap으로 결괏값을 추출함.
        for sql in sqls {
            glue.execute(sql).unwrap();
        }
    }

    // 비동기로 작동하면 오류는 안 발생하는가?
    async fn async_mutable_api() {
        let storage: SledStorage = SledStorage::new("data/async-mutable-api").unwrap();
        let mut glue: Glue<SledStorage> = Glue::new(storage);

        let sqls: [&str; 4] = [
            "CREATE TABLE Glue (id INTEGER);",
            "INSERT INTO Glue VALUES (100);",
            "INSERT INTO Glue VALUES (200);",
            "DROP TABLE Glue;",
        ];

        for sql in sqls {
            glue.execute_async(sql).await.unwrap();
        }
    }

    pub fn run() {
        mutable_api();
        block_on(async_mutable_api());
    }
}

fn main() {
    #[cfg(feature = "sled-storage")]
    api_usage::run();
}
