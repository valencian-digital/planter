bash ./mongo_setup.sh
echo "sleeping for 5 seconds before seeding data"
mongoimport --collection users --file users.json --jsonArray --uri $MONGO_URI
mongoimport --collection posts --file posts.json --jsonArray --uri $MONGO_URI