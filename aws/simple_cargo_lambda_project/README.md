

# README


This project demonstrates how to create and manage a simple project using [cargo-lambda](https://www.cargo-lambda.info/).

The function itself does nothing interesting, it accepts a plain JSON body which it logs and then echos back to the caller.


## Prerequisites

- The build/packing script `rust_lambda_helpers.zsh` is built for the zsh shell. If you are using bash or are on windows you may need to play with the script for your environment.
- You need Docker installed and running when you use the build script. If using Mac you can use brew `brew install --cask docker`


## Build the function

In the root of the project directory:-

1. Source the helper script `source rust_lambda_helpers.zsh`
2. Run the build function `al2build`. Note that the Docker daemon must be running as this script will spin up a container within which to run the Cargo build.
3. Run the package function `zipRustLambda` which will create a deployable `lambda.zip`

### Configure the Lambda function in AWS

In your Amazon account dashboard, go to Lambda and create a function as follows:-

- Name: `simple_lambda_local_docker_build`
- Runtime: `Amazon Linux xxx`
- Architecture: `arm64`
- Execution role: You can just go with the default as this function requires no additional permissions other than the defaults.

Upload the zip file:-

Select the Lambda you created and in the **Code** tab choose **Upload from** -> **.zip** file. Choose the local .zip file and **Save**.


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

