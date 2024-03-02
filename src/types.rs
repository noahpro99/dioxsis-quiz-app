#[derive(Debug, PartialEq)]
pub struct Quiz {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub questions: Vec<Question>,
}

#[derive(Debug, PartialEq)]
pub struct Question {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub options: Vec<QuestionOption>,
}

#[derive(Debug, PartialEq, Clone)]

pub struct QuestionOption {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub is_correct: bool,
}

pub fn get_quizzes() -> Vec<Quiz> {
    let quizzes = vec![
        Quiz {
            id: 1,
            title: "Quiz 1".to_string(),
            description: "This is the first quiz".to_string(),
            questions: vec![
                Question {
                    id: 1,
                    title: "Question 1".to_string(),
                    description: "This is the first question".to_string(),
                    options: vec![
                        QuestionOption {
                            id: 1,
                            title: "Option 1".to_string(),
                            description: "This is the first option".to_string(),
                            is_correct: true,
                        },
                        QuestionOption {
                            id: 2,
                            title: "Option 2".to_string(),
                            description: "This is the second option".to_string(),
                            is_correct: false,
                        },
                        QuestionOption {
                            id: 3,
                            title: "Option 3".to_string(),
                            description: "This is the third option".to_string(),
                            is_correct: false,
                        },
                        QuestionOption {
                            id: 4,
                            title: "Option 4".to_string(),
                            description: "This is the fourth option".to_string(),
                            is_correct: false,
                        },
                    ],
                },
                Question {
                    id: 2,
                    title: "Question 2".to_string(),
                    description: "This is the second question".to_string(),
                    options: vec![
                        QuestionOption {
                            id: 5,
                            title: "Option 5".to_string(),
                            description: "This is the fifth option".to_string(),
                            is_correct: false,
                        },
                        QuestionOption {
                            id: 6,
                            title: "Option 6".to_string(),
                            description: "This is the sixth option".to_string(),
                            is_correct: true,
                        },
                        QuestionOption {
                            id: 7,
                            title: "Option 7".to_string(),
                            description: "This is the seventh option".to_string(),
                            is_correct: false,
                        },
                        QuestionOption {
                            id: 8,
                            title: "Option 8".to_string(),
                            description: "This is the eighth option".to_string(),
                            is_correct: false,
                        },
                    ],
                },
            ],
        },
        Quiz {
            id: 1,
            title: "Quiz 1".to_string(),
            description: "This is the first quiz".to_string(),
            questions: vec![
                Question {
                    id: 1,
                    title: "Question 1".to_string(),
                    description: "This is the first question".to_string(),
                    options: vec![
                        QuestionOption {
                            id: 1,
                            title: "Option 1".to_string(),
                            description: "This is the first option".to_string(),
                            is_correct: true,
                        },
                        QuestionOption {
                            id: 2,
                            title: "Option 2".to_string(),
                            description: "This is the second option".to_string(),
                            is_correct: false,
                        },
                        QuestionOption {
                            id: 3,
                            title: "Option 3".to_string(),
                            description: "This is the third option".to_string(),
                            is_correct: false,
                        },
                        QuestionOption {
                            id: 4,
                            title: "Option 4".to_string(),
                            description: "This is the fourth option".to_string(),
                            is_correct: false,
                        },
                    ],
                },
                Question {
                    id: 2,
                    title: "Question 2".to_string(),
                    description: "This is the second question".to_string(),
                    options: vec![
                        QuestionOption {
                            id: 5,
                            title: "Option 5".to_string(),
                            description: "This is the fifth option".to_string(),
                            is_correct: false,
                        },
                        QuestionOption {
                            id: 6,
                            title: "Option 6".to_string(),
                            description: "This is the sixth option".to_string(),
                            is_correct: true,
                        },
                        QuestionOption {
                            id: 7,
                            title: "Option 7".to_string(),
                            description: "This is the seventh option".to_string(),
                            is_correct: false,
                        },
                        QuestionOption {
                            id: 8,
                            title: "Option 8".to_string(),
                            description: "This is the eighth option".to_string(),
                            is_correct: false,
                        },
                    ],
                },
            ],
        },
    ];
    quizzes
}
