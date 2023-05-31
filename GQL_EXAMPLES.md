## Mutations

```
mutation {
  addUser(userData: {
    username:"John", 
    hashedPassword:"123", 
    email:"apple@apple.com"
  })
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