use crate::{ASK_USER, questions};
use crate::input_reader::{DropDownVal, Input};
use crate::writer::try_write;

pub struct AskUser<'a> {
    pub open: &'a str,
    pub drop_down: &'a str,
}

impl Input {
    pub(crate) fn open_question(
        mut self,
        question: &str,
        num_mode: bool,
        arrow_mode: bool,
    ) -> Self {
        let scouting1 = self.n_or_val(&format!("{}: {}", question, ASK_USER.open));
        if let Some(what) = scouting1 {
            let val = what.trim().split(',');
            for add_val in questions::write_questions_firestore_opened(val, num_mode) {
                if num_mode && arrow_mode {
                    self.arrow_vec.push(add_val);
                } else if num_mode {
                    self.num_question_vec.push(add_val);
                } else {
                    self.question_vec.push(add_val);
                }
            }
            if !self.first_done {
                self.first_done = true;
            }
        }
        self
    }

    pub(crate) fn drop_question(mut self, question: &str, but: bool, many: bool) -> Self {
        loop {
            let pit2 = self.n_or_val(&format!("{}: {}", question, ASK_USER.drop_down));
            if let Some(what) = pit2 {
                let val = what.trim().split('|');
                match questions::write_questions_firestore_drop_down(val) {
                    Ok((head, vals)) => {
                        self = self.where_add(but, many, head, vals);
                        self.first_done = true;
                        break;
                    }
                    Err(data) => println!("{}", data),
                }
            } else {
                break;
            }
        }
        self
    }
    fn where_add(mut self, but: bool, many: bool, header: Vec<String>, body: Vec<String>) -> Self {
        for x in 0..header.len() {
            if but {
                self.multi_select_question.head.push(header[x].clone());
                self.multi_select_question.val.push(body[x].clone());
            } else if many {
                self.many_choice.head.push(header[x].clone());
                self.many_choice.val.push(body[x].clone());
            } else {
                self.drop_down.head.push(header[x].clone());
                self.drop_down.val.push(body[x].clone());
            }
        }
        self
    }
    pub fn next_question(self) -> Self {
        try_write("//new question", false);
        self
    }
    pub fn new() -> Self {
        better_file_maker::make_folders("output").unwrap_or(());
        try_write("Map<String, dynamic> initialData = Map<String, dynamic>();\n\nthis.initialData = const {
        'Header': 'Match Scouting',
        'Team Number': 0,", true);
        println!("output file created!");
        Self {
            first_done: true,
            question_vec: vec![],
            num_question_vec: vec![],
            drop_down: DropDownVal {
                head: vec![],
                val: vec![],
            },
            many_choice: DropDownVal {
                head: vec![],
                val: vec![],
            },
            multi_select_question: DropDownVal {
                head: vec![],
                val: vec![],
            },

            arrow_vec: vec![],
        }
    }
    pub fn end(self) {
        try_write(
            "};\nList<Question>? matchFormQuestions;\nmatchFormQuestions = [\
         ShortAnswer(
        'Team Number',
        TextInputType.number,
        initialValue: widget.initialData['Team Number'],
      ),",
            false,
        );
        for val in &self.num_question_vec {
            try_write(format!("ShortAnswer(\n{},\nTextInputType.number,\ninitialValue: widget.initialData[{}],\n),", val, val), false);
        }
        for val in &self.question_vec {
            try_write(format!("ShortAnswer(\n{},\nTextInputType.text,\ninitialValue: widget.initialData[{}],\n),", val, val), false);
        }
        for val in &self.arrow_vec {
            try_write(
                format!(
                    "UpDownArrowQuestion({},\ncounter: widget.initialData[{}],\n),",
                    val, val
                ),
                false,
            );
        }
        self.write_list_widget("drop");
        self.write_list_widget("multi");
        self.write_list_widget("many");
        try_write("];", false);
    }
    fn write_list_widget(&self, mode: &str)  {
        let (what, vers)  = {
            match mode {
                "drop" => {
                    (&self.drop_down , "DropDownQuestion")
                }
                "multi" => {
                    (&self.many_choice , "MultiSelectQuestion")
                }
                "many" => {
                    (&self.many_choice , "MultipleChoiceQuestion")
                }
                _ => {
                    panic!("bad use of mode widget")
                }
            }
        };
        for location in 0..what.head.len() {
            let header: &str = what.head.get(location).expect("Ram Corruption Error, Please Try Again and make sure power is being supplied to your pc");
            try_write(
                format!(
                    "{}(\n'{}',\n{}\n,answer: widget.initialData['{}']\n),",
                    vers,
                    header,
                    what
                        .val
                        .get(location)
                        .unwrap_or(&"error".to_owned()),
                    header
                ),
                false,
            );
        }
    }
}
//ShortAnswer(
//         'Team Number',
//         TextInputType.number,
//         initialValue: widget.initialData['Team Number'],
//       ),

//DropDownQuestion(
//         'Starting Rung',
//         ['None', 'Low', 'Middle'],
//         answer: widget.initialData['Starting Rung'],
//       ),
