# Table of Contents
- [Table of Contents](#table-of-contents)
- [General](#general)
- [User](#user)
  - [Create task](#create-task)
  - [Read all user](#read-all-user)
  - [Update user](#update-user)
  - [Delete user](#delete-user)
  - [Find user](#find-user)
  - [Search user](#search-user)
  - [Read user task](#read-user-task)
- [Task](#task)
  - [Create task](#create-task-1)
  - [Read all task](#read-all-task)
  - [Update task](#update-task)
  - [Delete task](#delete-task)
  - [Find task](#find-task)
  - [Search Task](#search-task)
  - [Assign Task](#assign-task)
  - [Read task owner](#read-task-owner)
- [Addtional Idea: Role (enum)](#addtional-idea-role-enum)
- [Object](#object)
  - [User Object](#user-object)
  - [NewUser Object](#newuser-object)
  - [UpdateUser Object](#updateuser-object)
  - [Task Object](#task-object)
  - [NewTask Object](#newtask-object)
  - [UpdateTask Object](#updatetask-object)
  - [AssignTask Object](#assigntask-object)
  - [Timestamp](#timestamp)
  - [ID](#id)

# General
- Post method only accept json content type
- if payload isn't specified, no payload is needed or optional.
- if return isn't specified, no information is returned back. (Status code still told the status of the request)
# User
## Create task
__POST /user__ \
Create user payload. \
Payload: [NewUser](#newuser-object) \
Return: [User](#user-object)
## Read all user
__GET /user__ \
Get all user \
Return: [User](#user-object)[]
## Update user
__PUT /user/:id__ \
Update user object.
Payload: [UpdateUser](#updateuser-object)
## Delete user
__DELETE /user/:id__

## Find user
__GET /user/:id__ \
Return: [User](#user-object)

## Search user
__GET /user/search?(firstname={firstname})(&)(lastname={lastname})__ \
One of its attribute need to be filled \
Return: [User](#user-object)[]

## Get team
__GET /user/:id/team__ \
Return: [Team](#team-object)

# Task
## Create task
__POST /task__ \
Create task payload.
Payload: [NewTask](#newtask-object)
Return: [Task](#task-object)
## Read all task
__GET /task__ \
Get all task.
Return: [Task](#task-object)
## Update task
__PUT /task/:id__ \
Payload: [UpdateTask](#update-task)

## Delete task
__DELETE /task/:id__ \
Delete task.
## Find task
__GET /task/:id__ \
Return: [Task](#task-object)

## Search Task
__GET /task/search?name={name}__ \
Return: [Task](#task-object)[]

## Read task owner
__GET /task/:id/team__ \
Return: [Team](#team-object)


# Team
## Create team
__POST /team__ \
Create team payload.
Payload: [NewTeam](#newtask-object)
Return: [Team](#task-object)
## Read all task
__GET /team__ \
Get all team.
Return: [Team](#team-object)
## Update team
__PUT /team/:id__ \
Payload: [PartialTask](#partial-task)

## Delete team
__DELETE /team/:id__ \
Delete task.

## Find team
__GET /team/:id__ \
Return: [Team](#team-object)

## Search team
__GET /team/search?name={name}__ \
Return: [Team](#team-object)[]

## Add user to the team
__POST /assign_user__ \
Payload: [AssignUser](#assignuser-object)

## Accept task
__POST /accept_task__ \
Payload: [AcceptTask](#accept-task)

## Read all task
__GET /team/:id/task__ \
Return: [Task](#task-object)[]

## Read all user
__GET /team/:id/user__ \
Return [User](#user-object)[]

# Addtional Idea: Role (enum)
making role as enum make 
- UX/UI Designer
- Frontend Developer
- Backend Developer
- Project Manager

# Object
## User Object
```
{
  id: ID,
  email: string,
  firstname: string,
  lastname: string,
  role: string,
  team_id: Nullable<ID>
}
```

## NewUser Object
```
{
  email: string,
  firstname: string,
  lastname: string,
  role: string,
  pwd: string
}
```
## PartialUser Object
At least one of the optional field is required.
```
{
  email?: string,
  firstname?: string,
  lastname?: string,
  role?: string,
  team_id?: Nullable<ID>
}
```
## Task Object
```
{
  id: ID,
  name: string,
  content: string,
  status: string,
  deadline: Timestamp,
  owner_team_id: Nullable<ID>
}
```

## NewTask Object
```
{
  name: string,
  content: string,
  status: string,
  deadline: Timestamp
}
```
## PartialTask Object
At least one of the optional field is required.
```
{
  name?: string,
  content?: string,
  status?: string,
  deadline?: Timestamp
}
```

## Team Object
```
{
  id: ID,
  name: String
}
```

## NewTeam Object
```
{
  name: string
}
```

## PartialTeam Object
```
{
  id?: ID,
  name?: string
}
```

# Other
## AssignUser Object
```
{
  team_id: ID,
  user_id: ID
}
```
## AcceptTask Object
```
{
  team_id: ID,
  task_id: ID
}
```


## Timestamp
Timestamp is string which formatted as 'YYYY-MM-DD HH:MM:SS' such as '2022-12-31 15:45:10'.

## ID
ID is 5-digit integer start from "10000" - "99999"