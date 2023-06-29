to build
`docker build -t theloniousjanke/free_rust .`
to run first time
`docker run --name free_rust -dp 8081:8081 theloniousjanke/free_rust`
to run other times
`docker run -dp 8081:8081 theloniousjanke/free_rust`