---
title:  "Deploying and managing services - AWS" 
excerpt: "Deployment and management in AWS"

categories:
  - AWS
tags:
  - [CloudFormation,]

toc: true
toc_sticky: true
 
date: 2022-06-20
last_modified_at: 2022-06-20

---


## CloudFormation
- Declarative way of outlining AWS infrastructure by code
- Infrastructure as code	
- Automatically generates resources diagram (Stack Designer)
- Increases re-usability	

## AWS Cloud Development Kit (CDK)	
- Define cloud infrastructure using a popular programming languages (JavaScript / Python / Java / .NET)	
- The code is compiled to CloudFormation template	
- Can deploy infrastructure and application runtime code together	

## AWS Elastic Beanstalk 	
- For developer. Deploying an application on AWS	
- Centralize all the component's view 	
- PaaS (Platform as a Service)	
- Can configure instances handled by Beanstalk	
- Capacity provisioning, load balancing and auto-scaling	
- Application health-monitoring and responsiveness	
- Only application code is the responsibility of the developer	
- Simply upload code and Elastic Beanstalk automatically handles the deployment	

## AWS CodeDeploy	
- Hybrid service (Works with EC2 and/or On-Premises Servers)	
- Server / Instances must be provisioned and configured ahead of time with the CodeDeploy Agent

## AWS CodeCommit	
- AWS in-house Git Version Control	
- Collaboration!	

## AWS CodeBuild	
- Code building: compiling source code, run tests, and produces packages that ready to be deployed	

## AWS CodePipeline	
- Orchestrate the steps to have the code automatically pushed to production	
- Code -> Build -> Test -> Provision -> Deploy	

## AWS CodeArtifact	
- Software packages depend on each other to be built (dependencies)	
- Artifact Management: storing and retrieving theses dependencies	
- AWS CodeArtifact is AWS-version of Artifact Management	
- Developer and CodeBuild can retrieve dependencies straight from CodeArtifact	

## AWS CodeStar	
- Unified dashboard to manage software development activities in one place (CodeCommit, CodePipeline, CodeBuild, CodeDeploy, Elastic Beanstalk, EC2, etc...)	

## AWS Cloud9


