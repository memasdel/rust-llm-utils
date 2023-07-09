# suggest user to se the env var OPEN_AI_TOKEN
echo "NOTE!!! please set your token to an env var: export OPEN_AI_TOKEN=<YOUR_TOKEN_HERE>"

# then we can get the models from the API
curl https://api.openai.com/v1/models -H "Authorization: Bearer ${OPEN_AI_TOKEN}" > open_ai_models.json

