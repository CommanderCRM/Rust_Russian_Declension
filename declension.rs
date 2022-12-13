fn main() {
    let mut user_input = String::new();
    println!("Введите ФИО, потом нажмите Enter: ");
    std::io::stdin()
        .read_line(&mut user_input)
        .expect("Некорректный ввод");
    let mut words = user_input.split_whitespace();
    if words.clone().count() != 3 {
        panic!("Введите ровно 3 слова: Фамилия Имя Отчество");
    }
    let mut male = 0;
    let mut female = 0;
    let surname = words.next().unwrap();
    let name = words.next().unwrap();
    let patronymic = words.next().unwrap();
    let male_names = vec!["Артем", "Александр", "Максим", "Дмитрий", "Матвей", "Артём"];
    let female_names = vec!["Анна", "София", "Виктория", "Дарья", "Анастасия"];
    if male_names.contains(&name) {
        male += 1;
    } else if female_names.contains(&name) {
        female += 1;
    }
    if !male_names.contains(&name) && !female_names.contains(&name) {
        panic!("Имя не в списке топ-5");
    } else {
        let new_sur = modify_surname(surname, male, female);
        let new_name = modify_name(name);
        let new_patr = modify_patronymic(patronymic, male, female);
        println!("{} {} {}", new_sur, new_name, new_patr)
    }
}

fn modify_name(mut name: &str) -> &str{
        match name {
            "Артем" => name = "Артема",
            "Артём" => name = "Артёма",
            "Александр" => name = "Александра",
            "Максим" => name = "Максима",
            "Дмитрий" => name = "Дмитрия",
            "Матвей" => name = "Матвея",
            "Анна" => name = "Анны",
            "София" => name = "Софии",
            "Виктория" => name = "Виктории",
            "Дарья" => name = "Дарьи",
            "Анастасия" => name = "Анастасии",
            _ => println!("Нужное имя не получено"),
        }
        return name;
}

fn modify_patronymic(patronymic: &str, male: u8, female: u8)-> String{
        let patronymic_woman = ["вна"];
        let patronymic_man = ["ич"];
        let mut new_patronymic = String::from(patronymic);
        if male == 1 {
          let last_two = &new_patronymic[new_patronymic.len()-4..];
          let last_two_str: &&str = &&last_two[..];
          if patronymic_man.contains(last_two_str) {
                new_patronymic.push('а');
          }
        }
        else if female == 1 {
          let last_three = &new_patronymic[new_patronymic.len()-6..];
          let last_three_str: &&str = &&last_three[..];
          if patronymic_woman.contains(last_three_str) {
            new_patronymic.pop();
            new_patronymic.push('ы');
          }
        }
        return new_patronymic;
}

