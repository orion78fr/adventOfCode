extern crate pest;
#[macro_use]
extern crate pest_derive;

use std::collections::HashMap;
use std::str::FromStr;

use pest::iterators::Pair;
use pest::iterators::Pairs;
use pest::Parser;

const PUZZLE: &str = include_str!("../input");

#[derive(Parser)]
#[grammar = "guard_schedule.pest"]
pub struct ScheduleParser;

#[derive(Debug)]
struct Event {
    date: Date,
    action: Action,
}

#[derive(Debug)]
struct Date {
    year: i16,
    month: i8,
    day: i8,
    hour: i8,
    minute: i8,
}

#[derive(Debug)]
enum Action {
    Sleep,
    Wake,
    StartShift(i32),
}

fn main() {
    let mut events: Vec<_> = PUZZLE.lines()
        .map(|sched| {
            ScheduleParser::parse(Rule::schedule, sched).unwrap().next().unwrap()
        })
        .map(parse_schedule)
        .collect();

    // Sort by date
    events.sort_unstable_by(|a, b| {
        a.date.year.cmp(&b.date.year)
            .then(a.date.month.cmp(&b.date.month))
            .then(a.date.day.cmp(&b.date.day))
            .then(a.date.hour.cmp(&b.date.hour))
            .then(a.date.minute.cmp(&b.date.minute))
    });

    let mut current_guard = -1;
    let mut start_minute = -1;
    let mut guard_schedules = HashMap::new();

    events.iter()
        .for_each(|e| {
            match e.action {
                Action::Sleep => start_minute = e.date.minute,
                Action::Wake => {
                    let schedule = guard_schedules.entry(current_guard)
                        .or_insert(HashMap::new());
                    for i in start_minute..e.date.minute {
                        *schedule.entry(i).or_insert(0) += 1
                    }
                }
                Action::StartShift(id) => current_guard = id
            }
        });

    let (id, _, schedule) = guard_schedules.iter()
        .map(|(id, m)| (id, m.values().sum::<i32>(), m))
        .max_by(|(_, cnt, _), (_, cnt2, _)| cnt.cmp(cnt2))
        .unwrap();

    let (most_sleeped_minute, _) = schedule.iter()
        .max_by(|(_, v), (_, v2)| v.cmp(v2))
        .unwrap();

    println!("{}", *id as i64 * *most_sleeped_minute as i64);
}

fn parse_schedule(schedule: Pair<Rule>) -> Event {
    let mut inner = schedule.into_inner();

    let date = parse_date(inner.next().unwrap());
    let action = parse_action(inner.next().unwrap());

    Event { date, action }
}

fn parse_date(date: Pair<Rule>) -> Date {
    let date = &mut date.into_inner();

    Date {
        year: next_num(date),
        month: next_num(date),
        day: next_num(date),
        hour: next_num(date),
        minute: next_num(date),
    }
}

fn parse_action(action: Pair<Rule>) -> Action {
    let action = action.into_inner().next().unwrap();
    match action.as_rule() {
        Rule::shift => Action::StartShift(next_num(&mut action.into_inner())),
        Rule::sleep => Action::Sleep,
        Rule::wakes => Action::Wake,
        _ => panic!()
    }
}

fn next_str<'a>(iterator: &'a mut Pairs<Rule>) -> &'a str {
    iterator.next().unwrap().as_str()
}

fn next_num<T: FromStr>(iterator: &mut Pairs<Rule>) -> T {
    next_str(iterator).parse().ok().unwrap()
}