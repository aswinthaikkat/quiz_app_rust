use clap::{App, Arg};
use std::fs;
use std::io;
use std::path::PathBuf;
//use serde_json::{ Value};
use std::result::Result;
use rand::seq::SliceRandom;
//use textwrap;
//use colored::*;
use std::error::Error;



fn main() {
	
   let matches = App::new("Quiz Maker")
        .version("0.1.0")
        .author("Aswin")
        .about("A fun and challenging quiz game")
        .arg(
            Arg::with_name("quiz_file")
                .short('f')
                .long("file")
                .value_name("FILE")
                .help("Path to the quiz data file")
                .required(true),
        )
        .arg(
            Arg::with_name("difficulty")
                .short('d')
                .long("difficulty")
                .value_name("LEVEL")
                .help("Set quiz difficulty level (easy, medium, hard)")
                .default_value("medium"),
        )
        .arg(
            Arg::with_name("number_of_questions")
                .short('n')
                .long("number")
                .value_name("COUNT")
                .help("Number of questions to ask")
                .default_value("3"),
        )
        .get_matches();

    // Access the parsed arguments
    let quiz_file = matches.value_of("quiz_file").unwrap();
    let difficulty = matches.value_of("difficulty").unwrap();
    let number_of_questions: u32 = matches
        .value_of("number_of_questions")
        .unwrap()
        .parse()
        .unwrap();
	let questions = read_quiz_data(PathBuf::from(quiz_file)).unwrap(); // Load questions
	
let selected_questions = select_random_questions(&questions, difficulty, number_of_questions as usize); // Filter and randomize

let score = conduct_quiz(selected_questions.clone());// Conduct the quiz
println!("Your final score is: {}", score); // Display the final score
		
		
		
		
// Function to read quiz data from a JSON file
fn read_quiz_data(file_path: PathBuf) -> Result<Vec<Question>, Box<dyn Error>> {
	
	 let file_contents = fs::read_to_string(file_path)
        .map_err(|io_error| Box::new(io_error) as Box<dyn Error>)?;
    let data: Vec<Question> = serde_json::from_str(&file_contents)?;
    Ok(data)
   
}

// Structure to represent a quiz question
#[derive(Clone)]
#[derive(serde::Deserialize)]
struct Question {
    difficulty: String,
    question: String,
    answers: Vec<String>,
    correct_answer: usize,
}

// Function to select random questions based on difficulty and count
fn select_random_questions(
    questions: &Vec<Question>,
    difficulty: &str,
    count: usize,
) -> Vec<Question> {
    // Filter questions based on difficulty, ignoring case
    let mut filtered_questions: Vec<Question> = questions
        .iter()
        .filter(|q| q.difficulty.to_lowercase() == difficulty.to_lowercase()) // Case-insensitive comparison
        .cloned()
        .collect();

	


    // Shuffle the filtered questions
    filtered_questions.shuffle(&mut rand::thread_rng());

    // Select and return the desired number of questions
    filtered_questions.into_iter().take(count).collect() // Limit to count
}

// Function to present a question and handle user input
fn ask_question(question: &Question) -> bool {
    println!("Question: {}", question.question);
    for (index, answer) in question.answers.iter().enumerate() {
        println!("{}. {}", index + 1, answer);
    }

    let mut user_answer = String::new();
    io::stdin()
        .read_line(&mut user_answer)
        .expect("Failed to read user input");

    let user_answer_index: usize = match user_answer.trim().parse::<usize>() {
        Ok(num) => num - 1,
        Err(_) => {
            println!("Invalid answer format. Please enter a number.");
            return false; // Indicate incorrect answer due to invalid input
        }
    };

    question.correct_answer == user_answer_index
}



fn conduct_quiz(questions: Vec<Question>) -> usize {
    let mut score = 0;
    // Iterate through each question in the selected questions
    for question in questions.iter() {  // Use .iter() to iterate by reference
        if ask_question(question) {
            score += 1;
            println!("Correct!");
        } else {
            println!("Incorrect. The correct answer was {}.", question.answers[question.correct_answer]);
        }
    }
    score
}
}


