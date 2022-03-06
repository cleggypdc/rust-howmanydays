use std::env::args;

enum NumDays {
    ThirtyDays,
    ThirtyOneDays,
    TwentyEight
}

#[derive(Debug)]
enum MonthName {
    January,
    February,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December
}

#[derive(Debug)]
struct InputError;

struct Month {
    name: MonthName,
    number: i32,
    number_of_days: NumDays,
}

impl Month {

    fn factory(s: &str) -> Result<Month, InputError> {
        match s.to_lowercase().as_str() {
            "january"|"jan"|"1" => Ok(Month {name:MonthName::January, number:1, number_of_days:NumDays::ThirtyOneDays}),
            "february"|"feb"|"2" => Ok(Month {name:MonthName::February, number:2, number_of_days:NumDays::TwentyEight}),
            "march"|"mar"|"3" => Ok(Month {name:MonthName::March, number:3, number_of_days:NumDays::ThirtyOneDays}),
            "april"|"apr"|"4" => Ok(Month {name:MonthName::April, number:4, number_of_days:NumDays::ThirtyDays}),
            "may"|"5" => Ok(Month {name:MonthName::May, number:5, number_of_days:NumDays::ThirtyOneDays}),
            "june"|"jun"|"6" => Ok(Month {name:MonthName::June, number:6, number_of_days:NumDays::ThirtyDays}),
            "july"|"jul"|"7" => Ok(Month {name:MonthName::July, number:7, number_of_days:NumDays::ThirtyOneDays}),
            "august"|"aug"|"8" => Ok(Month {name:MonthName::August, number:8, number_of_days:NumDays::ThirtyOneDays}),
            "september"|"sept"|"9" => Ok(Month {name:MonthName::September, number:9, number_of_days:NumDays::ThirtyDays}),
            "october"|"oct"|"10" => Ok(Month {name:MonthName::October, number:10, number_of_days:NumDays::ThirtyOneDays}),
            "november"|"nov"|"11" => Ok(Month {name:MonthName::November, number:11, number_of_days:NumDays::ThirtyDays}),
            "december"|"dec"|"12" => Ok(Month {name:MonthName::December, number:12, number_of_days:NumDays::ThirtyOneDays}),
            _ => Err(InputError)
        }
    }

    fn how_many_days(&self) -> i32 {
        match self.number_of_days {
            NumDays::ThirtyOneDays => 31,
            NumDays::ThirtyDays => 30,
            NumDays::TwentyEight => 28
        }
    }

    fn format_month_number(&self) -> String {
        match self.number {
            1 => String::from("1st"),
            2 => String::from("2nd"),
            3 => String::from("3rd"),
            _ => String::from(self.number.to_string() + "th")
        }
    }

    fn print_info(self) {
        println!("{:?}, the {} month of the year has {} days.", self.name, self.format_month_number(), self.how_many_days())
    }

}

fn main() {
    let args: Vec<String> = args().collect();

    let month = Month::factory(args[1].trim())
        .expect("Invalid input, enter the month name or number i.e (Jan/January/1)");

    month.print_info();
}