fn modify_surname(surname: &str, male: u8, female: u8) -> String {
        let good_woman_surname_endings = ["ина", "ова", "ёва", "ева"];
        let bad_woman_surname_endings = ["ых", "их", "ко"];
        let bad_woman_surname_endings_2 = ['о', 'е', 'и', 'у', 'ю'];
        let bad_woman_surname_endings_3 = [
            'б', 'в', 'г', 'д', 'ж', 'з', 'й', 'к', 'л', 'м', 'н', 'п', 'р', 'с', 'т', 'ф', 'х',
            'ц', 'ч', 'ш', 'щ',
        ];
        let good_man_surname_endings = ["ов", "ин", "ев"];
        let good_man_surname_endings_2 = [
            'б', 'в', 'г', 'д', 'ж', 'з', 'к', 'л', 'м', 'н', 'п', 'р', 'с', 'т', 'ф', 'х',
            'ц', 'ч', 'ш', 'щ',
        ];
        let good_man_surname_endings_3 = ["ой"];
        let bad_man_surname_endings = ["ых", "их", "ко"];
        let bad_man_surname_endings_2 = ['о', 'е', 'и', 'у', 'ю'];
        let good_man_surname_endings_4 = ["ий"];
        let mut new_surname = String::from(surname);
        if male == 1 {
            let last_one: char = surname.chars().last().unwrap();
            let last_two = &new_surname[new_surname.len()-4..];
            let last_two_str: &&str = &&last_two[..];
            if good_man_surname_endings.contains(last_two_str) {
                new_surname.push('а');
            } else if bad_man_surname_endings.contains(last_two_str) {
                new_surname = surname.to_string();
            } else if good_man_surname_endings_3.contains(last_two_str) {
                new_surname.pop();
                new_surname.push('г');
                new_surname.push('о');
            } else if good_man_surname_endings_2.contains(&last_one) {
                new_surname.push('а');
            } else if bad_man_surname_endings_2.contains(&last_one) {
                new_surname = surname.to_string();
            } else if good_man_surname_endings_4.contains(last_two_str) {
                new_surname.pop();
                new_surname.pop();
                new_surname.push('о');
                new_surname.push('г');
                new_surname.push('о');
            }
        } else if female == 1 {
            let last_three = &new_surname[new_surname.len()-6..];
            let last_two = &new_surname[new_surname.len()-4..];
            let last_one: char = surname.chars().last().unwrap();
            let last_two_str: &&str = &&last_two[..];
            let last_three_str: &&str = &&last_three[..];
            if good_woman_surname_endings.contains(last_three_str) {
                new_surname.pop();
                new_surname.push('о');
                new_surname.push('й');
            } else if bad_woman_surname_endings.contains(last_two_str) {
                new_surname = surname.to_string();
            } else if bad_woman_surname_endings_2.contains(&last_one)
                || bad_woman_surname_endings_3.contains(&last_one)
            {
                new_surname = surname.to_string();
            }
        }
        return new_surname;
}

#[test]
fn test1_modify_name()
{
  let s = "Артем";
  assert_eq!(modify_name(s), "Артема");
}

#[test]
fn test2_modify_name()
{
  let s = "Александр";
  assert_eq!(modify_name(s), "Александра");
}

#[test]
fn test3_modify_name()
{
  let s = "Анастасия";
  assert_eq!(modify_name(s), "Анастасии");
}

#[test]
fn test4_modify_name()
{
  let s = "Анна";
  assert_eq!(modify_name(s), "Анны");
}

#[test]
fn test5_modify_name()
{
  let s = "Дмитрий";
  assert_eq!(modify_name(s), "Дмитрия");
}

#[test]
fn test6_modify_surname()
{
  let s = String::from("Кривошеин");
  assert_eq!(modify_surname(&s,1,0), "Кривошеина");
}

#[test]
fn test7_modify_surname()
{
  let s = String::from("Гриценко");
  assert_eq!(modify_surname(&s,1,0), "Гриценко");
}

#[test]
fn test8_modify_surname()
{
  let s = String::from("Мухачева");
  assert_eq!(modify_surname(&s,0,1), "Мухачевой");
}

#[test]
fn test9_modify_surname()
{
  let s = String::from("Байгулова");
  assert_eq!(modify_surname(&s,0,1), "Байгуловой");
}

#[test]
fn test10_modify_surname()
{
  let s = String::from("Федосеев");
  assert_eq!(modify_surname(&s,1,0), "Федосеева");
}

#[test]
fn test11_modify_patronymic()
{
  let s = String::from("Игоревна");
  assert_eq!(modify_patronymic(&s,0,1), "Игоревны");
}

#[test]
fn test12_modify_patronymic()
{
  let s = String::from("Игоревич");
  assert_eq!(modify_patronymic(&s,1,0), "Игоревича");
}

#[test]
fn test13_modify_patronymic()
{
  let s = String::from("Андреевич");
  assert_eq!(modify_patronymic(&s,1,0), "Андреевича");
}

#[test]
fn test14_modify_patronymic()
{
  let s = String::from("Алексеевич");
  assert_eq!(modify_patronymic(&s,1,0), "Алексеевича");
}

#[test]
fn test15_modify_patronymic()
{
  let s = String::from("Сергеевна");
  assert_eq!(modify_patronymic(&s,0,1), "Сергеевны");
}