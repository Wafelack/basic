import discord
import json
from play import play

TOKEN = json.load(open("assets/token.json", "r"))['token']

client = discord.Client()
PREFIX = '!'
WAFEID = 723862906755743804


@client.event
async def on_ready():
    print("FerriBot is ready")


@client.event
async def on_message(message):
    if message.content.startswith(PREFIX + 'get_crate'):
        splited = message.content.split(' ', 2)
        if len(splited) != 2:
            await message.channel.send(f"Usage : {PREFIX}get_crate <crate>")
            return
        embed = discord.Embed(
            title=f'{splited[1]}', description=f'docs.rs : https://docs.rs/{splited[1]}\ncrates.io : https://crates.io/crates/{splited[1]}', color=00)
        await message.channel.send(f"<@{message.author.id}>\n", embed=embed)
    if message.content.startswith(PREFIX + 'test_file'):
        splited = message.content.split('\n')
        play(splited)
        return

try:
    client.run(TOKEN)
except KeyboardInterrupt:
    exit()
