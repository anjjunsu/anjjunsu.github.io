---
title:  "Compute Services - AWS" 
excerpt: "Compute Services in AWS"

categories:
  - AWS
tags:
  - [Lambda, Docker,]

toc: true
toc_sticky: true
 
date: 2022-06-20
last_modified_at: 2022-06-20

---



## Serverless
- Serverless means we don't need to take care about back-end thing (Abstraction)	
- But it doesn't mean that server isn't exists at all	
- FaaS (Function as a Service)	


## Lambda	
- Virtual *function*
- Serverless	
- Run and terminated *on-demand* (event-driven)	
- Think about functional programming!	
- Easy monitoring with AWS CloudWatch	
- Supports almost all programming language we need 	
- Up to 15 mins of invocation time	

## Amazon API Gateway
- Create, publish, maintain, monitor, and secure API
- Supports RESTful APIs and WebSocket APIs
- Serverless
- Expose Lambda function as HTTP API

## AWS Batch	
- Batch: Program that is assigned to the computer to run without further user interaction	
- Fully managed batch processing at *any scale*	
- AWS Batch will dynamically launch EC2 instance of Spot instance	
- AWS Batch provisions the right amount of compute/memory	
- Batch jobs are defined as Docker image and run on ECS

## Amazon Lightsail	
- Easy version AWS	

## Docker	
- Docker packages in *container* to make software deployment easier 
- Once the application is packaged, it can be run on any OS, and any environments (No compatibility issues)	
- Docker images are stored in Docker Repositories
	- Public Repository: https://hub.docker.com	
	- Private: Amazon ECR (Elastic Container Registry)!	
- Docker is *lighter* than virtual machines since it doesn't need to create virtual OS for itself, and use Docker Daemon	

## ECS (Elastic Container Service)	
- Launch Docker container on AWS	
- It is our responsibility to provision and maintain infrastructures such as EC2 	
- ECS only starting/stopping container	
- ECS can be integrated with the Application Load Balancer	

## Fargate	
- Launch Docker containers on AWS	
- Unlike ECS, it's Fargate's responsiblity to take care of infrastructures	
- Serverless service	
	
## ECR (Elastic Container Registry)	
- Private Docker container repository 	
- Store Docker container in ECR, and launch using ECS -or- Fargate	



