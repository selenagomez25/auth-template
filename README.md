# auth-template
hey guys, decided to make an open source hwid lock. **this is NOT secure**, i just made it for people who still use https://github.com/vil/hwid.
the server is written in rust, and the client is written in java. **you can easily port the client to your fabric/forge mod.**
this is no more than an overly complicated pastebin auth lol, so dont think this is uncrackable and make a ton of issues pls

## setting up the server
first of all, we gotta clone the repo (install [git bash](https://git-scm.com/downloads) for this). install [rust](https://www.rust-lang.org/) aswell
```
git clone https://github.com/selenagomez25/auth-template.git
cd auth-template
cd server
cargo build --release
```
after u run those commands, extract the `auth-template.exe` in `server\target\release` and copy config.yaml and hwids.yaml
put these files in your vps using some type of [ftp client](https://filezilla-project.org/) and then run `./auth-template.exe` on ur vps using some type of [ssh client](https://termius.com/).

## recommended vps
here are some commonly used vps providers. the cheapest option, which usually costs around $2-3, should suffice, but you can opt for more if needed.
* [vultr](https://www.vultr.com/ )
* [cloudfanatic](https://cloudfanatic.net/)
* [linode](https://cloud.linode.com/)

## what makes it better than vil's hwid lock?
my reasons are pretty simple, though vil's hwid is probably made for simplicity and stuff idk whatever
1. hwids arent on pastebin
2. webhooks are sent serverside
3. it automatically fetches uuid using minecrafts api so if u did want to use it on forge, u dont need to change that value
5. code is easily changable, allowing u to add/remove whatever u want!

## what makes it worse than vil's hwid lock?
1. server is coded in rust...
2. it requires a vps! this costs around $2/m
3. client depends on gson, and okhttp in order to use the client
4. and some more stuff that im lazy to list
   
## license
this project is licensed under the gnu general public license (gpl) license - see the [LICENSE](https://github.com/selenagomez25/auth-template/blob/master/LICENSE) file for details.

## contributing
contributions are welcome! feel free to submit a pull request whenever.
