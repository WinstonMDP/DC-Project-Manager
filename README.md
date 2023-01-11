# Типо ТЗ, но не совсем

## Введение

Action DDB - это документоориентированная база данных 
с простым интерфейсом, actions и набором обязательных полей.
Action DDB стала нужна для удобной настройки project/task manager КпЦ.

Задачи (task) - это те же документы, но с полем статуса
выполнения, которое может быть абсолютно разным в зависимости
от настроек системы, плюс функционал project/task manager может часто меняться
от проекта к проекту, поэтому лучше обобщить project/task manager до такой
базы данных.

При этом в Action DDB никаких task-ов не используется.
Project/task manager был упомянут лишь для указания мотивации\актуальности
создания данного продукта (Action DDB).

Чтобы понять техническую часть, нужно знать, что такое документоориентированная база данных
и из каких частей она состоит.

## Техническая часть

Action DDB - это CLI программа.

### Вход/регистрация пользователя

sign up \<user-name> \<user-email> \<user-password>

sign in \<user-name>|\<user-email> \<user-password>

sign out

add two-factor authentication ...

### Collection

collection in \<collection-id>

collection out

create collection

add collection description \<collection-description>

add collection action \<collection-action>

#### Структура collection

id: Integer

description: String

action: Action

### Document

documents выводит все document-ы в сокращённой форме.

document \<document-id>|random

create document

add document field \<document-field>

delete document field \<document-field>

edit document field value \<document-field> \<value>

#### Структура document

id: Integer

user: User

...

### Action

Actions - это что-то вроде GitHub Actions.

Actions должны предоставить возможность пользователям, например: ввести систему рейтинга,
чтобы за каждую выполненную задачу человеку что-то начислялось,
а за плохие действия списывалось; ввести ограничение на время выполнения задач;
ввести последовательность в выполнении задач, позволив образовывать
цепочки задач, то есть обязать выполнить, например, первую задачу перед второй;
встроить какую-ту автоматическую проверку задач; или встроить автоматическое
наполнение описания задач чем-то и т.д.
