# Типо ТЗ, но не совсем

Все данные о текущей сессии содержаться в файле.

Если чего-то нет по id,
предложить по name.

## Вход/регистрация пользователя

sign up \<user-name> \<user-email> \<user-password>

sign in \<user-name>|\<user-email> \<user-password>

sign out

add two-factor authentication ...

## Collection

collection in \<collection-id>|\<collection-name>

collection out

create collection \<collection-name>

add collection description \<collection-name>

add collection action \<collection-action>

### Структура collection

id: Integer

description: String

action: Action

## Document

show documents выводит все document-ы в сокращённой форме.

show document \<document-id>|\<document-name>|random

create document \<document-name>

add document field \<document-field>

delete document field \<document-field>

edit document field value \<document-field> \<value>

### Структура document

id: Integer

user: User

...

## Мотивация

Эта документоориентированная база данных с простым интерфейсом и набором
обязательных полей нужна для удобной настройки project/task manager КпЦ.
Задачи - это те же документы, но с полем статуса
выполнения, которое может быть абсолютно разным в зависимости
от настроек системы, плюс функционал project/task manager может часто меняться, поэтому лучше обобщить project/task manager до такой базы данных.

### Action

Actions должны предоставить возможность пользователям, например: ввести систему рейтинга,
чтобы за каждую выполненную задачу человеку что-то начислялось,
а за плохие действия списывалось; ввести ограничение на время выполнения задач;
ввести последовательность в выполнении задач, позволив образовывать
цепочки задач, то есть обязать выполнить, например, первую задачу перед второй;
встроить какую-ту автоматическую проверку задач; или встроить автоматическое
наполнение описания задач чем-то и т.д.

### Совет пользователям

Время, рассчитанное на задачу, должно быть не больше суток.

### Возможная интеграция с Git

Имя коммита: "document.id: document.name"

В git actions настроена проверка на то,
что такая задача есть и её коммита ещё не было.
