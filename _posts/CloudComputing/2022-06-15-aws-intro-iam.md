---
title:  "AWS Intro & IAM" 
excerpt: "AWS"

categories:
  - AWS
tags:
  - [Cloud Computing, AWS, IAM]

toc: true
toc_sticky: true
 
date: 2022-06-15
last_modified_at: 2022-06-15

---

## Introduction

### What is Cloud Computing?
- On-demand delivery of compute power, database storage, applications, and other IT resources       
- Pay as much as we use     
- Can provision the size and/or type of computing resources     
- No infrastructure on our end is required, and we can access any resources offered     

### Characteristics of Cloud Computing
1. On-demand self service
2. Measured service
3. Good elasticity and scalability
4. Multiple users can share same resource with corresponding security and privilege    

### Advantages of Cloud Computing
- Don't need to invest assets in infrastructure; this reduced total cost of ownership and initial cost
- Price is cheaper than owning private resources since AWS is more efficient due to large scale
- Elasticity, scalability: Don't need to estimate capacity, and scale based on actual measured usage
- Increase speed and agility
- No maintenance cost
- Easy global, multi-region deployment
- High-availability: fault, or disaster tolerance
- Agile

### AWS Regions & Acailability Zone
Region: Cluster of data centers     
Each region has multiple availability zones     
Each AZ is discrete data centers, and they are separated from each other to isolate impact of disasters

## IAM

### Idenity and Access Management (IAM)
- Don't ever use root account!    
- Users: People within organization. Can be grouped   
- Groups: Only contain users, not other group     
- Permissions for users or groups can be assigned by `policies` (JSON document format)        
- *Note:* Always assign the **least privilege** (don't give more power than user needs)   

### IAM Roles
Can assign **permissions** to AWS services with **IAM Roles**       

### IAM Security Tools   
- IAM Credentials Report        
: A report that lists all account's users and the status
- IAM Access Advisor    
: Shows the service permissions granted to a user, and access log   

