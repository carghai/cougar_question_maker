mod widgets;

use std::str::Split;


pub fn write_questions_firestore_opened(what: Split<char>, name : &str ,replace : bool) {
    let what_info: String = {
        let mut return_data: String = "".to_owned();
        for info in what {
            return_data = format!("'{}',{}", info, return_data);
        }
        return_data
    };
    crate::writer::try_write(format!("const List<String> {} = [{}];",name, what_info), replace);
}

pub fn write_questions_firestore_drop_down<'a>(what: Split<'a, char>, name : &'a str) -> Result<() , &'a str>{
    let what_info: String = {
        let mut return_data: String = "".to_owned();
        for info in what {
            let data = info.split(',');
            let mut section = "".to_owned();
            if data.clone().count() < 2 {
                return Err("Syntax Error, Please Make it Like this:  Question,answer1,answer2|Question2,answer1,answer2");
            }
            for cool in data {
                section = format!("'{}',{}", cool, return_data);
            }
            return_data = format!("[{}],{}", info, return_data);
        }
        return_data
    };
    crate::writer::try_write(format!("const List<List<String>> {} = [{}];",name, what_info), false);
    Ok(())
}
//
// pub fn write_widgets(what: Split<char>) {
//     let wid = widgets::DartWidget {};
//     let write_info: String = {
//         let mut val = "\n".to_owned();
//         for make in what {
//             val = format!("{}\n{}", wid.make_text_place_holder(make), val)
//         }
//         val
//     };
//     crate::writer::try_write(format!("\n\n\n{}", write_info), false);
// }


