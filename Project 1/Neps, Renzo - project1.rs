/**
 * Renzo Neps, CS 3304 project1
 */

/*
	<sentence>    -->  <subject> <verb_phrase> <object>
	<subject>     -->  <noun_phrase>
	<verb_phrase> -->  <verb> | <verb> <adv>
	<object>      -->  <noun_phrase>
	<verb>        -->  lifted | saw | found
	<adv>         -->  quickly | carefully | brilliantly
	<noun_phrase> -->  [<adj_phrase>] <noun> [<prep_phrase>]
	<noun>        -->  cow | alice | book
	<adj_phrase>  -->  <adj> | <adj> <adj_phrase>
	<adj>         -->  green | lean | mean
	<prep_phrase> -->  <prep> <noun_phrase>
	<prep>        -->  of | at | with
*/

use std::env;
use std::fs::File;
use std::io::{BufReader, Error, BufRead, Write};

/**
 * Main fuction that runs the program
 */
fn main() -> Result<(), Error>
{
    let args: Vec<String> = env::args().collect();

    if args.len() != 3
    {
        println!("Usage: ./project1 <input> <output>");
    }
    else
    {
        // Read the file
        let input = &args[1];
        let output = &args[2];
        let path = File::open(&input)?;
        let reader = BufReader::new(path);
        let mut out = File::create(output).expect("unable to create file");
        for line in reader.lines()
        {
            let this_line = line.unwrap();
            let words: Vec<&str> = this_line.split(" ").collect();
            
            write!(&mut out, "input-line => ");
            for word in words.iter()
            {
                write!(&mut out, "{} ", word);
            }
            writeln!(&mut out, "");
            
            // Check for all tokens to be valid
            if ifNotValidTokens(words.clone())
            {
                writeln!(&mut out, "    Input has invalid tokens.")?;
            }
            // Check to see if the sentence is valid
            else if ifNotValidSentence(words.clone())
            {
                writeln!(&mut out, "    Input is not a sentence.")?;
            }
            else
            {
                let break1: i16 = findBreak(words.clone(), 0 as i8);

                let subject: Vec<&str> = nounPhrase(words.clone(), 0 as i16);
                let verb_phrase: Vec<&str> = verbPhrase(words.clone(), break1);
                let object: Vec<&str> = nounPhrase(words.clone(), (break1 + 1) as i16);

                write!(&mut out, "    (");
                for x in subject.iter()
                {
                    write!(&mut out, "{}", x);
                }
                write!(&mut out, " ");
                for x in verb_phrase.iter()
                {
                    write!(&mut out, "{}", x);
                }
                write!(&mut out, " ");
                for x in object.iter()
                {
                    write!(&mut out, "{}", x);
                }
                writeln!(&mut out, ")");
            }
        }
    }
    Ok(())
}

/**
 *Return true if found invalid tokends
 */
fn ifNotValidTokens(line: Vec<&str>) -> bool
{
    for word in line.iter()
    {
        if checkToken(word) == 5
        {
            return true;
        }
    }
    return false;
}

/**
 * Return false if verb is in the line
 */
fn ifNotValidSentence(line: Vec<&str>) -> bool
{
    let mut _result: bool = false;
    for word in line.iter()
    {
        if checkToken(word) == 0
        {
            return false;
        }
    }
    return true;
}

/**
 * Check the token called, returns:
 *      - 0 for verb
 *      - 1 for adverb
 *      - 2 for noun
 *      - 3 for adjective
 *      - 4 for preposition
 */
fn checkToken(word: &str) -> i8
{
    match word
    {
        "lifted" | "saw" | "found" => return 0,
        "quickly" | "carefully" | "brilliantly" => return 1,
        "cow" | "alice" | "book" => return 2,
        "green" | "lean" | "mean" => return 3,
        "of" | "at" | "with" => return 4,
        _ => return 5,
    }
}

/**
 * Find the first word based on the number given
 */
fn findBreak(line: Vec<&str>, num: i8) -> i16
{
    let mut i: i16 = 0;
    for word in line.iter()
    {
        if checkToken(word) == num
        {
            return i;
        }
        i = i + 1 as i16;
    }
    return 0 as i16;
}

/**
 * Add any missing specified parenthesis
 */
fn addParenthesis(line: &mut Vec<&str>, amount: i8)
{
    for _i in 0..amount
    {
        line.push(")");
    }
}

/**
 * Construct a noun phrase starting at a specified break point
 */
fn nounPhrase<'a>(line: Vec<&'a str>, break_: i16) -> Vec<&'a str>
{
    let mut nounPhrase = Vec::<&str>::new();
    nounPhrase.push("(");
    let mut adjInARow: i8 = 0;
    let mut missingParen: i8 = 1;
    let mut counter: i16 = 0;

    while break_+counter < line.len() as i16 && checkToken(line[(break_+counter) as usize]) != 0
    {
        if checkToken(line[(break_+counter) as usize]) == 3
        {
            if adjInARow == 0
            {
                nounPhrase.push("(");
                missingParen = missingParen + 1;
            }
            adjInARow = adjInARow + 1;
            nounPhrase.push("(");
            nounPhrase.push(line[(break_+counter) as usize]);
        }
        else if checkToken(line[(break_+counter) as usize]) == 2
        {
            addParenthesis(&mut nounPhrase, adjInARow);
            if adjInARow == 0
            {
                nounPhrase.push("(");
                missingParen = missingParen + 1;
            }
            adjInARow = 0;
            nounPhrase.push(line[(break_+counter) as usize]);
        }
        else if checkToken(line[(break_+counter) as usize]) == 4
        {
            nounPhrase.push("(");
            nounPhrase.push(line[(break_+counter) as usize]);
            missingParen = missingParen + 1;
        }
        counter = counter + 1 as i16;
    }
    addParenthesis(&mut nounPhrase, missingParen);

    return nounPhrase;
}

/**
 * Construct the verb phrase at a specified break point
 */
fn verbPhrase<'a>(line: Vec<&'a str>, break_: i16) -> Vec<&'a str>
{
    let mut vp = Vec::<&str>::new();
    vp.push("(");
    vp.push(line[break_ as usize]);

    if checkToken(line[(break_ + 1) as usize]) == 1
    {
        vp.push(" ");
        vp.push(line[(break_ + 1) as usize]);
    }

    vp.push(")");
    return vp;
}