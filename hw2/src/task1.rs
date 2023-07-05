/*
----> ЗАДАНИЕ 1 "Поиск слова в строке"

Вывести номер строки в котором встречается нужное слово и саму строку в формате:
номер строки: строка...

 */

const SEARCH_TERM: &str = "picture";
 const QUOTE: &str = "Every face, every shop, bedroom window, public-house, and
 dark square is a picture feverishly turned--in search of what?
 It is the same with books. What do we seek through millions of pages?";


 fn main() {
     find_term(SEARCH_TERM, QUOTE);
 }

 fn find_term(search_term: &str, quote: &str) -> String {
    quote
        .split("\n")
        .enumerate()
        .filter(|s| s.1.contains(search_term))
        .map(|s| {(s.0 + 1).to_string() + ":" + s.1})
        .collect()
 }