

# README

This project demonstrates how to use some of the commands of [cargo-lambda](https://www.cargo-lambda.info/) for developing, testing and deploying a Lambda. The function itself does nothing interesting, it accepts a plain JSON body which it logs and then echos back to the caller. The project


## Create a project using Cargo Lambda

Use the [cargo lambda new](https://www.cargo-lambda.info/commands/new.html) command to create a project.

```sh
cargo lambda new simple-cargo-lambda
```


## Prerequisites

Complete both the AWS and Cargo Lambda setups described in [aws/README](../README.md).


## Build the function

```sh
cargo lambda build
```

## Running the function locally

Run the Cargo Lambda watch command to emulate the AWS Lambda control plane API.

```sh
cargo lambda watch
```


## Invoke the function locally

Since this function is just expecting a plain JSON body you can invoke it like so.

```sh
cargo lambda invoke --data-ascii "{ \"command\": \"hi\" }"
```


## Deploy the function to AWS

This step assumes:-

- You have an AWS account set up.
- You either have a set of credentials configured in your local AWS configuration that is permitted to deploy to Lambda, or you have explicitly obtained a temporary access token by using configuring single sign-on and have successfully ran `aws sso login` locally.

```sh
cargo lambda deploy
```

If you log into the AWS dashboard you should be able to see the deployed function.


## Running the function on AWS

The simplest way to test this function is using the Test tab of the Lambda console itself. Send any JSON request body and the function should log the request and echo the payload contents.

```
# request
{
  "payload": {
    "firstName": "Harry",
    "lastName": "Callahan",
    "nickName": "Dirty Harry"
  }
}

# log
START RequestId: 49dea0b0-8041-46e7-af5a-0333c0515c17 Version: $LATEST
INFO Lambda runtime invoke{requestId="49dea0b0-8041-46e7-af5a-0333c0515c17"
xrayTraceId="Root=1-673e57b6-4d3b2cce0232a5a97eb755eb;Parent=65fa8e08b986dec3;Sampled=0;Lineage=1:d556a4a8:0"}:
Payload {"firstName":"Harry","lastName":"Callahan","nickName":"Dirty Harry"}.
END RequestId: 49dea0b0-8041-46e7-af5a-0333c0515c17
REPORT RequestId: 49dea0b0-8041-46e7-af5a-0333c0515c17	Duration: 0.91 ms
Billed Duration: 1 ms	Memory Size: 128 MB	Max Memory Used: 16 MB
```

Alternatively, you could expose a function URL and send a request using something like Postman.
