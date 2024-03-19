#!/bin/bash

awslocal sqs create-queue --queue-name csv-file-ingestion

awslocal sqs create-queue --queue-name data-dispatch
