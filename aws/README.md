

# AWS Lambdas on Rust


Some sample AWS Lambda projects written in Rust.


---


<!--TOC-->


## Prerequisites

1. You need to complete the AWS setup steps.
2. (Optional) If you are not familiar with building/deploying Lambda functions written in Rust then you are highly encouraged to check out the Cargo Lambda project which will make the process much easier. See the [Cargo Lambda](#) section.


## AWS setup

1. You need an AWS Account.
2. You need the [AWS CLI installed](https://docs.aws.amazon.com/cli/latest/userguide/getting-started-quickstart.html).
3. Once the AWS CLI is installed, set up your local AWS configuration and credentials using `aws configure`
4. Your AWS credentials should have the necessary permissions to deploy a Lambda function.

Useful resources:-

- The recommended approach for authenticating with AWS is to use AWS IAM Identity Center - see [Configuring the AWS CLI to use AWS IAM Identity Center](https://docs.aws.amazon.com/cli/latest/userguide/cli-configure-sso.html#cli-configure-sso-configure).
- See [configure your local profile](https://docs.aws.amazon.com/cli/latest/userguide/cli-configure-sso.html#cli-configure-sso-configure) to setup the configuration required for SSO - you can discover the SSO start URL and Region via the AWS dashboard. See [Prerequisites](https://docs.aws.amazon.com/cli/latest/userguide/cli-configure-sso.html#cli-configure-sso-prereqs) section.

Once you are ready to deploy, you shall need to obtain a temporary access token. If using SSO use `aws sso login`


## Cargo Lambda

[Cargo Lambda](https://www.cargo-lambda.info/) is an extension to Cargo and provides useful tools and workflows to help you get started building Rust functions for AWS Lambda from scratch. 

[Install Cargo Lambda](https://www.cargo-lambda.info/guide/installation.html).

Some of the AWS Lambda demos use Cargo. Although it is not a prerequisite for working with Rust Lambda projects, it is a useful tool and worth investigating if you are just starting out building Lambdas using Rust. 

Pre-requisites for deploying functions to your AWS account using Cargo Lambda:-

1. You need to complete the AWS setup steps.
2. You need to have [Cargo Lambda installed](https://www.cargo-lambda.info/guide/installation.html).

### Function Types

Note that there are two types of Lambda functions that you can create with Cargo Lambda.

1. Basic functions designed to receive events specified in the payload.
	- These functions are typically implemented using just the lambda_runtime Rust crate. This is not usually what you require.	  

2. A HTTP function that is designed to be invoked from another service such as API Gateway, an Elastic Load Balancer, or function URLs. **These functions are typically what you want to be working with.**
	- These functions are typically implemented using the lambda_http Rust crate, which extends the lambda_runtime.
	- These functions require HTTP calls to be wrapped into the right JSON payloads because AWS Lambda doesn't support HTTP calls natively.

### Running functions locally

Cargo Lambda has a really cool [watch](https://www.cargo-lambda.info/commands/watch.html) function that emulates the AWS Lambda control plane API. It will also hot compile changes in your Lambda functions enabling you to do rapid local development.

You run the following in the root of a Rust workspace.

```sh
cargo lambda watch
```

### Invoking functions locally

You can invoke your function locally using the [cargo lambda invoke](https://www.cargo-lambda.info/commands/invoke.html) command. For example, if your Lambda accepts a plain JSON body then you can do.

```sh
cargo lambda invoke --data-ascii "{ \"command\": \"hi\" }"
```

If your function expects an event from an upstream AWS service such as API Gateway or an ELB then you can use the `--data-example` flag to fetch an example payload from the [aws-lambda-events](https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/lambda-events) repository, and use it as your request payload. See [example data](https://www.cargo-lambda.info/commands/invoke.html#example-data) section in the Cargo Lambda docs for more details.

```sh
cargo lambda invoke --data-example apigw-request
```

### Deploying your functions to AWS

You deploy a function using the [cargo lambda deploy](https://www.cargo-lambda.info/commands/deploy.html) command.

### Invoking remote functions deployed to AWS

In addition to using your regular dev/test toolset (such as Postman) for testing your functions, you can also use the `invoke` command to invoke your functions deployed to AWS. You do this by using the [--remote](https://www.cargo-lambda.info/commands/invoke.html#remote) flag.

```sh
cargo lambda invoke --remote --data-example apigw-request http-lambda
```




