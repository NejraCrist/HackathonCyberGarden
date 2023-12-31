#### Описание
Проект предназначен для изучения материала по методу карточек

#### Структура проекта:
1. conception
   Основные структуры для карточек
2. claster_creator
   Консольная программа для создание кластеров карточек
3. telegram_bot
   Приложение для запуска телеграмм бота (пример)

#### Запуск
Для запуска вам нужно установить rust:
`https://www.rust-lang.org/tools/install`

После войдите в корневую директорию проекта и запустите компиляцию телеграмм бота:
```bash
cargo run -r -p telegram_bot -- --help
```

Скомпилированная версия будет в папке target/release/ и её можно использовать без компиляции как cli утилиту

#### Запуск бота

Для запуска бота вам нужно указать переменную окружение TELOXIDE_TOKEN или аргумент --tg-token при запуске

Так-же, название базы данных (--db-name) и путь к папке с каталогами карточек (--claster-path)

```bash
./telegram_bot --db-name data.sqlite --claster-path "./flashcards" --tg-token 1234:ABCDEFGH
```