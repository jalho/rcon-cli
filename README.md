# `rcon-cli`

Example of usage as of `1494d337c`:

```
$ rcon-cli --password "foobar" --command "playerlist"
Received message: {
  "Message": "[]",
  "Identifier": 1,
  "Type": "Generic",
  "Stacktrace": ""
}
```

## Command reference

Some RCON commands issuable like so:

```
$ rcon-cli -p "Your_Rcon_Password" -c "server.description \\\"Weekly wiped low pop vanilla server.\nSchedule: Map wipes every Friday at 14 UTC.\\\""
Received message: {
  "Message": "server.hostname: \"'Reindeerland Vanilla'\"",
  "Identifier": 1,
  "Type": "Generic",
  "Stacktrace": ""
}
```

| command                                                                                                        | description                                                                                                                                                                                                                                          | tested (version)              |
| -------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------- |
| `env.time`                                                                                                     | Get the current game time.                                                                                                                                                                                                                           | 2024-03-09 (commit `30eb6f9`) |
| `global.ban 76561198135242017`                                                                                 | Ban a player from the server. This also updates `bans.cfg` on disk.                                                                                                                                                                                  | 2024-03-09 (commit `30eb6f9`) |
| `global.moderatorid 76561198135242017`                                                                         | Set a Steam user as a moderator of the server. This also updates `users.cfg` on disk.                                                                                                                                                                | 2024-03-09 (commit `30eb6f9`) |
| `global.ownerid 76561198135242017`                                                                             | Set a Steam user as an owner of the server. This also updates `users.cfg` on disk.                                                                                                                                                                   | 2024-03-09 (commit `30eb6f9`) |
| `global.playerlist`                                                                                            | List players currently in game (SteamID, IP address, health, display name etc.)                                                                                                                                                                      | 2024-03-09 (commit `30eb6f9`) |
| `global.playerlistpos`                                                                                         | List positions of players currently in game.                                                                                                                                                                                                         | 2024-03-09 (commit `30eb6f9`) |
| `global.removemoderator 76561198135242017`                                                                     | Unset a Steam user from being a moderator of the server. This also updates `users.cfg` on disk.                                                                                                                                                      | 2024-03-09 (commit `30eb6f9`) |
| `global.removeowner 76561198135242017`                                                                         | Unset a Steam user from being an owner of the server. This also updates `users.cfg` on disk.                                                                                                                                                         | 2024-03-09 (commit `30eb6f9`) |
| `global.say Hello world!`                                                                                      | Send a global chat message in-game as "SERVER".                                                                                                                                                                                                      | 2024-03-09 (commit `30eb6f9`) |
| `global.unban 76561198135242017`                                                                               | Remove a player ban. This also updates `bans.cfg` on disk.                                                                                                                                                                                           | 2024-03-09 (commit `30eb6f9`) |
| `server.description \\\"Weekly wiped low pop vanilla server.\nSchedule: Map wipes every Friday at 14 UTC.\\\"` | Set server description visible to players in the in-game menu.                                                                                                                                                                                       | 2024-03-09 (commit `30eb6f9`) |
| `server.description`                                                                                           | Get the current server description visible to players in the in-game menu.                                                                                                                                                                           | 2024-03-09 (commit `30eb6f9`) |
| `server.hostname \\\"Reindeerland Vanilla\\\"`                                                                 | Set server name listed in the in-game menu etc.                                                                                                                                                                                                      | 2024-03-09 (commit `30eb6f9`) |
| `server.hostname`                                                                                              | Get the current server name listed in the in-game menu etc.                                                                                                                                                                                          | 2024-03-09 (commit `30eb6f9`) |
| `server.maxplayers 100`                                                                                        | Set max number of players allowed on the server.                                                                                                                                                                                                     | 2024-03-09 (commit `30eb6f9`) |
| `server.maxplayers`                                                                                            | Get the current max number of players allowed on the server.                                                                                                                                                                                         | 2024-03-09 (commit `30eb6f9`) |
| `server.seed`                                                                                                  | Get the current game world seed.                                                                                                                                                                                                                     | 2024-03-09 (commit `30eb6f9`) |
| `server.tags vanilla,weekly`                                                                                   | Set server tags.                                                                                                                                                                                                                                     | 2024-03-09 (commit `30eb6f9`) |
| `server.tags`                                                                                                  | Get current tags of the server.                                                                                                                                                                                                                      | 2024-03-09 (commit `30eb6f9`) |
| `server.worldsize`                                                                                             | Get the current game world size.                                                                                                                                                                                                                     | 2024-03-09 (commit `30eb6f9`) |
| `world.rendermap`                                                                                              | Render the current game world map writing it on disk as a .PNG file.<br>This tends to kick players out of the server if issued on the server side.<br>The command can also be issued client side by any unprivileged user. That should be preferred. | 2024-03-09 (commit `30eb6f9`) |
