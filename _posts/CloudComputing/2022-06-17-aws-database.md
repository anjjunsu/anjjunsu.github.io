---
title:  "Database - AWS" 
excerpt: "Database in AWS"

categories:
  - AWS
tags:
  - [Database,]

toc: true
toc_sticky: true
 
date: 2022-06-14
last_modified_at: 2022-06-14

---

## Relational Databases	
- Data-table has relationships with other data-tables		
- Can use SQL language to do queries/lookup

## NoSQL Database	
- Benefits:
	- Flexible to evolve data model
	- Can optimize for a specific data model
	- Can optimize types for the data model		
	- Designed to scale-out by using distributed clusters

## RDS (Relational Database Service)	
- SQL DB
- RDS is managed by AWS. It's better to use RDS than deploying DB on EC2 (right...?)

## Amazon Aurora	
- AWS made -and- customized RDS		
- Supports both *PostgreSQL* and *MySQL*
- Since Aurora is optimized for AWS, it's faster than other RDS
- Aurora storage grows automatically	
- Aurora is more expensive than RDS
- NOT IN FREE TIER (...)

## RDS Deployments	

### Read Replicas
- Scale the *read workload* of DB
- Can scale up to 5 Read Replicas	
- But only written on the main DB	

### Multi-AZ
- In case of AZ outage, *Failover*
- Data is only read/written to the main database
- Can only have 1 other AZ as failover

### Multi-Region		
- Disaster recovery in case of region issue
- Improves performance in other deployed regions
- Only written on the main DB

## ElasicCache
- Get managed by Redis -or- Memcached	
- Helps reduce load off databases for read intensive workload (as what cache usually do...)	

## DynamoDB
- NoSQL database	
	- Key/Value database
- Scale to intense workloads, *serverless* database	
- Good performance, very low latency	
### DynamoDB Accelerator - DAX	
- Fully managed in-memory cache for DynamoDB	
- 10x performance improvement	
### DynamoDB - Global Tables	
- Make DynamoDB table accessible with *low latency* in multiple-regions	
- **Active-Active** replication. Can read/write to any AWS Region)	

## Redshift
- Based on PostgreSQL	
- *OLAP - Onlie Analytical Processing (analytics and data warehousing)*
- Load data once a hour, not every second
- **Columnar** storage of data	
- Supports SQL interface for queries	

## Amazon EMR (Elastic MapReduce)	
- EMR helps creating *Hadoop clusters (Big Data)* 
- Good for data processing, ML, web indexing, big data...	

## Amazon Athena	
- *Serverless* service to analyze data stored in Amazon S3	
- Supports SQL language

## Amazon QuickSight	
- *Serverless, ML-powered* business intelligence service to create interactive dasahboards	
- Allows business analytics, business insights 	
- Integrated with RDS, Aurora, Athena, Redshift, S3, ....

## Document DB	
- Like Aurora is *AWS-made version* of PostgreSQL and MySQL, DocumentDB is AWS version of MongoDB	
- NoSQL	database	
- MongoDB is used to store, query, and index JSON data	
- Fully managed, highly available with replication across 3 AZ	
- Automatically scales

## Amazon Neptue	
- **Graph** database	
- Fully managed	by AWS	
- Very usefull in social network services

## Amazon QLDB (Quantum Ledger Database)	
- Ledger is accounting term that is book containing accounts in which the classified and summarized information of financial transactions	
- Fully managed, serverless, High available, and can be replicated across 3 AZ	
- Used to review history of all the changes made to our application data	
- **Immutable**	
- *No decentralization component*, not like Amazon Managed Blockchain	

## Amazon Managed Blockchain	
- Blockchain service without central authority	
- Compatible with the Ethereum and Hyperledger Fabric	

## AWS Glue	
- ETL service (Extract, Transform, and Load)	
- Prepare and transform data for analytics	
- *Serverless*	
- It extracts data from S3 or RDS, glue it, and load into Redshift	

## DMS (Data Migration Service)	
- Migrates on-premise data to AWS	
- Source database remains available during migration	
- Quick, secure, resilient, self-healing
- Supports data migration from same -and- differenct data source type

