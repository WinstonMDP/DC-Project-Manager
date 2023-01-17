# Краткое описание (будет дополняться)

## Введение

Action DDB - это документоориентированная база данных
с простым интерфейсом и автоматизацией "Actions".
Action DDB стала нужна для удобной настройки project/task manager КпЦ.

Задачи (task) - это те же документы (documents), но с полем статуса
выполнения, которое может быть абсолютно разным (а может и отсутствовать вовсе) в зависимости
от настроек системы. К тому же, функционал project/task manager-а может часто 
меняться от проекта к проекту, поэтому было решено обобщить project/task manager до 
документоориентированной базы данных.

При этом в Action DDB никаких task-ов не используется (это лишь частный случай document).
Project/task manager был упомянут только для указания мотивации создания данного продукта (Action DDB).

### Action

Actions - это нечто вроде GitHub Actions.

Actions должны предоставить возможность пользователям, например: 
- ввести систему рейтинга,
чтобы за каждую выполненную задачу человеку начислялось какое-то количество "баллов".
- ввести ограничение на время выполнения задач.
- Образовывать цепочки задач, то есть сделать невозможным выполнение второй задачи перед первой
встроить какую-ту автоматическую проверку задач или встроить автоматическое
наполнение описания задач.
И многое другое (про структуру Action подробнее см. в технической части).

## Техническая часть

Action DDB - это CLI (Command Line Interface) программа.

### Вход/регистрация пользователя

sign up \<user-name> \<user-email> \<user-password>

sign in \<user-name>|\<user-email> \<user-password>

sign out

add two-factor authentication (интерфейс ещё рассматривается)

### Collection

collection in \<collection-id>

collection out

create collection

edit collection description \<collection-description-file>

add collection action \<collection-action-file>

remove collection action \<collection-action-file>

#### Структура collection

id: Integer

description: description.txt

actions: \[*.action]

documents: \[document]

### Document

documents выводит все document-ы в сокращённой форме.

document \<document-id>|random

create document

add document field \<document-field>

remove document field \<document-field>

edit document field value \<document-field> \<value>

#### Структура document

document представляется в формате json

id: Integer

Остальные по решению пользователя.

### Action

filename.action

```yml
on <regex-of-commands>:
    do:
        <first-step>
        <second-step>
        ...
```

Action знает, кто запустил команду.
