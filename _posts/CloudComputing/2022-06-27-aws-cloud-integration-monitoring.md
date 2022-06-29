---

 title:  "Cloud Integration and Monitoring - AWS" 
excerpt: "Intergrate and monitor services"

categories:
  - AWS
tags:
  - [Integration, Monitoring]

toc: true
toc_sticky: true
 
date: 2022-06-27
last_modified_at: 2022-06-27

---


## Amazon SQS (Simple Queue Service)	
- Connects two AWS services. So that we can *decouple* applications
- Once messages are read by consumer(poll), it will be deleted
- Consumer shares the work to read messages and scale horizontally	

## Amazon Kinesis	
- Real-time big data streaming	
- Kinesis collect, process, and analyze real-time streaming data at any scale	

## Amazon SNS
- SNS allows one application to send messages to **multiple receivers**
- "Event pulisher" only sends message to one SNS topic	
- As many "event subscribers" can recieve the message	

## Amazon MQ	
- Amazon managed Apache Active MQ	
- Instead of re-engineering traditional application running from on-premise, use open protocols, MQ

## Amazon CloudWatch Metrics
- **Metric** is a variable to monitor such as CPU utilization, network, etc. with timestamps
- Can configure CloudWatch dashboards of metrics	

## Amazon CloudWatch Alarms	
- Trigger notification for any metric	
- Alarms actions
	- Auto Scaling
	- EC2 Actions
	- SNS Notification

## Amazon CloudWatch Logs	
- *Real-time* monitoring	

## Amazon CloudWatch Events	
- Schedule: Cron
- Event Pattern: Do something if event happens
- Trigger Lambda functions, send SQS/SNS messages	

## Amazon EventBridge	
- Future of CloudWatch Events	
- Allows partner event bus

## AWS CloudTrail
- Govern, compliance, and audit AWS Account
- CloudTrail is enabled by default
- History of events/API calls made within **AWS Account**

## CloudTrail Events
- Management Events
	- Operation that are performed on resources in AWS account	
	- Can separate *Read Events* from *Write Events*
- Data Events
	- By default, data events are not logged	
	- Amazon S3 object-level activity and Lambda function execution activity	
- CloudTrail Insights Events
	- Detect unusual activity in AWS account
- Events are stored for 90 days in CloudTrail
- To preserve events more than 90 days, log them to S3, and use Athena	

## AWS X-Ray	
- Visual analysis of applications
- Good for debugging distributed services	

## Amazon CodeGuru	
- ML-powered service for *automated code reviews* and *application performance recommendations*	
- CodeGuru Reviewer: automated code reviews (development)	
- CodeGuru Profiler: recommendations about application performance during runtime (in production)

## AWS Service Health Dashboard	
- Shows all current and historical services health for all regions

## AWS Personal Health Dashboard	
- Alerts and remediation guidance when AWS is experiencing *events that may impact us*
- More personalized than Service Health Dashboard



