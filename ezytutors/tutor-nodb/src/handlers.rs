use super::state::AppState;
use actix_web::{web, HttpResponse};
use super::models::Course;
use chrono::Utc;

pub async fn health_check_handler(app_state: web::Data<AppState>) -> HttpResponse {
    let health_check_response = &app_state.health_check_response;
    let mut visitor_count = app_state.visitor_count.lock().unwrap();
    let response = format!("{} {} times", health_check_response, visitor_count);
    *visitor_count += 1;
    HttpResponse::Ok().json(&response)
}

pub async fn new_course(
    new_course: web::Json<Course>,
    app_state: web::Data<AppState>
) -> HttpResponse {
    println!("Received new course");
    let course_count_for_user = app_state
        .courses
        .lock()
        .unwrap()
        .clone()
        .into_iter()
        .filter(|course| course.tutor_id == new_course.tutor_id)
        .count();
    let new_course = Course {
        tutor_id: new_course.tutor_id,
        course_id: Some(course_count_for_user + 1),
        course_name: new_course.course_name.clone(),
        posted_time: Some(Utc::now().naive_utc()),
    };
    // app_state.courses.push(new_course);
    app_state.courses.lock().unwrap().push(new_course);
    HttpResponse::Ok().json("Added course")
}

/*
// lockを二回取得しているけど，atomicになっていないのでは？
pub async fn new_course(
    new_course: Web::Json<Course>,
    app_state: web::Data<AppState>,
) -> HttpResponse {
    println!("Received new course");

    let mut courses = app_state.courses.lock().unwrap();
    let course_count_for_user = courses
        .iter()
        .filter(|course| course.tutor_id == new_course.tutor_id)
        .count();

    let mut new_course = new_course.into_inner();
    new_course.course_id = Some(course_count_for_user + 1);
    new_course.posted_time = Some(Utc::now().naive_utc());

    courses.push(new_course);

    HttpResponse::Ok().json("Added course")
}
*/