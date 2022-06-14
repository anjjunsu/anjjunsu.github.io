---
title:  "Network - AWS (WIP)" 
excerpt: "Network in AWS"

categories:
  - AWS
tags:
  - [Cloud Computing, AWS, Network,]

toc: true
toc_sticky: true
 
date: 2022-06-14
last_modified_at: 2022-06-14

---


## VPC (Virtual Private Cloud)
VPC is a custom defined virtual networking environment in AWS.    
We have full control over VPC:  
- Selection of own IP address range
- creation of subnets
- Configuration of route tables and network gateways      
Also, we can logically isolate sections(subnets) in VPC so that we can launch multiple AWS services independantly; and define network connectivity and restrictions between subnets which can be used in multi-tier architecture. 

## CloudFront

## Route 53
Route 53 is highly available and scalable cloud DNS (Domain Name Service) => AWS in-house DNS          
DNS translates domain names to IP addresses.     

## IGW (Internet Gateway)
IGW allows communication between my own VPC and the internet.  

## NAT (Network Address Translation)
NAT allows my private subnet to connect to services outside my VPC. 
But external services cannot initiate a connection.
