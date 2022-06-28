# Table of Contents
- [Table of Contents](#table-of-contents)
- [User](#user)
  - [Create task](#create-task)
  - [Read all user](#read-all-user)
  - [Update user](#update-user)
  - [Delete user](#delete-user)
  - [Find user](#find-user)
- [Task](#task)
  - [Create task](#create-task-1)
  - [Read all task](#read-all-task)
  - [Update task](#update-task)
  - [Delete task](#delete-task)
  - [Find task](#find-task)
- [Addtional Idea: Role (enum)](#addtional-idea-role-enum)
- [Timestamp](#timestamp)
- [ID](#id)

# User
## Create task
__POST /user__ \
Create user payload.
```
{
  email: string,
  firstname: string,
  lastname: string,
  role: string
}
```
This method, if successful, returns same object that is sent with id of the user.
```
{
  id: ID,
  email: string,
  firstname: string,
  lastname: string,
  role: string
}
```
## Read all user
__GET /user__ \
Get all task.
## Update user
__PUT /user/:id__ \
Update user object. At least one of the optional field is required.
```
{
  email?: string,
  firstname?: string,
  lastname?: string,
  role?: string
}
```
This method, if successful, returns with status code 200
## Delete user
__DELETE /user/:id__ \
return the user before delete
```
{
  id: ID,
  email: string,
  firstname: string,
  lastname: string,
  role: string
}
```

## Find user
__GET /user/:id__
```
{
  id: ID,
  email: string,
  firstname: string,
  lastname: string,
  role: string
}
```

# Task
## Create task
__POST /task__ \
Create task payload.
```
{
  name: string,
  content: string,
  status: string,
  deadline: Timestamp
}
```
This method, if successful, returns same object that is sent with id of the task.
```
{
  id: ID,
  name: string,
  content: string,
  status: string,
  deadline: Timestamp
}
```
## Read all task
__GET /task__ \
Get all task.
## Update task
__PUT /task/:id__ \
Update task. At least one of the optional field is required.
```
{
  name?: string,
  content?: string,
  status?: string,
  deadline?: Timestamp
}
```
This method, if successful, returns with status code 200

## Delete task
__DELETE /task/:id__ \
Delete task. \
return the task before delete
```
{
  name?: string,
  content?: string,
  status?: string,
  deadline?: Timestamp
}
```

## Find task
__GET /task/:id__ \
return 404 if not found.
```
{
  id: ID,
  name: string,
  content: string,
  status: string,
  deadline: Timestamp
}
```

# Addtional Idea: Role (enum)
making role as enum make 
- UX/UI Designer
- Frontend Developer
- Backend Developer
- Project Manager

# Timestamp
Timestamp is string which formatted as 'YYYY-MM-DD HH:MM:SS' such as '2022-12-31 15:45:10'.

# ID
ID is 5-digit number string start from "10000" - "99999"