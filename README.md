# auth-template
hey guys, decided to make an open source hwid lock. **this is NOT secure**, i just made it for people who still use https://github.com/vil/hwid.
the server is written in rust, and the client is written in java. **you can easily port the client to your fabric/forge mod.**
this is no more than an overly complicated pastebin auth lol, so dont think this is uncrackable and make a ton of issues pls

## installation
first of all, we gotta clone the repo (install [git bash](https://git-scm.com/downloads) for this). install [rust](https://www.rust-lang.org/) aswell
```
git clone https://github.com/selenagomez25/auth-template.git
cd auth-template
cd server
cargo build --release
```
after u run those commands, extract the `auth-template.exe` in `server\target\release` and copy config.yaml and hwids.yaml
put these files in your vps using some type of [ftp client](https://filezilla-project.org/) and then run `./auth-template.exe` on ur vps using some type of [ssh client](https://termius.com/).
u will then need to replace the shit in the client so
```java
    private static final String API_KEY = "your_secret_api_key_here";
    private static final String SERVER_ADDRESS = "http://127.0.0.1:3030";
```
in main.java to ur shit, afterwards ur done!

## recommended vps
for those asking on which vps to get, these are some commonly used ones.
https://www.vultr.com/ 
https://cloudfanatic.net/
https://cloud.linode.com/
these three are all pretty good. you only need the cheapest option which is usually 2-3$, but if u want u could get more

## what makes it better than vil's hwid lock?
my reasons are pretty simple, though vil's hwid is probably made for simplicity and stuff idk whatever
1. hwids arent on pastebin
2. webhooks are sent serverside
3. it automatically fetches uuid using minecrafts api so if u did want to use it on forge, u dont need to change that value
5. code is easily changable, allowing u to add/remove whatever u want!

## what makes it worse than vil's hwid lock?
1. server is coded in rust...
2. it requires a vps! this costs around $2/m
3. client depends on some things like lombok, gson, and okhttp in order to use the client
4. and some more stuff that im lazy to list
