---
title:  "AWS ELB & ASG" 
excerpt: "AWS"

categories:
  - AWS
tags:
  - [Cloud Computing, AWS, ELB, ASG,]

toc: true
toc_sticky: true
 
date: 2022-06-16
last_modified_at: 2022-06-16

---

## Scalability
Scalability is the property of a system to handle a growing amount of work by adding resources to the system	
### Vertical Scalability	
- Increasing the size, or power of the instance	
- Harder to scale compared to horizontal scaling

### Horizontal Scalability (= elasticity)
- Increasing the number of instances	
- Nature of cloud computing allows easy horizontal scaling

## Availability 
- Availability means the ability of a system to operate continuously without failing
- Usually, availability has a close relationshipt with horizontal scaling
- High availability increases resiliency from disaster

## Scalability and High Avaiability analogy on EC2
- Vertical Scaling: Upgrade EC2 tier
- Horizontal Scaling: ASG or ELB
- High Availability: Multi AZ ASG or ELB

## Elasticity	
Elasticity means there will be an *easy way* to scale the system, such as ASG

## Agility
Agility means how fast the system can respond/adopt changes

## ELB (Elastic Load Balancer)
- Load balancer forward incomming traffic to multiple servers to prevent overload
- Expose a single point of access to application
- Performs a regular health check of instances

### Types of ELB
- Application Load Balancer: for (HTTP / HTTPS) - Layer 7
- Network Load Balancer: (ultra-high performance, allows for TCP) - Layer 4
- Classic Load Balancer: Layer 4 & 7 (It has been a default load balancer in eary age of AWS. Not a good idea to use this)

## ASG (Auto Scaling Group)
- ASG scale out and scale in automatically depending on the workload
- Ensures minimum and maximum (just enough) resources are running
- Replaces unhealthy instances
- This saves us from billing nightmare (Don't ask me how I learned it...)

### ASG Scaling Strategies
**Manual Scaling:**	
Manually updating size of ASG

**Dynamic Scaling:**		
- Simple / Step Scaling: Using CloudWatch, set certain threasholds to scale in/out ASG
- Target Tracking Scaling: Set target for ASG (e.g. targeting to maintain ASG's CPU usage at arount 50%)
- Scheduled Scaling: Set ASG based on time (e.g. increase server resource at 7 pm on Friday)

**Predictive Scaling:**		
Use ML to predict future traffic ahead of time, and automatically provisions the right number of EC2 instances in advance

