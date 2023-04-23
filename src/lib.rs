#[derive(Debug, Clone)]
pub struct TriviaContext {
    /// Whether or not questions may repeat
    pub repeat_questions: bool,
    categories: Vec<CachedCategory>,
    spent_questions: Vec<Question>,
}

pub enum Seeding {
    Random,
    Specific(usize),
}

impl TriviaContext {
    pub fn new(repeat_questions: bool) -> Self {
        TriviaContext {
            repeat_questions,
            categories: index_categories(),
            spent_questions: vec![],
        }
    }

    /// Get a question of a specific category. If Specific seeding is larger than the amount of questions, a modulo will be applied
    pub fn get_question(&mut self, category: Category, seeding: Seeding) -> Question {
        use rand::prelude::*;

        let category = self
            .categories
            .iter()
            .find(|c| c.category == category)
            .unwrap();

        let idx = match seeding {
            Seeding::Random => thread_rng().gen_range(0..category.questions.len()),
            Seeding::Specific(s) => s,
        };

        if self.repeat_questions == true {
            return category
                .questions
                .get(idx % category.questions.len())
                .unwrap()
                .clone();
        } else {
            let mut question = category
                .questions
                .get(idx % category.questions.len())
                .unwrap()
                .clone();
            while self.spent_questions.contains(&question) {
                question = category
                    .questions
                    .get(idx % category.questions.len())
                    .unwrap()
                    .clone();
            }
            self.spent_questions.push(question.clone());
            return question;
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Category {
    Animals,
    BrainTeasers,
    Celebrities,
    Entertainment,
    ForKids,
    General,
    Geography,
    History,
    Hobbies,
    Humanities,
    Literature,
    Movies,
    Music,
    People,
    ReligionFaith,
    ScienceTechnology,
    Sports,
    Television,
    VideoGames,
    World,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Question {
    pub category: Category,
    pub question: String,
    pub answers: Vec<String>,
    pub correct_answer: String,
}

impl Question {
    fn new(category: Category) -> Self {
        Question {
            answers: vec![],
            category,
            correct_answer: String::new(),
            question: String::new(),
        }
    }
}

#[derive(Debug, Clone)]
struct CachedCategory {
    category: Category,
    questions: Vec<Question>,
}

fn index_categories() -> Vec<CachedCategory> {
    let animals = include_str!("./resources/animals.txt");
    let brain_teasers = include_str!("./resources/brain-teasers.txt");
    let celebrities = include_str!("./resources/celebrities.txt");
    let entertainment = include_str!("./resources/entertainment.txt");
    let for_kids = include_str!("./resources/for-kids.txt");
    let general = include_str!("./resources/general.txt");
    let geography = include_str!("./resources/geography.txt");
    let history = include_str!("./resources/history.txt");
    let hobbies = include_str!("./resources/hobbies.txt");
    let humanities = include_str!("./resources/humanities.txt");
    let literature = include_str!("./resources/literature.txt");
    let movies = include_str!("./resources/movies.txt");
    let music = include_str!("./resources/music.txt");
    let people = include_str!("./resources/people.txt");
    let religion_faith = include_str!("./resources/religion-faith.txt");
    let science_technology = include_str!("./resources/science-technology.txt");
    let sports = include_str!("./resources/sports.txt");
    let television = include_str!("./resources/television.txt");
    let video_games = include_str!("./resources/video-games.txt");
    let world = include_str!("./resources/world.txt");

    vec![
        CachedCategory {
            category: Category::Animals,
            questions: get_questions(animals, Category::Animals),
        },
        CachedCategory {
            category: Category::BrainTeasers,
            questions: get_questions(brain_teasers, Category::BrainTeasers),
        },
        CachedCategory {
            category: Category::Celebrities,
            questions: get_questions(celebrities, Category::Celebrities),
        },
        CachedCategory {
            category: Category::Entertainment,
            questions: get_questions(entertainment, Category::Entertainment),
        },
        CachedCategory {
            category: Category::ForKids,
            questions: get_questions(for_kids, Category::ForKids),
        },
        CachedCategory {
            category: Category::General,
            questions: get_questions(general, Category::General),
        },
        CachedCategory {
            category: Category::Geography,
            questions: get_questions(geography, Category::Geography),
        },
        CachedCategory {
            category: Category::History,
            questions: get_questions(history, Category::History),
        },
        CachedCategory {
            category: Category::Hobbies,
            questions: get_questions(hobbies, Category::Hobbies),
        },
        CachedCategory {
            category: Category::Humanities,
            questions: get_questions(humanities, Category::Humanities),
        },
        CachedCategory {
            category: Category::Literature,
            questions: get_questions(literature, Category::Literature),
        },
        CachedCategory {
            category: Category::Movies,
            questions: get_questions(movies, Category::Movies),
        },
        CachedCategory {
            category: Category::Music,
            questions: get_questions(music, Category::Music),
        },
        CachedCategory {
            category: Category::People,
            questions: get_questions(people, Category::People),
        },
        CachedCategory {
            category: Category::ReligionFaith,
            questions: get_questions(religion_faith, Category::ReligionFaith),
        },
        CachedCategory {
            category: Category::ScienceTechnology,
            questions: get_questions(science_technology, Category::ScienceTechnology),
        },
        CachedCategory {
            category: Category::Sports,
            questions: get_questions(sports, Category::Sports),
        },
        CachedCategory {
            category: Category::Television,
            questions: get_questions(television, Category::Television),
        },
        CachedCategory {
            category: Category::VideoGames,
            questions: get_questions(video_games, Category::VideoGames),
        },
        CachedCategory {
            category: Category::World,
            questions: get_questions(world, Category::World),
        },
    ]
}

fn get_questions(text: &str, category: Category) -> Vec<Question> {
    let mut questions = vec![];
    let mut cur_question = Question::new(category);

    for line in text.lines() {
        let first = match line.chars().nth(0) {
            Some(c) => c,
            None => continue,
        };

        match first {
            '#' => {
                questions.push(cur_question);
                cur_question = Question::new(category);
                cur_question.question = line[3..].to_string();
            }
            '^' => {
                cur_question.correct_answer = line[2..].to_string();
            }
            // Don't think it actually past D, but might as well be redundant
            'A' | 'B' | 'C' | 'D' | 'E' | 'F' => cur_question.answers.push(line[2..].to_string()),
            _ => continue,
        }
    }

    questions
}
