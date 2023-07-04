use actix_web::web;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Course {
    pub tutor_id: usize,
    pub course_id: Option<usize>,
    pub course_name: String,
    pub posted_time: Option<NaiveDateTime>,
}

/*
   先ほど、Actix Webサーバーに登録されたアプリケーションの状態は、抽出器web::Data<T>を使用してハンドラが利用できるようになることを見ました。
   同様に、受信リクエストボディのデータは、抽出器web::Json<T>を使ってハンドラ関数が利用できるようになります。
*/
impl From<web::Json<Course>> for Course {
    fn from(course: web::Json<Course>) -> Self {
        Course {
            tutor_id: course.tutor_id,
            course_id: course.course_id,
            course_name: course.course_name.clone(),
            posted_time: course.posted_time,
        }
    }
}

/*
私たちはコースのデータモデルを定義しました。さて、コースが追加された場合、どのように保存するのでしょうか？
私たちはリレーショナルデータベースまたは同様の永続的なデータストアを使用したくありません。そこで、より簡単なオプションから始めましょう。
先ほど、Actixには複数の実行スレッド間でアプリケーションの状態を共有する機能があることを見ました。この機能をインメモリ・データストアに使ってみてはどうだろうか。
先ほど、tutor-nodb/src/state.rsでAppState構造体を定義し、訪問回数を追跡するようにしました。この構造体を拡張して、コースコレクションも保存するようにしましょう。

-> tutor-nodb/src/state.rs
 */
