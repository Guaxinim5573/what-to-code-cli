use colored::*;
use serde::{Deserialize};

const API_URL: &str = "https://what-to-code.com/api/ideas/random";

#[derive(Deserialize)]
struct Tag {
    value: String
}

#[derive(Deserialize)]
struct Idea {
    title: String,
    description: String,
    tags: Vec<Tag>,
    // id: u64 // Not in use
    likes: u64
}

fn main() -> Result<(), minreq::Error> {
    let response = minreq::get(API_URL)
        .send()?;
    let idea: Idea = response.json()?;
    println!("    {}", idea.title.bold());
    if idea.description.len() > 0 {
        println!("{}", idea.description);
    }
    println!("{} {}", "Likes:".red(), idea.likes);
    if idea.tags.len() > 0 {
        print!("{} ", "Tags:".green());
        for (i, tag) in idea.tags.iter().enumerate() {
            if i > 0 {
                print!(", {}", tag.value);
            } else {
                print!("{}", tag.value);
            }
        }
        println!("");
    }
    Ok(())
}