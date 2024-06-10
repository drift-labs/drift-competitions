while true; do
  yarn run settle-competition
  if [ $? -ne 1 ]; then
    break
  fi
done