use rusqlite::Connection;

// create database student with columns: 
//   first_name, last_name, index_nr, speciality 

#[derive(Debug)]
struct Student{
    id: i32,
    first_name: String,
    last_name: String,
    index_nr: String,
    speciality: String
}

fn main() {
    println!("Hello, world!");

    let conn = Connection::open_in_memory().unwrap();

    conn.execute(
        "
        create table student (
            id integer primary key,
            first_name text not null,
            last_name text not null,
            index_nr text,
            speciality text
        )", ()).unwrap();

    
    let sample_student = Student{
        id: 1,
        first_name: "mike".to_string(),
        last_name: "jagger".to_string(),
        index_nr: "324121".to_string(),
        speciality: "rock".to_string()
    };

    conn.execute(
        "insert into student 
            (id, first_name, last_name, index_nr, speciality) values (?1, ?2, ?3, ?4, ?5)",
         (&sample_student.id, &sample_student.first_name, &sample_student.last_name, 
            &sample_student.index_nr, &sample_student.speciality)).unwrap();

    

    let mut statement = conn.prepare(
        "select id, first_name, last_name, index_nr, speciality from student"
    ).unwrap();

    let students = statement.query_map([], |row| {
        Ok(Student{
            id: row.get(0).unwrap(),
            first_name: row.get(1).unwrap(),
            last_name: row.get(2).unwrap(),
            index_nr: row.get(3).unwrap(),
            speciality: row.get(4).unwrap()
        })

    }).unwrap();

    for student in students{
        println!("student: {:?}", student.unwrap());
    }
    
}
