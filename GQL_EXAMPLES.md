## Mutations

```
mutation {
  insertUser(
    userData: {
      username: "John"
      hashedPassword: "123"
      email: "apple@apple.com"
    }
  ) {
    id,
    userId
  }
}

```

```
{
  getAllTestObjects {
    id, val
  }
}

mutation AddUser($userData: UserInputData) {
  addUser(userData:$userData) {
    
  }
}
```