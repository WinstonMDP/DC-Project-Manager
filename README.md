# Типо ТЗ, но не совсем

Все данные о текущей сессии содержаться в файле.

## Консоль

Если чего-то нет по id,
предложить по name.

### Вход/регистрация пользователя

sign up \<user-name> \<user-email> \<user-password>

sign in \<user-name>|\<user-email> \<user-password>

sign out

### Выбор проекта

create project \<project-name>

add project description \<project-description>

project in \<project-id>|\<project-name>

project out

add project rights \<project-id>|\<project-name>|nothing \<project-rights>

project history \<project-id>|\<project-name>|nothing

project history nothing выводит историю текущего проекта.

## Структура проекта

id: Integer

name: String

description: String = ""

rights: Rights

history: History

### Задачи по проекту

show tasks выводит все available задачи.

show task \<task-id>|\<task-name>|random

create task \<task-name>

add task description \<task-id>|\<task-name> \<task-description>

add task action \<task-id>|\<task-name> \<task-action>

activate \<task-id>|\<task-name> статус с unavailable в available.

deactivate \<task-id>|\<task-name> статус с available в unavailable.

select \<task-id>|\<task-name> статус с available в selected.

deselect статус с selected в available.

achieve \<task-id>|\<task-name> статус с selected в achieved.

recover \<task-id>|\<task-name> статус с achieved в unavailable.

task history \<task-id>|\<task-name>

## Структура задачи

id: Integer

status: {unavailable, available, selected, achieved} = unavailable

selectedUser: User | nothing

name: String

description: String = ""

action (что-то типо action в Git Actions): Action = nothing

history: History

### Если action реализовать не получится, то к структуре добавить

selectDate: Date | nothing

taskTime: Time

## Action

Actions должны предоставить возможность пользователям, например: ввести систему рейтинга,
чтобы за каждую выполненную задачу человеку что-то начислялось,
а за плохие действия списывалось; ввести ограничение на время выполнения задач;
ввести последовательность в выполнении задач, позволив образовывать
цепочки задач, то есть обязать выполнить, например, первую задачу перед второй;
встроить какую-ту автоматическую проверку задач; или встроить автоматическое
наполнение описания задач чем-то и т.д.

## Совет пользователям

Время, рассчитанное на задачу, должно быть не больше суток.

## Возможная интеграция с Git

Имя коммита: "task.id: task.name"

В git actions настроена проверка на то,
что такая задача есть и её коммита ещё не было.
