#!/bin/bash
echo "sleeping for 5 seconds before initialization"
sleep 5
echo mongo_setup.sh time now: `date +"%T" `
mongosh --host mongo:27017 <<EOF
  var cfg = {
    "_id": "rs0",
    "version": 1,
    "members": [
      {
        "_id": 0,
        "host": "mongo:27017",
        "priority": 1
      },
    ]
  };
  rs.initiate(cfg);
EOF