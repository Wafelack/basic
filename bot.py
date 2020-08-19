import discord
import json

TOKEN = json.load(open("assets/token.json", "r"))['token']

client = discord.Client()


@client.event
async def on_ready():
    print("FerriBot is ready")

client.run(TOKEN)
