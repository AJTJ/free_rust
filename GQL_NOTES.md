# Write your query or mutation here
mutation {
  insertUser(
    userInput: {
      username: "memes"
      password: "memes"
      email: "memes@memes.com"
    }
  ) {
    username
  }
}
