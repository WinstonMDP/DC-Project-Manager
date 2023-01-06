# Типо ТЗ, но не совсем

Все данные о текущей сессии содержаться в файле.

Если чего-то нет по id,
предложить по name.

### Вход/регистрация пользователя

sign up \<user-name> \<user-email> \<user-password>

sign in \<user-name>|\<user-email> \<user-password>

sign out

### Выбор проекта

project in \<project-id>|\<project-name>

project out

create project \<project-name>

add project description \<project-description>

add project rights \<project-id>|\<project-name>|nothing \<project-rights>

project history \<project-id>|\<project-name>|nothing

project history выводит историю текущего проекта.

## Структура проекта

id: Integer

### Задачи по проекту

show tasks выводит все available задачи.

show task \<task-id>|\<task-name>|random

create task \<task-name>

add task description \<task-id>|\<task-name> \<task-description>

add task action \<task-id>|\<task-name> \<task-action>

select \<task-id>|\<task-name> статус с unselected в selected.

deselect статус с selected в unselected.

achieve \<task-id>|\<task-name> статус с selected в achieved.

recover \<task-id>|\<task-name> статус с achieved в unselected.

task history \<task-id>|\<task-name>

## Структура задачи

id: Integer

action (что-то типо action в Git Actions): Action = nothing

## Action

Action имеет информацию о пользователе, выполнившем команду.

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
