use std::ffi::OsStr;
use std::io::Write;
use std::path::PathBuf;
use std::time::Instant;

// Число для получения размера файлов в МБ (1*10^6)
const FILE_SIZE_BASE: f64 = 1e6;

// Вспомогательная функция get_input, чтобы получать ввод от пользователя
fn get_input(query: &str) -> std::io::Result<String> {
    print!("{}", query);
    std::io::stdout().flush()?;

    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer)?;

    Ok(buffer.trim().to_owned())
}

// Функция, возвращающая данные, необходимые для поиска файла
fn get_search_data() -> Option<(String, String, Vec<String>)> {
    // Запрос пути, по которому будет осуществляться поиск файла
    let search_path = match get_input("Введите путь, по которому будет осуществлен поиск файла: ") {
        Ok(path) => path,
        Err(_) => return None
    };
    // Запрос имени файла или части имени файла, которого требуется найти (без расширения)
    let search_name = match get_input("Введите имя файла (без расширения): ") {
        Ok(path) => path,
        Err(_) => return None
    };
    // Запрос расширения файла
    let search_extensions = match get_input("Введите расширение(-я) (разделять пробелом!) ") {
        Ok(extensions_string) => get_extensions(extensions_string),
        Err(_) => return None
    };
    // В случае пустого ввода:
    if search_path.is_empty() || (search_name.is_empty() && search_extensions.is_empty()) {
        println!("Введите что-либо!");
        return None;
    }

    Some((search_path.to_lowercase(), search_name.to_lowercase(), search_extensions))
}

// Функция, непосредственно выполняющая поиск файлов, считывающая время и кол-во совпадений
fn search_files(search_path: &str, filename: &str, extensions: &Vec<String>,
                now: &Instant, results_count: &mut i32) {
    // Обозначения неуказанных расширений или имен файлов
    let is_no_extensions = extensions.is_empty();
    let is_empty_filename = filename.is_empty();

    // Чтение файлов, введенных пользователем
    let files = match std::fs::read_dir(search_path) {
        Ok(files) => files,
        Err(_) => return
    };

    // Проход по прочтенным файлам
    for entry in files {
        if let Ok(entry) = entry {
            let path = entry.path();
            let file_name = convert_os_str(path.file_stem());
            let file_extension = convert_os_str(path.extension());

            // Проверка на папку (если найденное программой - не файл, а папка)
            if path.is_dir() {
                if is_no_extensions && file_name.contains(filename) {
                    file_found(&path, now, results_count);
                }

                search_files(
                    path.to_str().unwrap_or_default(),
                    filename,
                    extensions,
                    now,
                    results_count
                );
            } else if is_empty_filename && extensions.contains(&file_extension) {
                file_found(&path, now, results_count);
            } else if path.is_file() && file_name.contains(filename) {
                if (!is_no_extensions && extensions.contains(&file_extension)) || is_no_extensions {
                    file_found(&path, now, results_count);
                }
            }
        }
    }
}

// Функция преобразования ОС строки в строку Раст
fn convert_os_str(os_str: Option<&OsStr>) -> String {
    os_str
        .unwrap_or_default()
        .to_str()
        .unwrap_or_default()
        .to_lowercase()
}

// Функция, разделяющая и возвращающая расширения файлов
fn get_extensions(extensions_string: String) -> Vec<String> {
    extensions_string.split_whitespace().map(|ext| ext.to_lowercase()).collect()
}

// Функция найденного файла
fn file_found(path: &PathBuf, now: &Instant, results_count: &mut i32) {
    *results_count += 1;
    print_path_info(path, now);
}

// Функция вывода времени поиска файла и его размера
fn print_path_info(path: &PathBuf, now: &Instant) {
    print!(
        "{} - Найден за {} секунд(-ы)",
        path.display(),
        now.elapsed().as_secs_f64()
    );

    match std::fs::metadata(path) {
        Ok(metadata) => {
            print!(" - {} МБ\n", metadata.len() as f64 / FILE_SIZE_BASE);
        }
        Err(_) => println!()
    }
}

//Главная функция через луп
fn main() {
    loop {
        let (path, filename, extensions) = match get_search_data() {
            None => continue,
            Some(data) => data
        };

        println!();

        let now = Instant::now();
        let mut results_count = 0;

        search_files(
            path.as_str(),
            filename.as_str(),
            &extensions,
            &now,
            &mut results_count
        );

        println!(
            "\nОбщее время поиска: {} секунд(-ы)\n{} совпадений\n",
            now.elapsed().as_secs_f64(),
            results_count
        );
    }
}
