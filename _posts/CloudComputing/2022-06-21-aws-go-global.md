---

 title:  "Go Global - AWS" 
excerpt: "Make Global Infra in AWS"

categories:
  - AWS
tags:
  - [Global,]

toc: true
toc_sticky: true
 
date: 2022-06-21
last_modified_at: 2022-06-21

---



## Route 53
- Route 53 is a AWS managed DNS service 
### Routing Policeis
- Simple Routing Policy 
    - No health check   
- Weighted Routing Policy
    - Route incomming traffic based on each EC2's pre-defined weight
- Latency Routing Policy    
    - Route incomming traffic to region with lowest latency
- Failover Routing Policy   
    - If primary instance fails, re-direct to failover instance     

## CloudFront
- Cache the content at the *edge location* to improve **read performance** (content is served at the edge location)
- DDoS protection, and can be integrated with Shield, and AWS Web Application Firewall
- Origins
	- S3 bucket
	- HTTP: ALB, EC2, S3 website, any HTTP backend
- CloudFront is good for static content since it will be cached for a TTL (Time To Live). If content needed to be dynamic, use S3 Cross Region Replication	

## S3 Transfer Acceleration	
- Increase transfer spped by transferring file to AWS edge location. 
- Once the file transferred to edge location, it will be  transferred to S3 bucket through the fast private AWS network

## AWS Global Accelerator	
- Leverage the AWS internal network to optimize the route (instead of using public network, use AWS internal network)	
- Increases global application *availability* and *performance*	

## AWS Outposts
- Enables the efficient **Hybrid Cloud**. AWS Outposts actes as *server rack* that allows same AWS infrastructure, services to on-premises application just as in the cloud
- AWS will setup and manage *Outposts Racks*
- It is user's responsibility to secure physical server security	

## AWS WaveLength	
- **WaveLength Zones** are infrastructure deployments embedded within the 5G network providers' data center	
- Operate ASW services on the edge of 5G networks
- Ultra low latency through 5G network

## AWS Local Zones	
- Placing AWS services *closer* to end-user to run latency-sensitive applications	
- Act like extension of an AWS Region	


