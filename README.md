# Copilot dice roll demo

This is a demo that takes deploys a signle container application to AWS ECS using AWS Copilot.
The application is a simple dice roll application that takes a input of the number of dice to roll and returns the results of the dice roll.

## Prerequisites
You will need to have the following installed on your machine:
- [AWS CLI](https://docs.aws.amazon.com/cli/latest/userguide/install-cliv2.html)
- [Docker](https://docs.docker.com/get-docker/)
- [AWS copilot](https://aws.github.io/copilot-cli/docs/getting-started/install/)

## Deploying the application
To deploy the application run the following command:
```bash
copilot init --app demo --name dice-roll --type "Load Balanced Web Service" --dockerfile "./Dockerfile"
```
This will create a new application called demo and a new service called dice-roll. The service will be a load balanced web service and will use the Dockerfile in the current directory to build the container image.

Once the application has been created you can deploy it to AWS by running the following command:
```bash
copilot deploy
```
This will deploy the application to AWS ECS and will create a new VPC, ECS cluster, ALB, and ECS service.

## Testing the application
Once the application has been deployed you can test it by running the following command:
```bash
copilot svc show
```
This will show you the URL of the application. You can then test the application by running the following command:
```bash
curl <URL>/roll/2
```
This will return the results of the dice roll.

## Cleaning up
To clean up the application run the following command:
```bash
copilot app delete
```
This will delete the application and all of the resources that were created by copilot.


