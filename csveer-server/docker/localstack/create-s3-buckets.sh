#!/bin/bash

awslocal s3api create-bucket --bucket pending-csv-files

awslocal s3api put-bucket-notification-configuration --bucket pending-csv-files --notification-configuration '{
  "QueueConfigurations": [
    {
      "QueueArn": "arn:aws:sqs:us-east-1:000000000000:csv-file-digestion",
      "Events": ["s3:ObjectCreated:*"]
    }
  ]
}'

