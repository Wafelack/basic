import discord
import json
from play import play
import urllib.request
import asyncio

TOKEN = json.load(open("assets/token.json", "r"))['token']

client = discord.Client()
PREFIX = '!'
WAFEID = 723862906755743804


async def show_bitcoin(chan):
    while True:
        VALUES = json.loads(urllib.request.urlopen('https://blockchain.info/ticker').read().decode())
        buy_usd = VALUES['USD']['buy']
        sell_usd = VALUES['USD']['sell']

        buy_eur = VALUES['EUR']['buy']
        sell_eur = VALUES['EUR']['sell']

        buy_rub = VALUES['RUB']['buy']
        sell_rub = VALUES['RUB']['sell']

        embed = discord.Embed(title="BTC Value:",
                              description=f"**USD :**\n\tSell : ${sell_usd}\n\tBuy: ${buy_usd}\n\n**EUR :**\n\tSell : {sell_eur}€\n\tBuy : {buy_eur}€\n\n**RUB :**\n\tBuy : {buy_rub} Pуб\n\tSell : {sell_rub} Pуб",
                              color=0xffbb00)
        embed.set_footer(text="Data picked on https://blockchain.info/ticker")

        await chan.send(embed=embed)
        await asyncio.sleep(1800)


@client.event
async def on_ready():
    print("FerriBot is ready")
    BOTS = client.get_channel(743853002523148349)
    client.loop.create_task(
        show_bitcoin(BOTS)
    )


@client.event
async def on_message(message):
    if message.content.startswith(PREFIX + 'get_crate'):
        splited = message.content.split(' ', 2)
        if len(splited) != 2:
            await message.channel.send(f"Usage : {PREFIX}get_crate <crate>")
            return
        embed = discord.Embed(
            title=f'{splited[1]}',
            description=f'docs.rs : https://docs.rs/{splited[1]}\ncrates.io : https://crates.io/crates/{splited[1]}',
            color=00)
        await message.channel.send(f"<@{message.author.id}>\n", embed=embed)
    if message.content.startswith(PREFIX + 'playground'):
        splited = message.content.split('\n')
        stderr, stdout = play(splited)
        if stdout == "":
            await message.channel.send(f"Standard error : ```" + stderr.replace('`', '\'') + "```\n```No standard "
                                                                                             "output !```")
        else:
            await message.channel.send(
                f"Standard error : ```" + stderr.replace('`', '\'') + f"```\nStandard output : ```{stdout}```")
        return
    if message.content == PREFIX + "make_me_rustacean":
        await message.author.add_roles(message.author.guild.get_role(743864011334090762))
        await message.channel.send("Le rôle vous a été donné avec succès !")

    if message.content == PREFIX + "help":
        embed = discord.Embed(title="Rustacean",
                              description="**!make_me_rustacean** : Gives you the role Rustacean\n**!get_crate <crate_name>** : Gives you the link of a crate\n**!playground \`\`\`rs\n\t<code>\`\`\`** : Runs the code on the rust playground")
        embed.set_author(name="Rustacean",
                         icon_url="https://cdn.discordapp.com/attachments/727885557430222849/754727660432785428/rustacean-flat-noshadow.png")
        embed.set_footer(text="!help • Copyleft Wafelack • Rustacean")
        await message.channel.send(embed=embed)


try:
    client.run(TOKEN)
except KeyboardInterrupt:
    exit()
