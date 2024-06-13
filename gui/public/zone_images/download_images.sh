cat zones.json | jq '.[].id' | while read id; do
  wget -O $id.png "https://wow.zamimg.com/images/wow/classic/maps/enus/zoom/$id.jpg"
done
